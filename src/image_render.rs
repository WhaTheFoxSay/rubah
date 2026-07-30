use image::{imageops::FilterType, DynamicImage, GenericImageView};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub fn render_image_to_lines(img_bytes: &[u8], target_width: u32, max_height_rows: u32) -> Option<Vec<Line<'static>>> {
    let raw_img = image::load_from_memory(img_bytes).ok()?;

    // 1. Convert RGBA images to pure RGB8 to strip alpha channel memory artifacts
    let rgb_buf = raw_img.to_rgb8();
    let img = DynamicImage::ImageRgb8(rgb_buf);

    let (orig_w, orig_h) = img.dimensions();
    if orig_w == 0 || orig_h == 0 {
        return None;
    }

    // Strictly clamp available width to max 46 columns to fit inside Reader Pane (40% width) without line wrapping
    let available_cols = target_width.clamp(28, 46);
    let img_aspect = orig_w as f32 / orig_h as f32;

    let mut lines = Vec::new();

    #[cfg(target_os = "windows")]
    {
        // On Windows CMD/PowerShell (conhost.exe), solid full-block '█' with FG-only 24-bit TrueColor
        let font_aspect_ratio = 0.52;
        let target_pixel_w = available_cols;
        let target_pixel_h = ((available_cols as f32 / img_aspect) * font_aspect_ratio).round() as u32;
        let final_pixel_h = target_pixel_h.clamp(8, max_height_rows.clamp(10, 16));

        let resized = img.resize_exact(target_pixel_w, final_pixel_h, FilterType::Lanczos3);
        let (width, height) = resized.dimensions();

        for y in 0..height {
            let mut spans = Vec::new();
            for x in 0..width {
                let px = resized.get_pixel(x, y);
                let fg = Color::Rgb(px[0], px[1], px[2]);
                spans.push(Span::styled("█", Style::default().fg(fg)));
            }
            lines.push(Line::from(spans));
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // On macOS and Linux terminals, half-block '▀' dual-pixel FG+BG engine
        let target_pixel_w = available_cols;
        let target_pixel_h = ((available_cols as f32 / img_aspect)).round() as u32;
        let max_pixel_h = (max_height_rows.clamp(8, 16)) * 2;
        let final_pixel_h = target_pixel_h.clamp(12, max_pixel_h);
        let final_pixel_h = if final_pixel_h % 2 != 0 { final_pixel_h + 1 } else { final_pixel_h };

        let resized = img.resize_exact(target_pixel_w, final_pixel_h, FilterType::Lanczos3);
        let (width, height) = resized.dimensions();

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
    }

    Some(lines)
}
