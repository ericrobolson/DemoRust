use ext_image::{DynamicImage, GenericImage};
use std::cell::RefCell;

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
        // Safeguard pixels
        let x = x % self.width;
        let y = y % self.height;

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

impl Into<ext_image::Rgba<u8>> for Color {
    fn into(self) -> ext_image::Rgba<u8> {
        ext_image::Rgba([self.a, self.g, self.b, self.a])
    }
}

/// Color struct
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b, a: 255 }
    }
}
impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self { r, g, b, a }
    }
}
impl From<[u8; 3]> for Color {
    fn from([r, g, b]: [u8; 3]) -> Self {
        Self { r, g, b, a: 255 }
    }
}

impl From<[u8; 4]> for Color {
    fn from([r, g, b, a]: [u8; 4]) -> Self {
        Self { r, g, b, a }
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self {
        let r = (r * 255.0) as u8;
        let g = (g * 255.0) as u8;
        let b = (b * 255.0) as u8;
        let a = (a * 255.0) as u8;

        Self { r, g, b, a }
    }
}

impl From<[f32; 4]> for Color {
    fn from([r, g, b, a]: [f32; 4]) -> Self {
        let r = (r * 255.0) as u8;
        let g = (g * 255.0) as u8;
        let b = (b * 255.0) as u8;
        let a = (a * 255.0) as u8;

        Self { r, g, b, a }
    }
}
