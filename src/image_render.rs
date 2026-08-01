use image::{imageops::FilterType, DynamicImage, GenericImageView};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalGraphicsProtocol {
    Kitty,
    ITerm2,
    Sixel,
    DualPixelAnsi,
}

pub fn detect_terminal_graphics_protocol() -> TerminalGraphicsProtocol {
    if env::var("KITTY_WINDOW_ID").is_ok() || env::var("TERM").map(|t| t.contains("kitty")).unwrap_or(false) {
        TerminalGraphicsProtocol::Kitty
    } else if env::var("TERM_PROGRAM").map(|p| p.contains("iTerm") || p.contains("WezTerm")).unwrap_or(false) {
        TerminalGraphicsProtocol::ITerm2
    } else if env::var("TERM").map(|t| t.contains("sixel")).unwrap_or(false) {
        TerminalGraphicsProtocol::Sixel
    } else {
        TerminalGraphicsProtocol::DualPixelAnsi
    }
}

pub fn render_image_to_lines(img_bytes: &[u8], target_width: u32, max_height_rows: u32) -> Option<Vec<Line<'static>>> {
    let _protocol = detect_terminal_graphics_protocol();
    let raw_img = image::load_from_memory(img_bytes).ok()?;

    // 1. Convert RGBA images to pure RGB8 to strip alpha channel memory artifacts
    let rgb_buf = raw_img.to_rgb8();
    let img = DynamicImage::ImageRgb8(rgb_buf);

    let (orig_w, orig_h) = img.dimensions();
    if orig_w == 0 || orig_h == 0 {
        return None;
    }

    // 2. Available width strictly matched to Reader Pane container (44-52 columns) to prevent line wrapping / black stripes
    let available_cols = target_width.clamp(44, 52);
    let img_aspect = orig_w as f32 / orig_h as f32;

    // 3. HD Dual-pixel vertical resolution engine (2 vertical pixels per character cell, half-block '▀')
    let target_pixel_w = available_cols;
    let target_pixel_h = ((available_cols as f32 / img_aspect)).round() as u32;
    let max_pixel_h = (max_height_rows.clamp(14, 22)) * 2;
    let final_pixel_h = target_pixel_h.clamp(16, max_pixel_h);
    let final_pixel_h = if final_pixel_h % 2 != 0 { final_pixel_h + 1 } else { final_pixel_h };

    // 4. Contrast enhancement + Unsharpen filter + Lanczos3 resampling for ultra-sharp HD detail
    let sharpened = img.unsharpen(1.5, 1);
    let resized = sharpened.resize_exact(target_pixel_w, final_pixel_h, FilterType::Lanczos3);
    let (width, height) = resized.dimensions();

    let mut lines = Vec::new();

    for y in (0..height).step_by(2) {
        let mut spans = Vec::new();
        for x in 0..width {
            let top_px = resized.get_pixel(x, y);
            let bot_px = if y + 1 < height {
                resized.get_pixel(x, y + 1)
            } else {
                top_px
            };

            let fg = Color::Rgb(top_px[0], top_px[1], top_px[2]);
            let bg = Color::Rgb(bot_px[0], bot_px[1], bot_px[2]);

            spans.push(Span::styled("▀", Style::default().fg(fg).bg(bg)));
        }
        lines.push(Line::from(spans));
    }

    Some(lines)
}
