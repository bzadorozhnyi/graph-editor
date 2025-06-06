use std::{fs::File, io::BufWriter, path::PathBuf};

use eframe::egui::{ColorImage, Rect};
use image::{ImageBuffer, Rgba};

use crate::error::GraphEditorError;

pub fn save_color_image_to_png(path: PathBuf, image: &ColorImage) -> Result<(), GraphEditorError> {
    let width = image.width() as u32;
    let height = image.height() as u32;

    let raw = image
        .pixels
        .iter()
        .flat_map(|c| c.to_array())
        .collect::<Vec<_>>();
    let buffer: ImageBuffer<Rgba<u8>, _> =
        ImageBuffer::from_raw(width, height, raw).ok_or(GraphEditorError::FailedTakeScreenshot)?;

    let file = File::create(path).map_err(|_| GraphEditorError::FailedTakeScreenshot)?;
    let mut writer = BufWriter::new(file);
    buffer
        .write_to(&mut writer, image::ImageFormat::Png)
        .map_err(|_| GraphEditorError::FailedTakeScreenshot)?;

    Ok(())
}

pub fn crop_color_image(
    image: &ColorImage,
    rect: Rect,
    pixels_per_point: f32,
) -> Option<ColorImage> {
    let [width, height] = image.size;
    let pixels = &image.pixels;

    let x0 = (rect.min.x * pixels_per_point).round() as usize;
    let y0 = (rect.min.y * pixels_per_point).round() as usize;
    let x1 = (rect.max.x * pixels_per_point).round() as usize;
    let y1 = (rect.max.y * pixels_per_point).round() as usize;

    if x1 > width || y1 > height || x0 >= x1 || y0 >= y1 {
        return None;
    }

    let (cropped_width, cropped_height) = (x1 - x0, y1 - y0);
    let mut cropped_pixels = Vec::with_capacity(cropped_width * cropped_height);

    for y in y0..=y1 {
        let start = y * width + x0;
        let end = y * width + x1;

        cropped_pixels.extend_from_slice(&pixels[start..end]);
    }

    Some(ColorImage {
        size: [cropped_width, cropped_height],
        pixels: cropped_pixels,
    })
}
