use super::{
    bounds::{Bounds2f, Bounds3f},
    point::{Point2f, Point3f},
    vector::{Vector2f, Vector3f},
};

use std::ops;

// mixed type operations
impl_op_ex!(+ |a: &Point3f, b: &Vector3f| -> Point3f {
    Point3f {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
});

impl_op_ex!(-|a: &Point3f, b: &Vector2f| -> Point2f {
    Point2f {
        x: a.x - b.x,
        y: a.y - b.y,
    }
});

// Misc functions
pub fn point2f_inside_bounds2f(p: &Point2f, b: &Bounds2f) -> bool {
    p.x >= b.p_min.x && p.x <= b.p_max.x && p.y >= b.p_min.y && p.y <= b.p_max.y
}

pub fn point3f_inside_bounds3f(p: &Point3f, b: &Bounds3f) -> bool {
    p.x >= b.p_min.x
        && p.x <= b.p_max.x
        && p.y >= b.p_min.y
        && p.y <= b.p_max.y
        && p.z >= b.p_min.z
        && p.z <= b.p_max.z
}

pub fn point3f_inside_exclusive_bounds3f(p: &Point3f, b: &Bounds3f) -> bool {
    p.x >= b.p_min.x
        && p.x < b.p_max.x
        && p.y >= b.p_min.y
        && p.y < b.p_max.y
        && p.z >= b.p_min.z
        && p.z < b.p_max.z
}
