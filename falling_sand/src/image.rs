use ext_image::{DynamicImage, GenericImage};
use std::cell::RefCell;

use crate::color::Color;

/// Raw RGBA8 image.
#[derive(Clone, Debug, PartialEq)]
pub struct Rgba8Image {
    width: u32,
    height: u32,
    img: DynamicImage,
    dirty: RefCell<bool>,
}

impl Rgba8Image {
    /// Returns a new image.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            img: ext_image::DynamicImage::new_rgba8(width, height),
            width,
            height,
            dirty: RefCell::new(false),
        }
    }

    /// Resizes the given image.
    pub fn resize(&mut self, width: u32, height: u32) {
        let filter = ext_image::imageops::FilterType::Nearest;
        self.img = self.img.resize(width, height, filter);
        *self.dirty.borrow_mut() = true;
    }

    /// Puts the given pixel on the image.
    pub fn put_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x >= self.width() || y >= self.height() {
            return;
        }

        // Mark as dirty
        *self.dirty.borrow_mut() = true;

        self.img.put_pixel(x, y, color.into());
    }

    /// Returns whether the image is dirty since the bytes were last gotten.
    pub fn is_dirty(&self) -> bool {
        *self.dirty.borrow()
    }

    /// Returns the bytes of the image.
    pub fn rgba_bytes(&self) -> &[u8] {
        *self.dirty.borrow_mut() = false;

        self.img.as_bytes()
    }

    /// Returns the width of the image.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the image.
    pub fn height(&self) -> u32 {
        self.height
    }
}
