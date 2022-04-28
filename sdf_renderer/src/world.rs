use ext_glam::Vec3;

use crate::csg::{self, Csg};

pub struct World {
    renderables: Vec<Csg>,
}

impl World {
    pub fn new() -> Self {
        Self {
            renderables: vec![Csg {
                kind: csg::Primitive::Sphere { radius: 1.0 },
                position: (0.0, 0.0, 0.0).into(),
                op: None,
            }],
        }
    }

    pub fn reset(&mut self) {
        self.renderables.clear()
    }

    pub fn scene_sdf(&self, sample_point: Vec3) -> f32 {
        sample_point.length() - 1.0
    }
}

pub enum Renderable {
    Csg(Csg),
}
