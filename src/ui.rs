use crate::app::{ActivePane, ActiveTab, App, InputMode};
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
    fg: Color::Rgb(220, 220, 224),
    accent: Color::Rgb(255, 158, 59),     // Rubah Warm Orange / Fox Gold
    highlight: Color::Rgb(126, 156, 216), // Steel Blue
    muted: Color::Rgb(114, 113, 133),     // Muted Gray
    border: Color::Rgb(84, 88, 117),      // Dark Slate
    border_active: Color::Rgb(255, 158, 59), // Active Orange Border
    success: Color::Rgb(152, 187, 108),  // Sage Green
    warning: Color::Rgb(224, 108, 117),  // Crimson Red
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header Banner
            Constraint::Min(10),   // Main 3-Pane Body
            Constraint::Length(1), // Search input if active / Status bar
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

    if app.input_mode == InputMode::AddFeedTitle
        || app.input_mode == InputMode::AddFeedUrl
        || app.input_mode == InputMode::AddFeedCategory
    {
        draw_add_feed_modal(f, app, f.area());
    }
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let header_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let title_spans = vec![
        Span::styled(" 🦊 RUBAH ", Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD)),
        Span::styled("v0.1.0 ", Style::default().fg(THEME.muted)),
        Span::styled("| Ruang Baca Harian ", Style::default().fg(THEME.fg).add_modifier(Modifier::ITALIC)),
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
        Span::styled("  [1] All Feeds ", Style::default().fg(THEME.muted))
    };

    let tab_fav = if app.active_tab == ActiveTab::Bookmarks {
        Span::styled(" [2] Bookmarks ★ ", Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD))
    } else {
        Span::styled("  [2] Bookmarks ★ ", Style::default().fg(THEME.muted))
    };

    let status_text = if app.is_loading {
        Span::styled(" ⏳ Memuat RSS... ", Style::default().fg(THEME.highlight))
    } else {
        Span::styled(format!(" {} ", app.status_message), Style::default().fg(THEME.fg))
    };

    let right_spans = vec![tab_all, Span::styled(" | ", Style::default().fg(THEME.border)), tab_fav, status_text];
    let right_p = Paragraph::new(Line::from(right_spans))
        .alignment(Alignment::Right)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(THEME.border)));
    
    f.render_widget(right_p, header_layout[1]);
}

fn draw_body(f: &mut Frame, app: &mut App, area: Rect) {
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25), // Feeds Sidebar
            Constraint::Percentage(35), // Articles List
            Constraint::Percentage(40), // Article Reader
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

    let items: Vec<ListItem> = app
        .feeds
        .iter()
        .enumerate()
        .map(|(idx, feed)| {
            let prefix = if idx == app.selected_feed_idx && is_active {
                "▶ "
            } else if idx == app.selected_feed_idx {
                "• "
            } else {
                "  "
            };

            let content = vec![
                Span::styled(prefix, Style::default().fg(THEME.accent)),
                Span::styled(format!("[{}] ", feed.category), Style::default().fg(THEME.highlight)),
                Span::styled(&feed.title, Style::default().fg(THEME.fg)),
            ];

            let style = if idx == app.selected_feed_idx {
                Style::default().bg(Color::Rgb(40, 40, 55)).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            ListItem::new(Line::from(content)).style(style)
        })
        .collect();

    let title = format!(" 📡 Channel ({}) ", app.feeds.len());
    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style),
        );

    f.render_widget(list, area);
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
            let read_symbol = if art.is_read { "○ " } else { "● " };
            let read_color = if art.is_read { THEME.muted } else { THEME.accent };

            let star_symbol = if art.is_bookmarked { "★ " } else { "  " };

            let content = vec![
                Span::styled(read_symbol, Style::default().fg(read_color)),
                Span::styled(star_symbol, Style::default().fg(Color::Yellow)),
                Span::styled(&art.title, Style::default().fg(THEME.fg)),
            ];

            let sub_line = vec![
                Span::styled("   ⏱ ", Style::default().fg(THEME.muted)),
                Span::styled(&art.published, Style::default().fg(THEME.muted)),
                Span::styled(format!(" | {}", art.author), Style::default().fg(THEME.highlight)),
            ];

            let item_lines = vec![Line::from(content), Line::from(sub_line)];

            let style = if idx == app.selected_article_idx {
                Style::default().bg(Color::Rgb(40, 40, 55)).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            ListItem::new(item_lines).style(style)
        })
        .collect();

    let title = format!(" 📰 Berita ({}) ", articles.len());
    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style),
        );

    f.render_widget(list, area);
}

fn draw_reader_pane(f: &mut Frame, app: &mut App, area: Rect) {
    let is_active = app.active_pane == ActivePane::Reader;
    let border_style = if is_active {
        Style::default().fg(THEME.border_active)
    } else {
        Style::default().fg(THEME.border)
    };

    let current_art = app.current_article();
    let text = if let Some(art) = current_art {
        vec![
            Line::from(Span::styled(
                art.title.clone(),
                Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD),
            )),
            Line::from(vec![
                Span::styled("Sumber: ", Style::default().fg(THEME.muted)),
                Span::styled(art.feed_title.clone(), Style::default().fg(THEME.highlight)),
                Span::styled(" | Tanggal: ", Style::default().fg(THEME.muted)),
                Span::styled(art.published.clone(), Style::default().fg(THEME.fg)),
            ]),
            Line::from(vec![
                Span::styled("Penulis: ", Style::default().fg(THEME.muted)),
                Span::styled(art.author.clone(), Style::default().fg(THEME.fg)),
            ]),
            Line::from(vec![
                Span::styled("Link: ", Style::default().fg(THEME.muted)),
                Span::styled(art.link.clone(), Style::default().fg(Color::Cyan).add_modifier(Modifier::UNDERLINED)),
            ]),
            Line::from("─".repeat(area.width.saturating_sub(4) as usize)),
            Line::from(""),
            Line::from(Span::styled(art.content.clone(), Style::default().fg(THEME.fg))),
        ]
    } else {
        vec![Line::from(Span::styled(
            "Pilih artikel untuk membaca konten...",
            Style::default().fg(THEME.muted),
        ))]
    };

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title(" 📖 Pembaca Berita ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style),
        )
        .wrap(Wrap { trim: false })
        .scroll((app.reader_scroll, 0));

    f.render_widget(paragraph, area);
}

fn draw_search_bar(f: &mut Frame, app: &App, area: Rect) {
    if app.input_mode == InputMode::Search || !app.search_query.is_empty() {
        let text = vec![
            Span::styled(" 🔍 Cari: ", Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD)),
            Span::styled(&app.search_query, Style::default().fg(THEME.fg)),
            if app.input_mode == InputMode::Search {
                Span::styled("█", Style::default().fg(THEME.accent))
            } else {
                Span::styled("", Style::default())
            },
        ];
        let p = Paragraph::new(Line::from(text));
        f.render_widget(p, area);
    }
}

fn draw_footer(f: &mut Frame, _app: &App, area: Rect) {
    let keys = vec![
        Span::styled(" [Tab] ", Style::default().fg(THEME.accent)),
        Span::styled("Pane ", Style::default().fg(THEME.fg)),
        Span::styled("[j/k] ", Style::default().fg(THEME.accent)),
        Span::styled("Pilih ", Style::default().fg(THEME.fg)),
        Span::styled("[Enter/o] ", Style::default().fg(THEME.accent)),
        Span::styled("Buka Link ", Style::default().fg(THEME.fg)),
        Span::styled("[b] ", Style::default().fg(THEME.accent)),
        Span::styled("Bookmark ", Style::default().fg(THEME.fg)),
        Span::styled("[r] ", Style::default().fg(THEME.accent)),
        Span::styled("Refresh ", Style::default().fg(THEME.fg)),
        Span::styled("[a] ", Style::default().fg(THEME.accent)),
        Span::styled("Tambah Feed ", Style::default().fg(THEME.fg)),
        Span::styled("[d] ", Style::default().fg(THEME.accent)),
        Span::styled("Hapus ", Style::default().fg(THEME.fg)),
        Span::styled("[/] ", Style::default().fg(THEME.accent)),
        Span::styled("Cari ", Style::default().fg(THEME.fg)),
        Span::styled("[?] ", Style::default().fg(THEME.accent)),
        Span::styled("Bantuan ", Style::default().fg(THEME.fg)),
        Span::styled("[q] ", Style::default().fg(THEME.accent)),
        Span::styled("Keluar ", Style::default().fg(THEME.fg)),
    ];
    let p = Paragraph::new(Line::from(keys)).alignment(Alignment::Center);
    f.render_widget(p, area);
}

fn draw_help_modal(f: &mut Frame, area: Rect) {
    let modal_area = centered_rect(60, 60, area);
    f.render_widget(Clear, modal_area);

    let help_text = vec![
        Line::from(Span::styled("🦊 RUBAH - RUANG BACA HARIAN HELP", Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD))),
        Line::from("──────────────────────────────────────────────────"),
        Line::from(vec![Span::styled("  Tab / Shift+Tab", Style::default().fg(THEME.highlight)), Span::raw(" : Pindah antar panel (Channel, Berita, Reader)")]),
        Line::from(vec![Span::styled("  j / k atau ↓ / ↑", Style::default().fg(THEME.highlight)), Span::raw(" : Navigasi item ke bawah / atas")]),
        Line::from(vec![Span::styled("  Enter / o", Style::default().fg(THEME.highlight)), Span::raw("        : Membuka berita di Web Browser default OS")]),
        Line::from(vec![Span::styled("  b", Style::default().fg(THEME.highlight)), Span::raw("                : Simpan / Hapus bookmark artikel")]),
        Line::from(vec![Span::styled("  r", Style::default().fg(THEME.highlight)), Span::raw("                : Refresh / Muat ulang seluruh RSS feed")]),
        Line::from(vec![Span::styled("  a", Style::default().fg(THEME.highlight)), Span::raw("                : Tambah channel RSS feed baru")]),
        Line::from(vec![Span::styled("  d", Style::default().fg(THEME.highlight)), Span::raw("                : Hapus channel RSS feed terpilih")]),
        Line::from(vec![Span::styled("  /", Style::default().fg(THEME.highlight)), Span::raw("                : Cari kata kunci di judul/konten berita")]),
        Line::from(vec![Span::styled("  1 / 2", Style::default().fg(THEME.highlight)), Span::raw("            : Switch tab All Feeds (1) / Bookmarks (2)")]),
        Line::from(vec![Span::styled("  Esc", Style::default().fg(THEME.highlight)), Span::raw("              : Reset pencarian / Tutup modal bantuan")]),
        Line::from(vec![Span::styled("  q", Style::default().fg(THEME.highlight)), Span::raw("                : Keluar dari aplikasi Rubah")]),
        Line::from(""),
        Line::from(Span::styled("Tekan [Esc] atau [?] untuk menutup bantuan.", Style::default().fg(THEME.muted))),
    ];

    let help_p = Paragraph::new(help_text)
        .block(
            Block::default()
                .title(" Bantuan Shortcut ")
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(THEME.accent)),
        )
        .alignment(Alignment::Left);

    f.render_widget(help_p, modal_area);
}

fn draw_add_feed_modal(f: &mut Frame, app: &App, area: Rect) {
    let modal_area = centered_rect(50, 40, area);
    f.render_widget(Clear, modal_area);

    let block = Block::default()
        .title(" ➕ Tambah RSS Feed Baru ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(THEME.accent));

    f.render_widget(block, modal_area);

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // URL
            Constraint::Length(3), // Category
            Constraint::Min(1),    // Instruction
        ])
        .split(modal_area);

    let title_style = if app.input_mode == InputMode::AddFeedTitle {
        Style::default().fg(THEME.accent)
    } else {
        Style::default().fg(THEME.border)
    };
    let title_p = Paragraph::new(app.new_feed_title.as_str()).block(
        Block::default()
            .title("Judul Feed")
            .borders(Borders::ALL)
            .border_style(title_style),
    );
    f.render_widget(title_p, inner_chunks[0]);

    let url_style = if app.input_mode == InputMode::AddFeedUrl {
        Style::default().fg(THEME.accent)
    } else {
        Style::default().fg(THEME.border)
    };
    let url_p = Paragraph::new(app.new_feed_url.as_str()).block(
        Block::default()
            .title("URL RSS/Atom Feed")
            .borders(Borders::ALL)
            .border_style(url_style),
    );
    f.render_widget(url_p, inner_chunks[1]);

    let cat_style = if app.input_mode == InputMode::AddFeedCategory {
        Style::default().fg(THEME.accent)
    } else {
        Style::default().fg(THEME.border)
    };
    let cat_p = Paragraph::new(app.new_feed_category.as_str()).block(
        Block::default()
            .title("Kategori (Contoh: Teknologi, Berita)")
            .borders(Borders::ALL)
            .border_style(cat_style),
    );
    f.render_widget(cat_p, inner_chunks[2]);

    let hint = Line::from(vec![
        Span::styled("[Tab] ", Style::default().fg(THEME.accent)),
        Span::styled("Next field | ", Style::default().fg(THEME.fg)),
        Span::styled("[Enter] ", Style::default().fg(THEME.accent)),
        Span::styled("Simpan | ", Style::default().fg(THEME.fg)),
        Span::styled("[Esc] ", Style::default().fg(THEME.accent)),
        Span::styled("Batal ", Style::default().fg(THEME.fg)),
    ]);
    f.render_widget(Paragraph::new(hint), inner_chunks[3]);
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
