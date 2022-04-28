// CSG based on this: http://jamie-wong.com/2016/07/15/ray-marching-signed-distance-functions/
// This is simply a data structure to build up an AST of CSGs.

use ext_glam::Vec3;

#[derive(Debug, PartialEq, Clone)]
pub struct Csg {
    pub kind: Primitive,
    pub position: Vec3,
    pub op: Option<Box<Op>>,
}

impl Csg {
    /// Creates a new CSG
    pub fn new(kind: Primitive, position: Vec3) -> Self {
        Self {
            kind,
            op: None,
            position,
        }
    }

    /// Adds an op to the CSG.
    pub fn set_op(&mut self, csg: Self, op: Operation) -> &mut Self {
        self.op = Some(Box::new(Op { op, other: csg }));
        self
    }

    /// Performs a union with another CSG.
    pub fn union(&mut self, csg: Self) -> &mut Self {
        self.set_op(csg, Operation::Union)
    }

    /// Performs an intersection with another CSG.
    pub fn intersect(&mut self, csg: Self) -> &mut Self {
        self.set_op(csg, Operation::Intersection)
    }

    /// Performs a difference with another CSG.
    pub fn difference(&mut self, csg: Self) -> &mut Self {
        self.set_op(csg, Operation::Difference)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Op {
    pub op: Operation,
    pub other: Csg,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Intersection,
    Union,
    Difference,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Primitive {
    Sphere { radius: f32 },
}
