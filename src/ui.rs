use crate::app::{ActivePane, ActiveTab, App, ChannelTreeItem, InputMode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap,
    },
    Frame,
};

#[allow(dead_code)]
pub struct Theme {
    pub bg: Color,
    pub fg: Color,
    pub accent: Color,
    pub highlight: Color,
    pub muted: Color,
    pub border: Color,
    pub border_active: Color,
    pub success: Color,
    pub warning: Color,
}

pub const THEME: Theme = Theme {
    bg: Color::Reset,
    fg: Color::Reset,                        // Auto-adapts to terminal default text color
    accent: Color::Rgb(235, 115, 0),        // Rubah Warm Fox Orange
    highlight: Color::Rgb(40, 110, 210),    // Deep Sapphire Blue
    muted: Color::Rgb(110, 110, 125),       // Neutral Slate Gray
    border: Color::Rgb(140, 145, 160),      // Crisp Border Line
    border_active: Color::Rgb(235, 115, 0), // Active Orange Border
    success: Color::Rgb(40, 160, 70),       // Vibrant Green
    warning: Color::Rgb(220, 60, 60),       // Crimson Red
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let search_height = if app.input_mode == InputMode::Search { 3 } else { 1 };

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header Banner
            Constraint::Min(5),    // Main 3-Pane Body
            Constraint::Length(search_height), // Search input (3 baris) / Status bar (1 baris)
            Constraint::Length(1), // Footer Keybindings
        ])
        .split(f.area());

    draw_header(f, app, main_chunks[0]);
    draw_body(f, app, main_chunks[1]);
    draw_search_bar(f, app, main_chunks[2]);
    draw_footer(f, app, main_chunks[3]);

    if app.show_help {
        draw_help_modal(f, f.area());
    }

    if app.show_uninstall_confirm {
        draw_uninstall_modal(f, f.area());
    }

    if app.input_mode == InputMode::AddFeedTitle
        || app.input_mode == InputMode::AddFeedUrl
        || app.input_mode == InputMode::AddFeedCategory
    {
        draw_add_feed_modal(f, app, f.area());
    }

    if app.input_mode == InputMode::MoveFeedCategory {
        draw_move_category_modal(f, app, f.area());
    }

    if app.input_mode == InputMode::DeleteCategoryConfirm {
        draw_delete_category_modal(f, app, f.area());
    }
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let header_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(area);

    let latency_span = match app.latency_ms {
        Some(ms) if ms < 120 => Span::styled(format!("[{}ms]", ms), Style::default().fg(THEME.success).add_modifier(Modifier::BOLD)),
        Some(ms) if ms < 300 => Span::styled(format!("[{}ms]", ms), Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD)),
        Some(ms) => Span::styled(format!("[{}ms]", ms), Style::default().fg(THEME.warning).add_modifier(Modifier::BOLD)),
        None => Span::styled("[Offline]", Style::default().fg(THEME.warning).add_modifier(Modifier::BOLD)),
    };

    let now = chrono::Local::now();
    let clock_str = now.format("%a, %d %b %Y %H:%M:%S").to_string();

    let title_spans = vec![
        Span::styled(" 🦊 Rubah [Ruang Baca Harian] ", Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD)),
        Span::styled(concat!("v", env!("CARGO_PKG_VERSION"), " "), Style::default().fg(THEME.muted)),
        Span::styled("| ", Style::default().fg(THEME.border)),
        latency_span,
        Span::styled(" | ", Style::default().fg(THEME.border)),
        Span::styled(format!("{} ", clock_str), Style::default().fg(THEME.fg)),
    ];

    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(THEME.border));

    let title_p = Paragraph::new(Line::from(title_spans)).block(title_block);
    f.render_widget(title_p, header_layout[0]);

    let tab_all = if app.active_tab == ActiveTab::AllFeeds {
        Span::styled(" [1] All Feeds ", Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(" [1] All Feeds ", Style::default().fg(THEME.muted))
    };

    let tab_fav = if app.active_tab == ActiveTab::Bookmarks {
        Span::styled(" [2] Bookmarks ", Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(" [2] Bookmarks ", Style::default().fg(THEME.muted))
    };

    let right_spans = vec![tab_all, Span::styled(" | ", Style::default().fg(THEME.border)), tab_fav];
    let right_p = Paragraph::new(Line::from(right_spans))
        .alignment(Alignment::Right)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(THEME.border)));
    
    f.render_widget(right_p, header_layout[1]);
}

fn draw_body(f: &mut Frame, app: &mut App, area: Rect) {
    if app.is_fullscreen_reader {
        draw_reader_pane(f, app, area);
        return;
    }

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(28), // Feeds & Categories Sidebar
            Constraint::Percentage(34), // Articles List
            Constraint::Percentage(38), // Article Reader
        ])
        .split(area);

    draw_feeds_pane(f, app, body_chunks[0]);
    draw_articles_pane(f, app, body_chunks[1]);
    draw_reader_pane(f, app, body_chunks[2]);
}

fn draw_feeds_pane(f: &mut Frame, app: &mut App, area: Rect) {
    let is_active = app.active_pane == ActivePane::Feeds;
    let border_style = if is_active {
        Style::default().fg(THEME.border_active)
    } else {
        Style::default().fg(THEME.border)
    };

    let tree_items = app.visible_channel_items();
    let items: Vec<ListItem> = tree_items
        .iter()
        .enumerate()
        .map(|(idx, item)| {
            let is_selected = idx == app.selected_tree_idx;

            match item {
                ChannelTreeItem::CategoryHeader { name, is_expanded, count } => {
                    let symbol = if *is_expanded { "[-] " } else { "[+] " };
                    let header_text = format!("{} ({})", name, count);

                    let content = vec![
                        Span::styled(symbol, Style::default().fg(if is_selected { Color::Rgb(15, 15, 20) } else { THEME.accent }).add_modifier(Modifier::BOLD)),
                        Span::styled(header_text, Style::default().fg(if is_selected { Color::Rgb(15, 15, 20) } else { THEME.fg }).add_modifier(Modifier::BOLD)),
                    ];

                    let style = if is_selected {
                        Style::default().bg(THEME.accent).fg(Color::Rgb(15, 15, 20)).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(THEME.fg)
                    };

                    ListItem::new(Line::from(content)).style(style)
                }
                ChannelTreeItem::FeedItem { feed, .. } => {
                    let content = vec![
                        Span::styled("  └─ ", Style::default().fg(if is_selected { Color::Rgb(20, 20, 35) } else { THEME.muted })),
                        Span::styled(&feed.title, Style::default().fg(if is_selected { Color::Rgb(15, 15, 20) } else { THEME.fg })),
                    ];

                    let style = if is_selected {
                        Style::default().bg(THEME.accent).fg(Color::Rgb(15, 15, 20)).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(THEME.fg)
                    };

                    ListItem::new(Line::from(content)).style(style)
                }
            }
        })
        .collect();

    let total_feeds = app.feeds.len();
    let title = format!(" Channel & Kategori ({}) ", total_feeds);
    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style),
        );

    f.render_stateful_widget(list, area, &mut app.feed_list_state);
}

fn draw_articles_pane(f: &mut Frame, app: &mut App, area: Rect) {
    let is_active = app.active_pane == ActivePane::Articles;
    let border_style = if is_active {
        Style::default().fg(THEME.border_active)
    } else {
        Style::default().fg(THEME.border)
    };

    let articles = app.current_articles();
    let items: Vec<ListItem> = articles
        .iter()
        .enumerate()
        .map(|(idx, art)| {
            let is_selected = idx == app.selected_article_idx;
            let (dot_symbol, dot_color) = if is_selected {
                ("● ", THEME.success)
            } else if art.is_read {
                ("○ ", THEME.accent)
            } else {
                ("● ", THEME.accent)
            };
            let star_symbol = if art.is_bookmarked { "[B] " } else { "    " };

            let text_color = if is_selected {
                Color::Rgb(15, 15, 20)
            } else {
                THEME.fg
            };

            let sub_color = if is_selected {
                Color::Rgb(30, 30, 45)
            } else {
                THEME.muted
            };

            let author_color = if is_selected {
                Color::Rgb(20, 20, 35)
            } else {
                THEME.highlight
            };

            // Marquee Title Logic for Selected Article
            let max_title_len = area.width.saturating_sub(9) as usize;
            let title_chars: Vec<char> = art.title.chars().collect();

            let display_title = if is_selected && title_chars.len() > max_title_len {
                let overflow = title_chars.len() - max_title_len;
                let pause_ticks = 13;
                let cycle_len = pause_ticks + overflow + pause_ticks;
                let current_step = app.marquee_tick % cycle_len;

                let offset = if current_step < pause_ticks {
                    0
                } else if current_step < pause_ticks + overflow {
                    current_step - pause_ticks
                } else {
                    overflow
                };

                title_chars.iter().skip(offset).take(max_title_len).collect::<String>()
            } else if title_chars.len() > max_title_len {
                let truncated: String = title_chars.iter().take(max_title_len.saturating_sub(1)).collect();
                format!("{}...", truncated)
            } else {
                art.title.clone()
            };

            let content = vec![
                Span::styled(dot_symbol, Style::default().fg(dot_color)),
                Span::styled(star_symbol, Style::default().fg(Color::Yellow)),
                Span::styled(display_title, Style::default().fg(text_color)),
            ];

            let sub_line = vec![
                Span::styled("   Waktu: ", Style::default().fg(sub_color)),
                Span::styled(&art.published, Style::default().fg(sub_color)),
                Span::styled(format!(" | {}", art.author), Style::default().fg(author_color)),
            ];

            let item_lines = vec![Line::from(content), Line::from(sub_line)];

            let style = if is_selected {
                Style::default().bg(THEME.accent).fg(Color::Rgb(15, 15, 20)).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(THEME.fg)
            };

            ListItem::new(item_lines).style(style)
        })
        .collect();

    let title = if !app.search_query.trim().is_empty() {
        format!(" Berita ({}) [Cari: '{}'] (Esc: Reset) ", articles.len(), app.search_query.trim())
    } else {
        format!(" Berita ({}) ", articles.len())
    };
    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style),
        );

    f.render_stateful_widget(list, area, &mut app.article_list_state);
}

fn draw_reader_pane(f: &mut Frame, app: &mut App, area: Rect) {
    let is_active = app.active_pane == ActivePane::Reader;
    let border_style = if is_active {
        Style::default().fg(THEME.border_active)
    } else {
        Style::default().fg(THEME.border)
    };

    let art = match app.current_article() {
        Some(a) => a,
        None => {
            let p = Paragraph::new("Tidak ada berita terpilih")
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .title(" Reader Mode ")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(border_style),
                );
            f.render_widget(p, area);
            return;
        }
    };

    let mut lines = Vec::new();
    lines.push(Line::from(vec![
        Span::styled("Judul   : ", Style::default().fg(THEME.muted)),
        Span::styled(&art.title, Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::from(""));

    lines.push(Line::from(vec![
        Span::styled("Sumber  : ", Style::default().fg(THEME.muted)),
        Span::styled(&art.feed_title, Style::default().fg(THEME.highlight)),
        Span::styled("  |  ", Style::default().fg(THEME.border)),
        Span::styled("Waktu: ", Style::default().fg(THEME.muted)),
        Span::styled(&art.published, Style::default().fg(THEME.fg)),
    ]));

    lines.push(Line::from(vec![
        Span::styled("Penulis : ", Style::default().fg(THEME.muted)),
        Span::styled(&art.author, Style::default().fg(THEME.fg)),
    ]));

    if !art.link.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("Link Web: ", Style::default().fg(THEME.muted)),
            Span::styled(&art.link, Style::default().fg(THEME.highlight).add_modifier(Modifier::UNDERLINED)),
        ]));
    }

    lines.push(Line::from(Span::styled(
        "-".repeat(area.width.saturating_sub(4) as usize),
        Style::default().fg(THEME.border),
    )));

    // Render 24-bit Sharpened Article Image
    if app.show_image {
        if let Some(img_lines) = &app.current_image_lines {
            for line in img_lines {
                lines.push(line.clone());
            }
            lines.push(Line::from(Span::styled(
                "[Foto Berita Utama] - Tekan [i] Toggle On/Off",
                Style::default().fg(THEME.muted).add_modifier(Modifier::ITALIC),
            )));
            lines.push(Line::from(""));
        }
    }

    let mut last_was_empty = true;
    for paragraph in art.content.lines() {
        let trimmed = paragraph.trim();
        if trimmed.is_empty() {
            if !last_was_empty {
                lines.push(Line::from(""));
                last_was_empty = true;
            }
        } else {
            lines.push(Line::from(Span::styled(trimmed, Style::default().fg(THEME.fg))));
            last_was_empty = false;
        }
    }

    let paragraph_widget = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Reader Mode (Tekan [j/k] Scroll | [Esc] Kembali ke Daftar Berita) ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style),
        )
        .wrap(Wrap { trim: false })
        .scroll((app.reader_scroll, 0));

    f.render_widget(paragraph_widget, area);
}

fn draw_search_bar(f: &mut Frame, app: &App, area: Rect) {
    if app.input_mode == InputMode::Search {
        let (query_text, query_style) = if app.search_query.is_empty() {
            ("[Ketik kata kunci di sini...]", Style::default().fg(THEME.muted))
        } else {
            (app.search_query.as_str(), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        };

        let spans = vec![
            Span::styled(" Kata Kunci: ", Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD)),
            Span::styled(query_text, query_style),
            Span::styled(" █", Style::default().fg(THEME.accent)),
            Span::styled("   [Enter] Buka | [Down/Up] Pilih | [Esc] Reset", Style::default().fg(THEME.muted)),
        ];
        let p = Paragraph::new(Line::from(spans)).block(
            Block::default()
                .title(" Mode Pencarian Berita ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(THEME.border_active)),
        );
        f.render_widget(p, area);
    } else if !app.search_query.is_empty() {
        let spans = vec![
            Span::styled(" Filter Cari Aktif: ", Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD)),
            Span::styled(format!("'{}' ", app.search_query), Style::default().fg(THEME.fg)),
            Span::styled(" (Tekan [Esc] untuk bersihkan filter pencarian)", Style::default().fg(THEME.muted)),
        ];
        let p = Paragraph::new(Line::from(spans));
        f.render_widget(p, area);
    } else {
        let spans = vec![
            Span::styled(" Tip: ", Style::default().fg(THEME.muted)),
            Span::styled(&app.status_message, Style::default().fg(THEME.fg)),
        ];
        let p = Paragraph::new(Line::from(spans));
        f.render_widget(p, area);
    }
}

fn draw_footer(f: &mut Frame, _app: &App, area: Rect) {
    let keys = vec![
        Span::styled(" [Tab] ", Style::default().fg(THEME.accent)),
        Span::styled("Navigasi  ", Style::default().fg(THEME.fg)),
        Span::styled("[j/k] ", Style::default().fg(THEME.accent)),
        Span::styled("Pilih  ", Style::default().fg(THEME.fg)),
        Span::styled("[Enter] ", Style::default().fg(THEME.accent)),
        Span::styled("Buka  ", Style::default().fg(THEME.fg)),
        Span::styled("[f] ", Style::default().fg(THEME.accent)),
        Span::styled("Fullscreen  ", Style::default().fg(THEME.fg)),
        Span::styled("[m] ", Style::default().fg(THEME.accent)),
        Span::styled("Pindah Kat  ", Style::default().fg(THEME.fg)),
        Span::styled("[Shift+C] ", Style::default().fg(THEME.accent)),
        Span::styled("Hapus Kat  ", Style::default().fg(THEME.fg)),
        Span::styled("[a] ", Style::default().fg(THEME.accent)),
        Span::styled("Tambah  ", Style::default().fg(THEME.fg)),
        Span::styled("[Shift+D] ", Style::default().fg(THEME.accent)),
        Span::styled("Hapus Feed  ", Style::default().fg(THEME.fg)),
        Span::styled("[?] ", Style::default().fg(THEME.accent)),
        Span::styled("Bantuan  ", Style::default().fg(THEME.fg)),
        Span::styled("[q] ", Style::default().fg(THEME.accent)),
        Span::styled("Keluar ", Style::default().fg(THEME.fg)),
    ];
    let p = Paragraph::new(Line::from(keys)).alignment(Alignment::Center);
    f.render_widget(p, area);
}

fn draw_help_modal(f: &mut Frame, area: Rect) {
    let popup_area = centered_rect(60, 75, area);
    f.render_widget(Clear, popup_area);

    let text = vec![
        Line::from(Span::styled("🦊 Rubah [Ruang Baca Harian] - Bantuan Shortcut Keyboard", Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD))),
        Line::from("--------------------------------------------------"),
        Line::from(vec![Span::styled("Tab / Shift+Tab  ", Style::default().fg(THEME.accent)), Span::raw(": Pindah antar panel")]),
        Line::from(vec![Span::styled("j / k / Up / Down", Style::default().fg(THEME.accent)), Span::raw(": Navigasi item")]),
        Line::from(vec![Span::styled("Enter / Space    ", Style::default().fg(THEME.accent)), Span::raw(": Toggle Kategori / Buka Artikel")]),
        Line::from(vec![Span::styled("f / F            ", Style::default().fg(THEME.accent)), Span::raw(": Toggle Fullscreen Reader Mode")]),
        Line::from(vec![Span::styled("m                ", Style::default().fg(THEME.accent)), Span::raw(": Pindahkan Feed ke Kategori lain")]),
        Line::from(vec![Span::styled("Shift + C        ", Style::default().fg(THEME.accent)), Span::raw(": Hapus Kategori beserta seluruh feed")]),
        Line::from(vec![Span::styled("Shift + D        ", Style::default().fg(THEME.accent)), Span::raw(": Hapus Feed terpilih")]),
        Line::from(vec![Span::styled("Esc              ", Style::default().fg(THEME.accent)), Span::raw(": Kembali ke daftar / reset cari")]),
        Line::from(vec![Span::styled("i                ", Style::default().fg(THEME.accent)), Span::raw(": Toggle Gambar ON/OFF")]),
        Line::from(vec![Span::styled("b                ", Style::default().fg(THEME.accent)), Span::raw(": Simpan / hapus Bookmark")]),
        Line::from(vec![Span::styled("o                ", Style::default().fg(THEME.accent)), Span::raw(": Buka artikel di Web Browser")]),
        Line::from(vec![Span::styled("r                ", Style::default().fg(THEME.accent)), Span::raw(": Refresh / reload seluruh feed")]),
        Line::from(vec![Span::styled("a                ", Style::default().fg(THEME.accent)), Span::raw(": Tambah channel RSS Feed baru")]),
        Line::from(vec![Span::styled("/                ", Style::default().fg(THEME.accent)), Span::raw(": Cari berita realtime")]),
        Line::from(vec![Span::styled("1 / 2            ", Style::default().fg(THEME.accent)), Span::raw(": Switch Tab (All Feeds / Bookmarks)")]),
        Line::from(vec![Span::styled("q                ", Style::default().fg(THEME.accent)), Span::raw(": Keluar dari aplikasi")]),
        Line::from("--------------------------------------------------"),
        Line::from(Span::styled("Tekan Esc atau [?] untuk menutup bantuan ini", Style::default().fg(THEME.muted).add_modifier(Modifier::ITALIC))),
    ];

    let block = Block::default()
        .title(" Bantuan ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(THEME.accent));

    let p = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(p, popup_area);
}

fn draw_uninstall_modal(f: &mut Frame, area: Rect) {
    let popup_area = centered_rect(50, 30, area);
    f.render_widget(Clear, popup_area);

    let text = vec![
        Line::from(Span::styled("Konfirmasi Uninstall Rubah", Style::default().fg(THEME.warning).add_modifier(Modifier::BOLD))),
        Line::from("------------------------------------------"),
        Line::from("Apakah Anda yakin ingin menghapus Rubah"),
        Line::from("dan seluruh data konfigurasinya dari sistem?"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Tekan ", Style::default().fg(THEME.fg)),
            Span::styled("[y]", Style::default().fg(THEME.warning).add_modifier(Modifier::BOLD)),
            Span::styled(" untuk Ya, atau ", Style::default().fg(THEME.fg)),
            Span::styled("[n]", Style::default().fg(THEME.success).add_modifier(Modifier::BOLD)),
            Span::styled(" untuk Batal", Style::default().fg(THEME.fg)),
        ]),
    ];

    let block = Block::default()
        .title(" Uninstall ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(THEME.warning));

    let p = Paragraph::new(text).block(block).alignment(Alignment::Center);
    f.render_widget(p, popup_area);
}

fn draw_add_feed_modal(f: &mut Frame, app: &App, area: Rect) {
    let popup_area = centered_rect(60, 50, area);
    f.render_widget(Clear, popup_area);

    let title_style = if app.input_mode == InputMode::AddFeedTitle {
        Style::default().fg(THEME.accent)
    } else {
        Style::default().fg(THEME.fg)
    };
    let url_style = if app.input_mode == InputMode::AddFeedUrl {
        Style::default().fg(THEME.accent)
    } else {
        Style::default().fg(THEME.fg)
    };
    let cat_style = if app.input_mode == InputMode::AddFeedCategory {
        Style::default().fg(THEME.accent)
    } else {
        Style::default().fg(THEME.fg)
    };

    let text = vec![
        Line::from(Span::styled("Tambah Channel RSS Feed Baru", Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD))),
        Line::from("--------------------------------------------"),
        Line::from(vec![
            Span::styled("1. Judul Channel  : ", title_style),
            Span::styled(&app.new_feed_title, Style::default().fg(THEME.fg)),
            if app.input_mode == InputMode::AddFeedTitle { Span::styled(" |", Style::default().fg(THEME.accent)) } else { Span::raw("") },
        ]),
        Line::from(vec![
            Span::styled("2. URL Feed RSS   : ", url_style),
            Span::styled(&app.new_feed_url, Style::default().fg(THEME.fg)),
            if app.input_mode == InputMode::AddFeedUrl { Span::styled(" |", Style::default().fg(THEME.accent)) } else { Span::raw("") },
        ]),
        Line::from(vec![
            Span::styled("3. Kategori       : ", cat_style),
            Span::styled(&app.new_feed_category, Style::default().fg(THEME.fg)),
            if app.input_mode == InputMode::AddFeedCategory { Span::styled(" |", Style::default().fg(THEME.accent)) } else { Span::raw("") },
        ]),
        Line::from("--------------------------------------------"),
        Line::from(Span::styled("Tip: Tekan [Tab/Up/Down] pada Kategori untuk memilih kategori yang sudah ada.", Style::default().fg(THEME.muted))),
        Line::from(Span::styled("Tekan [Tab] Pindah Input | [Enter] Simpan | [Esc] Batal", Style::default().fg(THEME.accent))),
    ];

    let block = Block::default()
        .title(" Tambah Feed ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(THEME.accent));

    let p = Paragraph::new(text).block(block);
    f.render_widget(p, popup_area);
}

fn draw_move_category_modal(f: &mut Frame, app: &App, area: Rect) {
    let popup_area = centered_rect(55, 35, area);
    f.render_widget(Clear, popup_area);

    let feed_title = match app.current_selected_channel_item() {
        Some(ChannelTreeItem::FeedItem { feed, .. }) => feed.title,
        _ => "Feed".to_string(),
    };

    let text = vec![
        Line::from(Span::styled("Pindahkan Kategori Feed", Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD))),
        Line::from("--------------------------------------------"),
        Line::from(vec![
            Span::styled("Channel  : ", Style::default().fg(THEME.muted)),
            Span::styled(feed_title, Style::default().fg(THEME.fg).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Kategori : ", Style::default().fg(THEME.accent)),
            Span::styled(&app.move_feed_category_input, Style::default().fg(THEME.fg)),
            Span::styled(" |", Style::default().fg(THEME.accent)),
        ]),
        Line::from("--------------------------------------------"),
        Line::from(Span::styled("Tekan [Enter] Simpan Kategori | [Esc] Batal", Style::default().fg(THEME.muted))),
    ];

    let block = Block::default()
        .title(" Pindah Kategori ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(THEME.accent));

    let p = Paragraph::new(text).block(block);
    f.render_widget(p, popup_area);
}

fn draw_delete_category_modal(f: &mut Frame, app: &App, area: Rect) {
    let popup_area = centered_rect(55, 35, area);
    f.render_widget(Clear, popup_area);

    let cat_name = app.target_category_to_delete.as_deref().unwrap_or("Kategori");

    let text = vec![
        Line::from(Span::styled("Konfirmasi Hapus Kategori", Style::default().fg(THEME.warning).add_modifier(Modifier::BOLD))),
        Line::from("--------------------------------------------"),
        Line::from(format!("Apakah Anda yakin ingin menghapus Kategori '{}'", cat_name)),
        Line::from("beserta SELURUH channel RSS Feed di dalamnya?"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Tekan ", Style::default().fg(THEME.fg)),
            Span::styled("[y] / [Enter]", Style::default().fg(THEME.warning).add_modifier(Modifier::BOLD)),
            Span::styled(" Hapus, atau ", Style::default().fg(THEME.fg)),
            Span::styled("[n] / [Esc]", Style::default().fg(THEME.success).add_modifier(Modifier::BOLD)),
            Span::styled(" Batal", Style::default().fg(THEME.fg)),
        ]),
    ];

    let block = Block::default()
        .title(" Hapus Kategori ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(THEME.warning));

    let p = Paragraph::new(text).block(block).alignment(Alignment::Center);
    f.render_widget(p, popup_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
