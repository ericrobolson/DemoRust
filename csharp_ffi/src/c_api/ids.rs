/// A handle for a texture.
#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
#[repr(C)]
pub struct TextureId {
    pub id: u64,
}

/// A handle for a string.
#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
#[repr(C)]
pub struct StringId {
    pub id: u64,
}
