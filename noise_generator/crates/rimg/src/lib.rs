pub use image::ImageError;

/// A RGBA8 image.
/// Represnts a PNG file.
#[derive(Clone, Debug, PartialEq)]
pub struct Png(image::ImageBuffer<image::Rgba<u8>, Vec<u8>>);

impl Png {
    /// Creates a new RGBA8 image of the given size.
    pub fn new(w: usize, h: usize) -> Self {
        let img = image::ImageBuffer::new(w as u32, h as u32);

        Self(img)
    }

    /// Returns the width of the image.
    pub fn width(&self) -> usize {
        self.0.width() as usize
    }

    /// Returns the height of the image.
    pub fn height(&self) -> usize {
        self.0.height() as usize
    }

    /// Returns the images bytes.
    pub fn as_bytes(&self) -> &[u8] {
        let buf = self.0.as_raw();
        buf.as_slice()
    }

    /// Writes the image to the given writer.
    pub fn write_to<W>(&self, writer: &mut W) -> Result<(), ImageError>
    where
        W: std::io::Write + std::io::Seek,
    {
        let f = image::ImageOutputFormat::Png;
        self.0.write_to(writer, f)
    }

    /// Creates an image from the given bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ImageError> {
        match image::load_from_memory(bytes) {
            Ok(img) => Ok(Self(img.into_rgba8())),
            Err(e) => Err(e),
        }
    }

    /// Safely converts the coordinates so it doesn't go out of bounds.
    fn get_coordinates(&self, x: usize, y: usize) -> (usize, usize) {
        let x = x % self.width();
        let y = y % self.height();

        (x, y)
    }

    /// Returns the given pixel.
    pub fn get_pixel(&self, x: usize, y: usize) -> [u8; 4] {
        let (x, y) = self.get_coordinates(x, y);

        let c = self.0.get_pixel(x as u32, y as u32);
        c.0
    }

    /// Sets the given pixel.
    pub fn set_pixel(&mut self, x: usize, y: usize, color: [u8; 4]) {
        let (x, y) = self.get_coordinates(x, y);

        self.0.put_pixel(x as u32, y as u32, color.into());
    }
}

pub struct Color;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_get_pixel() {
        let w = 16;
        let h = 16;
        let mut img = Rgba8::new(w, h);
        let color = [255, 100, 10, 0];
        let x = 15;
        let y = 14;
        img.set_pixel(x, y, color);

        assert_eq!(color, img.get_pixel(x, y));
    }

    #[test]
    fn put_get_pixel_wraps_out_of_bounds() {
        let w = 16;
        let h = 16;
        let mut img = Rgba8::new(w, h);
        let color = [255, 100, 10, 0];
        let x = 255;
        let y = 66;
        img.set_pixel(x, y, color);

        assert_eq!(color, img.get_pixel(x, y));
    }

    #[test]
    fn new_sets_width_height() {
        let w = 255;
        let h = 333;
        let img = Rgba8::new(w, h);

        assert_eq!(w, img.width());
        assert_eq!(h, img.height());
    }
}
