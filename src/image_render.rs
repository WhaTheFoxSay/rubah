use base64::{engine::general_purpose::STANDARD, Engine as _};
use image::{imageops::FilterType, GenericImageView};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

#[allow(dead_code)]
pub fn render_image_to_lines(img_bytes: &[u8], target_width: u32, max_height_rows: u32) -> Option<Vec<Line<'static>>> {
    let img = image::load_from_memory(img_bytes).ok()?;

    let (orig_w, orig_h) = img.dimensions();
    if orig_w == 0 || orig_h == 0 {
        return None;
    }

    // Determine target dimensions keeping exact aspect ratio
    // Terminal character cells have ~1:2 width:height aspect ratio (height is 2x width)
    let available_cols = target_width.clamp(40, 85);
    let max_pixel_h = max_height_rows * 2;

    // Calculate aspect-ratio preserving dimensions
    let aspect = orig_w as f32 / orig_h as f32;
    let target_pixel_w = available_cols;
    let target_pixel_h = ((available_cols as f32 / aspect) / 1.0).round() as u32;

    let final_pixel_h = target_pixel_h.clamp(16, max_pixel_h);

    // High-definition Lanczos3 anti-aliasing downscaling
    let resized = img.resize_exact(target_pixel_w, final_pixel_h, FilterType::Lanczos3);
    let (width, height) = resized.dimensions();

    let mut lines = Vec::new();

    let mut y = 0;
    while y < height {
        let mut spans = Vec::new();
        for x in 0..width {
            let top_pixel = resized.get_pixel(x, y);
            let bot_pixel = if y + 1 < height {
                resized.get_pixel(x, y + 1)
            } else {
                top_pixel
            };

            let fg = Color::Rgb(top_pixel[0], top_pixel[1], top_pixel[2]);
            let bg = Color::Rgb(bot_pixel[0], bot_pixel[1], bot_pixel[2]);

            spans.push(Span::styled("▀", Style::default().fg(fg).bg(bg)));
        }
        lines.push(Line::from(spans));
        y += 2;
    }

    Some(lines)
}

#[allow(dead_code)]
pub fn generate_iterm2_hd_string(img_bytes: &[u8], width_cols: u16) -> Option<String> {
    if img_bytes.is_empty() {
        return None;
    }
    let b64 = STANDARD.encode(img_bytes);
    Some(format!(
        "\x1b]1337;File=inline=1;width={}ch;preserveAspectRatio=1:{}\x07",
        width_cols, b64
    ))
}
