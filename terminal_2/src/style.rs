#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b }
    }
}
impl Into<(u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct StyledCell {
    pub background: Option<Color>,
    pub color: Option<Color>,
    pub char: char,
}
impl Default for StyledCell {
    fn default() -> Self {
        Self {
            background: Default::default(),
            color: Default::default(),
            char: ' ',
        }
    }
}
