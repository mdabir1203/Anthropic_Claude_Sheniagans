// image_processing.rs

use image::{DynamicImage, ImageError, io::Reader as ImageReader};
use image::imageops::FilterType;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    inner: DynamicImage,
}

impl Image {
    pub fn new(inner: DynamicImage) -> Self {
        Image { inner }
    }

    pub fn width(&self) -> u32 {
        self.inner.width()
    }

    pub fn height(&self) -> u32 {
        self.inner.height()
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.inner = self.inner.resize(new_width, new_height, FilterType::Lanczos3);
    }

    pub fn save(&self, path: &str) -> Result<(), ImageError> {
        self.inner.save(path)
    }
}

pub fn open_image(path: &str) -> Result<DynamicImage, ImageError> {
    image::open(path)
}

pub fn load_image(path: &str) -> Result<Image, ImageError> {
    let img = ImageReader::open(path)?.decode()?;
    Ok(Image::new(img))
}
