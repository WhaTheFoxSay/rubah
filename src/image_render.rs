use image::{imageops::FilterType, DynamicImage, GenericImageView};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub fn render_image_to_lines(img_bytes: &[u8], target_width: u32, max_height_rows: u32) -> Option<Vec<Line<'static>>> {
    let raw_img = image::load_from_memory(img_bytes).ok()?;

    // Convert RGBA images to pure RGB8 to strip alpha channel memory artifacts
    let rgb_buf = raw_img.to_rgb8();
    let img = DynamicImage::ImageRgb8(rgb_buf);

    let (orig_w, orig_h) = img.dimensions();
    if orig_w == 0 || orig_h == 0 {
        return None;
    }

    // 1. Contrast & edge sharpening
    let sharpened = img.adjust_contrast(10.0);

    // 2. Compact width fitting layout (max 50 columns)
    let available_cols = target_width.clamp(35, 50);
    let font_aspect_ratio = 1.85;
    let img_aspect = orig_w as f32 / orig_h as f32;

    let target_pixel_w = available_cols;
    let target_pixel_h = ((available_cols as f32 / img_aspect) * font_aspect_ratio).round() as u32;

    // Cap max height to 10 terminal rows for neat layout fitting
    let max_pixel_h = (max_height_rows * 2).clamp(14, 20);
    let final_pixel_h = target_pixel_h.clamp(12, max_pixel_h);

    // 3. Lanczos3 high-definition anti-aliased resampling
    let resized = sharpened.resize_exact(target_pixel_w, final_pixel_h, FilterType::Lanczos3);
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
