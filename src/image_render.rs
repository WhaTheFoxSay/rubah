use image::{imageops::FilterType, GenericImageView};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub fn render_image_to_lines(img_bytes: &[u8], target_width: u32, target_height_rows: u32) -> Option<Vec<Line<'static>>> {
    let img = image::load_from_memory(img_bytes).ok()?;

    let target_pixel_width = target_width.clamp(20, 60);
    // Each terminal row represents 2 vertical pixels
    let target_pixel_height = target_height_rows * 2;

    let resized = img.resize_exact(target_pixel_width, target_pixel_height, FilterType::Triangle);
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
