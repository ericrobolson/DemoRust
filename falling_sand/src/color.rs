#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {
    pub fn white() -> Self {
        (255, 255, 255).into()
    }

    pub fn black() -> Self {
        (0, 0, 0).into()
    }

    pub fn red() -> Self {
        (255, 0, 0).into()
    }

    pub fn green() -> Self {
        (0, 255, 0).into()
    }

    pub fn blue() -> Self {
        (0, 0, 255).into()
    }

    /// Returns normalized f32 values.
    pub fn to_f32(&self) -> (f32, f32, f32, f32) {
        let max = u8::MAX as f32;

        (
            self.r as f32 / max,
            self.g as f32 / max,
            self.b as f32 / max,
            self.a as f32 / max,
        )
    }
}
impl Default for Color {
    fn default() -> Self {
        Self::white()
    }
}

impl From<u8> for Color {
    fn from(c: u8) -> Self {
        (c, c, c).into()
    }
}
impl From<[u8; 3]> for Color {
    fn from([r, g, b]: [u8; 3]) -> Self {
        (r, g, b).into()
    }
}
impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        (r, g, b, 255).into()
    }
}

impl From<(i32, i32, i32)> for Color {
    fn from((r, g, b): (i32, i32, i32)) -> Self {
        (r as u8, g as u8, b as u8, 255).into()
    }
}
impl From<[u8; 4]> for Color {
    fn from([r, g, b, a]: [u8; 4]) -> Self {
        (r, g, b, a).into()
    }
}
impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
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

impl Into<ext_image::Rgba<u8>> for Color {
    fn into(self) -> ext_image::Rgba<u8> {
        ext_image::Rgba([self.a, self.g, self.b, self.a])
    }
}
