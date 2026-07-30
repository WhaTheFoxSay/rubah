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

    // 2. Sharpen contrast & detail
    let sharpened = img.adjust_contrast(12.0);

    // 3. Calculate accurate aspect ratio for full-block character rendering
    let available_cols = target_width.clamp(30, 46);
    let font_aspect_ratio = 0.52; // Aspect ratio adjustment for terminal character cell
    let img_aspect = orig_w as f32 / orig_h as f32;

    let target_pixel_w = available_cols;
    let target_pixel_h = ((available_cols as f32 / img_aspect) * font_aspect_ratio).round() as u32;

    let max_pixel_h = max_height_rows.clamp(10, 16);
    let final_pixel_h = target_pixel_h.clamp(8, max_pixel_h);

    // 4. Lanczos3 high-definition anti-aliased resampling
    let resized = sharpened.resize_exact(target_pixel_w, final_pixel_h, FilterType::Lanczos3);
    let (width, height) = resized.dimensions();

    let mut lines = Vec::new();

    // 5. Render using solid full block '█' with FG-only 24-bit TrueColor
    // This eliminates terminal cell background (bg) truecolor mapping glitches on macOS Terminal.app
    for y in 0..height {
        let mut spans = Vec::new();
        for x in 0..width {
            let px = resized.get_pixel(x, y);
            let fg = Color::Rgb(px[0], px[1], px[2]);
            spans.push(Span::styled("█", Style::default().fg(fg)));
        }
        lines.push(Line::from(spans));
    }

    Some(lines)
}
