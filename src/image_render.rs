use image::{imageops::FilterType, DynamicImage, GenericImageView};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub fn render_image_to_lines(img_bytes: &[u8], target_width: u32, max_height_rows: u32) -> Option<Vec<Line<'static>>> {
    let raw_img = image::load_from_memory(img_bytes).ok()?;

    let rgb_buf = raw_img.to_rgb8();
    let img = DynamicImage::ImageRgb8(rgb_buf);

    let (orig_w, orig_h) = img.dimensions();
    if orig_w == 0 || orig_h == 0 {
        return None;
    }

    // Available width strictly matched to Reader Pane container width to prevent ANY line wrapping / black stripes
    let available_cols = target_width.clamp(40, 56);
    let img_aspect = orig_w as f32 / orig_h as f32;

    let target_pixel_w = available_cols;
    let target_pixel_h = ((available_cols as f32 / img_aspect)).round() as u32;
    let max_pixel_h = (max_height_rows.clamp(14, 22)) * 2;
    let final_pixel_h = target_pixel_h.clamp(16, max_pixel_h);
    let final_pixel_h = if final_pixel_h % 2 != 0 { final_pixel_h + 1 } else { final_pixel_h };

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

