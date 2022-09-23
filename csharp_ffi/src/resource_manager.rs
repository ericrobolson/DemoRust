use crate::c_api::{StringId, TextureId};
use std::collections::HashMap;

pub struct ResourceManager {
    strings: HashMap<StringId, String>,
    textures: HashMap<TextureId, ()>,
}
impl ResourceManager {
    pub fn new() -> Self {
        Self {
            strings: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn get_string(&self, string: StringId) -> Option<&String> {
        self.strings.get(&string)
    }
}
