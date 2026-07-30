use image::{imageops::FilterType, GenericImageView};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub fn render_image_to_lines(img_bytes: &[u8], target_width: u32, max_height_rows: u32) -> Option<Vec<Line<'static>>> {
    let img = image::load_from_memory(img_bytes).ok()?;

    // Allow full pane width (up to 85 columns for maximum pixel clarity)
    let available_width = target_width.clamp(35, 85);
    let max_pixel_height = (max_height_rows * 2).clamp(24, 60);

    // Preserve exact aspect ratio using Lanczos3 high-definition sharpening filter
    let resized = img.resize(available_width, max_pixel_height, FilterType::Lanczos3);
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
