use crate::app::{ActivePane, ActiveTab, App, ChannelTreeItem, InputMode};
use crate::i18n::t;
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
        draw_help_modal(f, app, f.area());
    }

    if app.show_uninstall_confirm {
        draw_uninstall_modal(f, app, f.area());
    }

    if app.is_updating_in_app {
        draw_update_progress_modal(f, app, f.area());
    } else if app.show_update_modal {
        draw_update_modal(f, app, f.area());
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

    let sub_title = t(app.language, "sub_title");
    let tab_all_str = format!(" [1] {} ", t(app.language, "tab_all_feeds"));
    let tab_fav_str = format!(" [2] {} ", t(app.language, "tab_bookmarks"));

    let title_spans = vec![
        Span::styled(format!(" 🦊 Rubah [{}] ", sub_title), Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD)),
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
        Span::styled(tab_all_str, Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(tab_all_str, Style::default().fg(THEME.muted))
    };

    let tab_fav = if app.active_tab == ActiveTab::Bookmarks {
        Span::styled(tab_fav_str, Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(tab_fav_str, Style::default().fg(THEME.muted))
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
    let feeds_title_label = t(app.language, "pane_channels").trim();
    let title = format!(" {} ({}) ", feeds_title_label, total_feeds);
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
                Color::Rgb(20, 40, 25)
            } else {
                THEME.success
            };

            // Dynamic marquee animation for selected article title if wider than column width
            let max_title_len = (area.width.saturating_sub(10)) as usize;
            let title_chars: Vec<char> = art.title.chars().collect();
            let display_title = if is_selected && title_chars.len() > max_title_len {
                let overflow = title_chars.len() - max_title_len;
                let total_steps = overflow + 10; // 5 steps pause at start + 5 steps pause at end
                let current_step = app.marquee_tick % total_steps;

                let pause_ticks = 5;
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

    let articles_title_label = t(app.language, "pane_articles").trim();
    let title = if !app.search_query.trim().is_empty() {
        format!(" {} ({}) [Search: '{}'] (Esc: Reset) ", articles_title_label, articles.len(), app.search_query.trim())
    } else {
        format!(" {} ({}) ", articles_title_label, articles.len())
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
            let p = Paragraph::new(t(app.language, "reader_select_prompt"))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .title(t(app.language, "pane_reader"))
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
        Span::styled(t(app.language, "reader_source"), Style::default().fg(THEME.muted)),
        Span::styled(&art.feed_title, Style::default().fg(THEME.highlight)),
        Span::styled("  |  ", Style::default().fg(THEME.border)),
        Span::styled(t(app.language, "reader_published"), Style::default().fg(THEME.muted)),
        Span::styled(&art.published, Style::default().fg(THEME.fg)),
    ]));

    lines.push(Line::from(vec![
        Span::styled(t(app.language, "reader_author"), Style::default().fg(THEME.muted)),
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
    let lang = app.language;
    if app.input_mode == InputMode::Search {
        let (query_text, query_style) = if app.search_query.is_empty() {
            (t(lang, "search_placeholder"), Style::default().fg(THEME.muted))
        } else {
            (app.search_query.as_str(), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        };

        let spans = vec![
            Span::styled(format!(" {} ", t(lang, "search_keyword")), Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD)),
            Span::styled(query_text, query_style),
            Span::styled(" █", Style::default().fg(THEME.accent)),
            Span::styled(t(lang, "search_hints"), Style::default().fg(THEME.muted)),
        ];
        let p = Paragraph::new(Line::from(spans)).block(
            Block::default()
                .title(t(lang, "search_title"))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(THEME.border_active)),
        );
        f.render_widget(p, area);
    } else if !app.search_query.is_empty() {
        let spans = vec![
            Span::styled(format!(" {} ", t(lang, "search_filter_active")), Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD)),
            Span::styled(format!("'{}' ", app.search_query), Style::default().fg(THEME.fg)),
            Span::styled(t(lang, "search_filter_hint"), Style::default().fg(THEME.muted)),
        ];
        let p = Paragraph::new(Line::from(spans));
        f.render_widget(p, area);
    } else {
        let spans = vec![
            Span::styled(format!(" {} ", t(lang, "status_tip_prefix")), Style::default().fg(THEME.muted)),
            Span::styled(&app.status_message, Style::default().fg(THEME.fg)),
        ];
        let p = Paragraph::new(Line::from(spans));
        f.render_widget(p, area);
    }
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let lang = app.language;
    let keys = vec![
        Span::styled(" [Tab] ", Style::default().fg(THEME.accent)),
        Span::styled(format!("{}  ", t(lang, "footer_nav")), Style::default().fg(THEME.fg)),
        Span::styled("[j/k] ", Style::default().fg(THEME.accent)),
        Span::styled(format!("{}  ", t(lang, "footer_select")), Style::default().fg(THEME.fg)),
        Span::styled("[Enter] ", Style::default().fg(THEME.accent)),
        Span::styled(format!("{}  ", t(lang, "footer_open")), Style::default().fg(THEME.fg)),
        Span::styled("[f] ", Style::default().fg(THEME.accent)),
        Span::styled(format!("{}  ", t(lang, "footer_fullscreen")), Style::default().fg(THEME.fg)),
        Span::styled("[l] ", Style::default().fg(THEME.accent)),
        Span::styled(format!("{}  ", t(lang, "footer_lang")), Style::default().fg(THEME.fg)),
        Span::styled("[u] ", Style::default().fg(THEME.accent)),
        Span::styled(format!("{}  ", t(lang, "footer_update")), Style::default().fg(THEME.fg)),
        Span::styled("[?] ", Style::default().fg(THEME.accent)),
        Span::styled(format!("{}  ", t(lang, "footer_help")), Style::default().fg(THEME.fg)),
        Span::styled("[q] ", Style::default().fg(THEME.accent)),
        Span::styled(t(lang, "footer_quit"), Style::default().fg(THEME.fg)),
    ];
    let p = Paragraph::new(Line::from(keys)).alignment(Alignment::Center);
    f.render_widget(p, area);
}

fn draw_help_modal(f: &mut Frame, app: &App, area: Rect) {
    let popup_area = centered_rect(60, 78, area);
    f.render_widget(Clear, popup_area);

    let lang = app.language;
    let text = vec![
        Line::from(Span::styled(t(lang, "help_heading"), Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD))),
        Line::from("--------------------------------------------------"),
        Line::from(vec![Span::styled("Tab / Shift+Tab  ", Style::default().fg(THEME.accent)), Span::raw(t(lang, "help_tab"))]),
        Line::from(vec![Span::styled("j / k / Up / Down", Style::default().fg(THEME.accent)), Span::raw(t(lang, "help_jk"))]),
        Line::from(vec![Span::styled("Enter / Space    ", Style::default().fg(THEME.accent)), Span::raw(t(lang, "help_enter"))]),
        Line::from(vec![Span::styled("f / F            ", Style::default().fg(THEME.accent)), Span::raw(t(lang, "help_f"))]),
        Line::from(vec![Span::styled("l / L            ", Style::default().fg(THEME.accent)), Span::raw(t(lang, "help_l"))]),
        Line::from(vec![Span::styled("u / U            ", Style::default().fg(THEME.accent)), Span::raw(t(lang, "help_u"))]),
        Line::from(vec![Span::styled("m                ", Style::default().fg(THEME.accent)), Span::raw(t(lang, "help_m"))]),
        Line::from(vec![Span::styled("Shift + C        ", Style::default().fg(THEME.accent)), Span::raw(t(lang, "help_shift_c"))]),
        Line::from(vec![Span::styled("Shift + D        ", Style::default().fg(THEME.accent)), Span::raw(t(lang, "help_shift_d"))]),
        Line::from(vec![Span::styled("Esc              ", Style::default().fg(THEME.accent)), Span::raw(t(lang, "help_esc"))]),
        Line::from(vec![Span::styled("i                ", Style::default().fg(THEME.accent)), Span::raw(t(lang, "help_i"))]),
        Line::from(vec![Span::styled("b                ", Style::default().fg(THEME.accent)), Span::raw(t(lang, "help_b"))]),
        Line::from(vec![Span::styled("o                ", Style::default().fg(THEME.accent)), Span::raw(t(lang, "help_o"))]),
        Line::from(vec![Span::styled("r                ", Style::default().fg(THEME.accent)), Span::raw(t(lang, "help_r"))]),
        Line::from(vec![Span::styled("a                ", Style::default().fg(THEME.accent)), Span::raw(t(lang, "help_a"))]),
        Line::from(vec![Span::styled("/                ", Style::default().fg(THEME.accent)), Span::raw(t(lang, "help_search"))]),
        Line::from(vec![Span::styled("1 / 2            ", Style::default().fg(THEME.accent)), Span::raw(t(lang, "help_tabs"))]),
        Line::from(vec![Span::styled("q                ", Style::default().fg(THEME.accent)), Span::raw(t(lang, "help_q"))]),
        Line::from("--------------------------------------------------"),
        Line::from(Span::styled(t(lang, "help_close"), Style::default().fg(THEME.muted).add_modifier(Modifier::ITALIC))),
    ];

    let block = Block::default()
        .title(t(lang, "help_title"))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(THEME.border_active));

    let p = Paragraph::new(text).wrap(Wrap { trim: true });
    f.render_widget(p.block(block), popup_area);
}

fn draw_update_modal(f: &mut Frame, app: &App, area: Rect) {
    let popup_area = centered_rect(60, 52, area);
    f.render_widget(Clear, popup_area);
    let lang = app.language;

    let (title, border_color, lines) = if let Some(ref info) = app.update_info {
        if info.has_update {
            let lines = vec![
                Line::from(Span::styled(t(lang, "update_msg_new"), Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD))),
                Line::from("--------------------------------------------------"),
                Line::from(vec![Span::styled(t(lang, "update_curr_ver"), Style::default().fg(THEME.muted)), Span::styled(format!("v{}", info.current_version), Style::default().fg(THEME.fg))]),
                Line::from(vec![Span::styled(t(lang, "update_latest_ver"), Style::default().fg(THEME.muted)), Span::styled(format!("v{}", info.latest_version), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))]),
                Line::from("--------------------------------------------------"),
                Line::from(Span::styled(t(lang, "update_notes_label"), Style::default().fg(THEME.fg).add_modifier(Modifier::BOLD))),
                Line::from(Span::styled(&info.release_notes, Style::default().fg(THEME.muted))),
                Line::from("--------------------------------------------------"),
                Line::from(vec![
                    Span::styled("Proses update ke v".to_string() + &info.latest_version + " langsung dari dalam aplikasi?", Style::default().fg(THEME.fg)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Tekan ", Style::default().fg(THEME.fg)),
                    Span::styled("[y]", Style::default().fg(THEME.success).add_modifier(Modifier::BOLD)),
                    Span::styled(" Ya, Update Sekarang | ", Style::default().fg(THEME.fg)),
                    Span::styled("[n / Esc]", Style::default().fg(THEME.warning).add_modifier(Modifier::BOLD)),
                    Span::styled(" Batal", Style::default().fg(THEME.fg)),
                ]),
            ];
            (t(lang, "update_title_new"), THEME.accent, lines)
        } else {
            let lines = vec![
                Line::from(Span::styled(t(lang, "update_msg_latest"), Style::default().fg(THEME.success).add_modifier(Modifier::BOLD))),
                Line::from("--------------------------------------------------"),
                Line::from(vec![Span::styled(t(lang, "update_curr_ver"), Style::default().fg(THEME.muted)), Span::styled(format!("v{}", info.current_version), Style::default().fg(THEME.fg))]),
                Line::from(vec![Span::styled(t(lang, "update_latest_ver"), Style::default().fg(THEME.muted)), Span::styled(t(lang, "update_status_up_to_date"), Style::default().fg(THEME.success).add_modifier(Modifier::BOLD))]),
                Line::from("--------------------------------------------------"),
                Line::from(Span::styled(t(lang, "update_msg_up_to_date"), Style::default().fg(THEME.fg))),
                Line::from("--------------------------------------------------"),
                Line::from(Span::styled(t(lang, "update_close_hint"), Style::default().fg(THEME.muted).add_modifier(Modifier::ITALIC))),
            ];
            (t(lang, "update_title_latest"), THEME.success, lines)
        }
    } else {
        let lines = vec![
            Line::from(Span::styled(" 🦊 Memeriksa Pembaruan... ", Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD))),
            Line::from("Menghubungi server GitHub API..."),
        ];
        (" Periksa Update ", THEME.border_active, lines)
    };

    let p = Paragraph::new(lines).wrap(Wrap { trim: true });
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color));

    f.render_widget(p.block(block), popup_area);
}

fn draw_update_progress_modal(f: &mut Frame, app: &App, area: Rect) {
    let popup_area = centered_rect(65, 45, area);
    f.render_widget(Clear, popup_area);

    let spinner_frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let spinner = spinner_frames[app.marquee_tick % spinner_frames.len()];

    let mut lines = Vec::new();

    if app.update_completed {
        lines.push(Line::from(Span::styled("✔ Pembaruan Berhasil Terpasang!", Style::default().fg(THEME.success).add_modifier(Modifier::BOLD))));
        lines.push(Line::from("--------------------------------------------------"));
        lines.push(Line::from(Span::styled("Biner Rubah versi rilis terbaru telah berhasil diunduh dan dipasang.", Style::default().fg(THEME.fg))));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled("Tekan [Enter] atau [Esc] untuk merestart aplikasi.", Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD))));
    } else if let Some(ref err) = app.update_failed {
        lines.push(Line::from(Span::styled("⚠️ Gagal Melakukan Pembaruan", Style::default().fg(THEME.warning).add_modifier(Modifier::BOLD))));
        lines.push(Line::from("--------------------------------------------------"));
        lines.push(Line::from(Span::styled(err, Style::default().fg(THEME.fg))));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled("Tekan [Enter] atau [Esc] untuk menutup.", Style::default().fg(THEME.muted))));
    } else {
        lines.push(Line::from(vec![
            Span::styled(format!("{} ", spinner), Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD)),
            Span::styled("Memproses Pembaruan Otomatis Dari Dalam Aplikasi...", Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD)),
        ]));
        lines.push(Line::from("--------------------------------------------------"));
        lines.push(Line::from(Span::styled(&app.update_stage_status, Style::default().fg(THEME.fg))));
        lines.push(Line::from(""));

        // Progress Bar Geometry
        let bar_width = 30;
        let filled = ((app.update_percentage / 100.0) * bar_width as f32).round() as usize;
        let filled = filled.min(bar_width);
        let empty = bar_width - filled;

        let bar_str = format!("[{}{}] {:.1}%", "█".repeat(filled), "░".repeat(empty), app.update_percentage);
        lines.push(Line::from(Span::styled(bar_str, Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD))));

        if app.update_total_bytes > 0 {
            let mb_downloaded = app.update_downloaded_bytes as f64 / 1_048_576.0;
            let mb_total = app.update_total_bytes as f64 / 1_048_576.0;
            lines.push(Line::from(Span::styled(
                format!("  Ukuran File Patch: {:.2} MB / {:.2} MB", mb_downloaded, mb_total),
                Style::default().fg(THEME.muted),
            )));
        }
        lines.push(Line::from("--------------------------------------------------"));
        lines.push(Line::from(Span::styled("Harap tunggu, biner baru sedang diunduh dan dipasang secara otomatis...", Style::default().fg(THEME.muted).add_modifier(Modifier::ITALIC))));
    }

    let block = Block::default()
        .title(" 🦊 In-App Auto Updater ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(if app.update_completed { THEME.success } else { THEME.accent }));

    let p = Paragraph::new(lines).wrap(Wrap { trim: true });
    f.render_widget(p.block(block), popup_area);
}

fn draw_uninstall_modal(f: &mut Frame, app: &App, area: Rect) {
    let popup_area = centered_rect(52, 30, area);
    f.render_widget(Clear, popup_area);
    let lang = app.language;

    let text = vec![
        Line::from(Span::styled(t(lang, "uninstall_heading"), Style::default().fg(THEME.warning).add_modifier(Modifier::BOLD))),
        Line::from("------------------------------------------"),
        Line::from(t(lang, "uninstall_body_1")),
        Line::from(t(lang, "uninstall_body_2")),
        Line::from(""),
        Line::from(vec![
            Span::styled(t(lang, "uninstall_press_y"), Style::default().fg(THEME.fg)),
            Span::styled("[y]", Style::default().fg(THEME.warning).add_modifier(Modifier::BOLD)),
            Span::styled(t(lang, "uninstall_y_label"), Style::default().fg(THEME.fg)),
            Span::styled("[n]", Style::default().fg(THEME.success).add_modifier(Modifier::BOLD)),
            Span::styled(t(lang, "uninstall_n_label"), Style::default().fg(THEME.fg)),
        ]),
    ];

    let block = Block::default()
        .title(t(lang, "uninstall_title"))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(THEME.warning));

    let p = Paragraph::new(text).block(block).alignment(Alignment::Center);
    f.render_widget(p, popup_area);
}

fn draw_add_feed_modal(f: &mut Frame, app: &App, area: Rect) {
    let popup_area = centered_rect(60, 50, area);
    f.render_widget(Clear, popup_area);
    let lang = app.language;

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
        Line::from(Span::styled(t(lang, "add_modal_heading"), Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD))),
        Line::from("--------------------------------------------"),
        Line::from(vec![
            Span::styled(t(lang, "add_field_title"), title_style),
            Span::styled(&app.new_feed_title, Style::default().fg(THEME.fg)),
            if app.input_mode == InputMode::AddFeedTitle { Span::styled(" |", Style::default().fg(THEME.accent)) } else { Span::raw("") },
        ]),
        Line::from(vec![
            Span::styled(t(lang, "add_field_url"), url_style),
            Span::styled(&app.new_feed_url, Style::default().fg(THEME.fg)),
            if app.input_mode == InputMode::AddFeedUrl { Span::styled(" |", Style::default().fg(THEME.accent)) } else { Span::raw("") },
        ]),
        Line::from(vec![
            Span::styled(t(lang, "add_field_cat"), cat_style),
            Span::styled(&app.new_feed_category, Style::default().fg(THEME.fg)),
            if app.input_mode == InputMode::AddFeedCategory { Span::styled(" |", Style::default().fg(THEME.accent)) } else { Span::raw("") },
        ]),
        Line::from("--------------------------------------------"),
        Line::from(Span::styled(t(lang, "add_cat_tip"), Style::default().fg(THEME.muted))),
        Line::from(Span::styled(t(lang, "add_hints"), Style::default().fg(THEME.accent))),
    ];

    let block = Block::default()
        .title(t(lang, "add_modal_title"))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(THEME.accent));

    let p = Paragraph::new(text).block(block);
    f.render_widget(p, popup_area);
}

fn draw_move_category_modal(f: &mut Frame, app: &App, area: Rect) {
    let popup_area = centered_rect(55, 35, area);
    f.render_widget(Clear, popup_area);
    let lang = app.language;

    let feed_title = match app.current_selected_channel_item() {
        Some(ChannelTreeItem::FeedItem { feed, .. }) => feed.title,
        _ => "Feed".to_string(),
    };

    let text = vec![
        Line::from(Span::styled(t(lang, "move_modal_heading"), Style::default().fg(THEME.accent).add_modifier(Modifier::BOLD))),
        Line::from("--------------------------------------------"),
        Line::from(vec![
            Span::styled("Channel  : ", Style::default().fg(THEME.muted)),
            Span::styled(feed_title, Style::default().fg(THEME.fg).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(t(lang, "add_field_cat"), Style::default().fg(THEME.accent)),
            Span::styled(&app.move_feed_category_input, Style::default().fg(THEME.fg)),
            Span::styled(" |", Style::default().fg(THEME.accent)),
        ]),
        Line::from("--------------------------------------------"),
        Line::from(Span::styled(t(lang, "move_hints"), Style::default().fg(THEME.muted))),
    ];

    let block = Block::default()
        .title(t(lang, "move_modal_title"))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(THEME.accent));

    let p = Paragraph::new(text).block(block);
    f.render_widget(p, popup_area);
}

fn draw_delete_category_modal(f: &mut Frame, app: &App, area: Rect) {
    let popup_area = centered_rect(55, 35, area);
    f.render_widget(Clear, popup_area);
    let lang = app.language;

    let cat_name = app.target_category_to_delete.as_deref().unwrap_or("Kategori");

    let text = vec![
        Line::from(Span::styled(t(lang, "del_cat_heading"), Style::default().fg(THEME.warning).add_modifier(Modifier::BOLD))),
        Line::from("--------------------------------------------"),
        Line::from(format!("{} '{}'", t(lang, "del_cat_warning"), cat_name)),
        Line::from(t(lang, "del_cat_sub")),
        Line::from(""),
        Line::from(vec![
            Span::styled("Tekan ", Style::default().fg(THEME.fg)),
            Span::styled("[y] / [Enter]", Style::default().fg(THEME.warning).add_modifier(Modifier::BOLD)),
            Span::styled(t(lang, "del_cat_y"), Style::default().fg(THEME.fg)),
            Span::styled("[n] / [Esc]", Style::default().fg(THEME.success).add_modifier(Modifier::BOLD)),
            Span::styled(t(lang, "del_cat_n"), Style::default().fg(THEME.fg)),
        ]),
    ];

    let block = Block::default()
        .title(t(lang, "del_cat_title"))
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
