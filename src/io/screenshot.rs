use crate::io::filter::FileFilter;
use egui::{ColorImage, Pos2, Rect};
use image::ImageError;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

pub struct Screenshot {
    region: Rect,
    file_filter: Option<FileFilter>,
    pixels_per_point: f32,
    image: ColorImage,
}

impl Default for Screenshot {
    fn default() -> Self {
        Self {
            region: Rect::from([Pos2::ZERO, Pos2::ZERO]),
            file_filter: Some(FileFilter::png()),
            pixels_per_point: 0.0,
            image: Default::default(),
        }
    }
}

impl Screenshot {
    pub fn with_region(mut self, rect: Rect) -> Self {
        self.region = rect;
        self
    }

    pub fn with_file_filter(mut self, file_filter: FileFilter) -> Self {
        self.file_filter = Some(file_filter);
        self
    }

    pub fn with_px_per_point(mut self, px_per_point: f32) -> Self {
        self.pixels_per_point = px_per_point;
        self
    }

    pub fn with_image(mut self, image: Arc<ColorImage>) -> Self {
        self.image = image.region(&self.region, Some(self.pixels_per_point));

        self
    }

    pub fn save_dialog(&self) -> Result<(), ImageError> {
        let mut file_dialog = rfd::FileDialog::new();

        if let Some(filter) = &self.file_filter {
            file_dialog = file_dialog.add_filter(&filter.name, &filter.file_extensions)
        }

        if let Some(path) = file_dialog.save_file() {
            Ok(self.save_in(path)?)
        } else {
            Ok(())
        }
    }

    pub fn save_in(&self, path: PathBuf) -> Result<(), ImageError> {
        let result = image::save_buffer(
            &path,
            self.image.as_raw(),
            self.image.width() as u32,
            self.image.height() as u32,
            image::ColorType::Rgba8,
        );

        if result.is_err() {
            let _ = fs::remove_file(&path);
        }

        result
    }
}
