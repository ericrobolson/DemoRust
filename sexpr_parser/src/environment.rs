pub type Parent = Box<Environment>;

pub struct Environment {}

impl Environment {
    pub fn new(parent: Option<Parent>) {}
}
