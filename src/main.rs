mod app;
mod cli;
mod models;
mod network;
mod storage;
mod ui;

use app::{ActiveTab, App, InputMode};
use cli::{Cli, Commands};
use models::FeedSource;
use storage::Storage;

use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Add { url, title, category } => {
                let storage = Storage::new();
                let feed_title = title.unwrap_or_else(|| url.clone());
                let feed = FeedSource::new(&feed_title, &url, &category);
                storage.add_feed(&feed)?;
                println!("✅ Berhasil menambahkan RSS Feed: '{}' ({})", feed_title, url);
                return Ok(());
            }
            Commands::List => {
                let storage = Storage::new();
                let feeds = storage.get_feeds()?;
                println!("📡 DAFTAR CHANNEL RSS RUBAH ({} channel):\n", feeds.len());
                for (idx, feed) in feeds.iter().enumerate() {
                    println!("  {:2}. [{}] {} - {}", idx + 1, feed.category, feed.title, feed.url);
                }
                return Ok(());
            }
        }
    }

    // Initialize TUI Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Set panic hook to ensure terminal is restored on crash
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original_hook(panic_info);
    }));

    let mut app = App::new();

    // Spawn initial RSS fetch in background
    app.refresh_all_feeds().await;

    let res = run_app(&mut terminal, &mut app).await;

    // Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Aplikasi Rubah mengalami error: {:?}", err);
    }

    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != event::KeyEventKind::Press {
                    continue;
                }

                // Handling dialog Add Feed Input Modes
                match app.input_mode {
                    InputMode::AddFeedTitle => match key.code {
                        KeyCode::Enter | KeyCode::Tab => {
                            app.input_mode = InputMode::AddFeedUrl;
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Char(c) => {
                            app.new_feed_title.push(c);
                        }
                        KeyCode::Backspace => {
                            app.new_feed_title.pop();
                        }
                        _ => {}
                    },
                    InputMode::AddFeedUrl => match key.code {
                        KeyCode::Enter | KeyCode::Tab => {
                            app.input_mode = InputMode::AddFeedCategory;
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Char(c) => {
                            app.new_feed_url.push(c);
                        }
                        KeyCode::Backspace => {
                            app.new_feed_url.pop();
                        }
                        _ => {}
                    },
                    InputMode::AddFeedCategory => match key.code {
                        KeyCode::Enter => {
                            app.submit_new_feed();
                        }
                        KeyCode::Tab => {
                            app.input_mode = InputMode::AddFeedTitle;
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Char(c) => {
                            app.new_feed_category.push(c);
                        }
                        KeyCode::Backspace => {
                            app.new_feed_category.pop();
                        }
                        _ => {}
                    },
                    InputMode::Search => match key.code {
                        KeyCode::Enter | KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Char(c) => {
                            app.search_query.push(c);
                            app.selected_article_idx = 0;
                        }
                        KeyCode::Backspace => {
                            app.search_query.pop();
                            app.selected_article_idx = 0;
                        }
                        _ => {}
                    },
                    InputMode::Normal => {
                        if app.show_help {
                            if key.code == KeyCode::Esc || key.code == KeyCode::Char('?') || key.code == KeyCode::Char('q') {
                                app.show_help = false;
                            }
                            continue;
                        }

                        match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Char('?') => app.show_help = true,
                            KeyCode::Tab => app.next_pane(),
                            KeyCode::BackTab => app.prev_pane(),

                            KeyCode::Char('j') | KeyCode::Down => app.next_item(),
                            KeyCode::Char('k') | KeyCode::Up => app.prev_item(),

                            KeyCode::Char('d') if app.active_pane == app::ActivePane::Reader => app.scroll_reader_down(),
                            KeyCode::Char('u') if app.active_pane == app::ActivePane::Reader => app.scroll_reader_up(),
                            KeyCode::PageDown => app.scroll_reader_down(),
                            KeyCode::PageUp => app.scroll_reader_up(),

                            KeyCode::Char('1') => {
                                app.active_tab = ActiveTab::AllFeeds;
                                app.selected_article_idx = 0;
                            }
                            KeyCode::Char('2') => {
                                app.active_tab = ActiveTab::Bookmarks;
                                app.selected_article_idx = 0;
                            }

                            KeyCode::Char('r') => {
                                app.refresh_all_feeds().await;
                            }
                            KeyCode::Char('b') => {
                                app.toggle_current_bookmark();
                            }
                            KeyCode::Char('o') => {
                                app.mark_current_read();
                                app.open_current_in_browser();
                            }
                            KeyCode::Enter | KeyCode::Char(' ') => {
                                app.mark_current_read();
                                match app.active_pane {
                                    app::ActivePane::Feeds => app.active_pane = app::ActivePane::Articles,
                                    app::ActivePane::Articles => app.active_pane = app::ActivePane::Reader,
                                    app::ActivePane::Reader => app.active_pane = app::ActivePane::Articles,
                                }
                            }
                            KeyCode::Char('a') => {
                                app.input_mode = InputMode::AddFeedTitle;
                            }
                            KeyCode::Char('D') => {
                                app.delete_selected_feed();
                            }
                            KeyCode::Char('/') => {
                                app.input_mode = InputMode::Search;
                            }
                            KeyCode::Esc => {
                                if app.active_pane == app::ActivePane::Reader {
                                    app.active_pane = app::ActivePane::Articles;
                                } else {
                                    app.search_query.clear();
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}
