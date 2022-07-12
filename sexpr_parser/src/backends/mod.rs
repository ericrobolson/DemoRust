pub mod file;
pub mod targetv1;

pub type Target = targetv1::TargetV1;

pub struct Backend {
    target: Target,
}

impl Backend {
    pub fn new(target: Target) -> Self {
        Self { target }
    }
}
