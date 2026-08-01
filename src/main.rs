mod app;
mod cli;
mod image_render;
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
                println!("[OK] Berhasil menambahkan RSS Feed: '{}' ({})", feed_title, url);
                return Ok(());
            }
            Commands::List => {
                let storage = Storage::new();
                let feeds = storage.get_feeds()?;
                println!("[LIST] DAFTAR CHANNEL RSS RUBAH ({} channel):\n", feeds.len());
                for (idx, feed) in feeds.iter().enumerate() {
                    println!("  {:2}. [{}] {} - {}", idx + 1, feed.category, feed.title, feed.url);
                }
                return Ok(());
            }
            Commands::Uninstall => {
                println!("[REMOVE] Menghapus aplikasi Rubah dan data konfigurasi...");
                App::perform_uninstall()?;
                println!("[OK] Aplikasi Rubah berhasil di-uninstall dari sistem Anda.");
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
    let mut last_latency_check = std::time::Instant::now();
    let mut last_marquee_time = std::time::Instant::now();
    app.update_latency().await;

    loop {
        if last_marquee_time.elapsed() >= Duration::from_millis(150) {
            app.marquee_tick = app.marquee_tick.wrapping_add(1);
            last_marquee_time = std::time::Instant::now();
        }

        terminal.draw(|f| ui::draw(f, app))?;

        if last_latency_check.elapsed() > Duration::from_secs(5) {
            app.update_latency().await;
            last_latency_check = std::time::Instant::now();
        }

        if event::poll(Duration::from_millis(16))? {
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
                        KeyCode::Tab | KeyCode::Up | KeyCode::Down => {
                            app.cycle_category_suggestion();
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
                    InputMode::MoveFeedCategory => match key.code {
                        KeyCode::Enter => {
                            app.submit_move_feed_category();
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Char(c) => {
                            app.move_feed_category_input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.move_feed_category_input.pop();
                        }
                        _ => {}
                    },
                    InputMode::DeleteCategoryConfirm => match key.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                            app.confirm_delete_category();
                        }
                        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.target_category_to_delete = None;
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
                        if app.show_uninstall_confirm {
                            match key.code {
                                KeyCode::Char('y') | KeyCode::Char('Y') => {
                                    disable_raw_mode()?;
                                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                                    terminal.show_cursor()?;
                                    println!("\n--> 🦊 Rubah [Ruang Baca Harian] Uninstaller");
                                    println!("--> Menghapus binary executable 'baca' & shortcuts...");
                                    println!("--> Menghapus data konfigurasi & database...");
                                    println!("--> Menghapus cache & PATH environment...");
                                    let _ = App::perform_uninstall();
                                    println!("--> Uninstall berhasil selesai.");
                                    println!("Terima kasih telah menggunakan Rubah [Ruang Baca Harian].");
                                    println!("Sampai jumpa kembali! 🦊\n");
                                    std::process::exit(0);
                                }
                                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                                    app.show_uninstall_confirm = false;
                                }
                                _ => {}
                            }
                            continue;
                        }

                        if app.show_help {
                            if key.code == KeyCode::Esc || key.code == KeyCode::Char('?') || key.code == KeyCode::Char('q') {
                                app.show_help = false;
                            }
                            continue;
                        }

                        match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Char('?') => app.show_help = true,
                            KeyCode::Char('U') => app.show_uninstall_confirm = true,
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
                            KeyCode::Char('f') | KeyCode::Char('F') => {
                                app.toggle_fullscreen_reader().await;
                            }
                            KeyCode::Char('i') | KeyCode::Char('I') => {
                                app.toggle_image_display();
                            }
                            KeyCode::Char('b') => {
                                app.toggle_current_bookmark();
                            }
                            KeyCode::Char('m') => {
                                app.start_move_feed_category();
                            }
                            KeyCode::Char('C') => {
                                app.start_delete_category();
                            }
                            KeyCode::Char('o') => {
                                app.mark_current_read();
                                app.open_current_in_browser();
                            }
                            KeyCode::Enter | KeyCode::Char(' ') => {
                                app.mark_current_read();
                                match app.active_pane {
                                    app::ActivePane::Feeds => {
                                        if let Some(app::ChannelTreeItem::CategoryHeader { .. }) = app.current_selected_channel_item() {
                                            app.toggle_selected_category_expand();
                                        } else {
                                            app.active_pane = app::ActivePane::Articles;
                                        }
                                    }
                                    app::ActivePane::Articles => {
                                        app.active_pane = app::ActivePane::Reader;
                                        app.fetch_full_content_for_selected().await;
                                    }
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
                                if app.is_fullscreen_reader {
                                    app.is_fullscreen_reader = false;
                                    app.status_message = "Keluar dari Fullscreen Reader Mode".to_string();
                                } else if app.active_pane == app::ActivePane::Reader {
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
