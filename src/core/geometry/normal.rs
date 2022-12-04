use std::ops::{self, Index, IndexMut};

use crate::core::pbrt::Float;

use super::vector::Vector3f;

// Normal3f
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Normal3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Normal3f {
    pub fn new(x: Float, y: Float, z: Float) -> Normal3f {
        Normal3f { x, y, z }
    }

    pub fn has_nans(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    pub fn length_squared(&self) -> Float {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> Float {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, v: &Normal3f) -> Float {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn abs_dot(&self, v: &Normal3f) -> Float {
        self.x.abs() * v.x.abs() + self.y.abs() * v.y.abs() + self.z.abs() * v.z.abs()
    }

    pub fn face_forward(&self, v: &Normal3f) -> Normal3f {
        if self.dot(v) < 0.0 as Float {
            -*self
        } else {
            *self
        }
    }

    pub fn abs(&self) -> Normal3f {
        Normal3f {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn normalize(&self) -> Normal3f {
        *self / self.length()
    }
}

impl From<Normal3f> for Vector3f {
    fn from(n: Normal3f) -> Vector3f {
        Vector3f {
            x: n.x,
            y: n.y,
            z: n.z,
        }
    }
}

impl From<Vector3f> for Normal3f {
    fn from(v: Vector3f) -> Normal3f {
        Normal3f {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl Index<usize> for Normal3f {
    type Output = Float;

    fn index(&self, i: usize) -> &Float {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Normal3f index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Normal3f {
    fn index_mut(&mut self, i: usize) -> &mut Float {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Normal3f index out of bounds"),
        }
    }
}

impl_op_ex!(+ |a: &Normal3f, b: &Normal3f| -> Normal3f {
    Normal3f {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
});

impl_op_ex!(+= |a: &mut Normal3f, b: &Normal3f| {
    a.x += b.x;
    a.y += b.y;
    a.z += b.z;
});

impl_op_ex!(-|a: &Normal3f, b: &Normal3f| -> Normal3f {
    Normal3f {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
});

impl_op_ex!(-= |a: &mut Normal3f, b: &Normal3f| {
    a.x -= b.x;
    a.y -= b.y;
    a.z -= b.z;
});

impl_op_ex!(*|a: &Normal3f, b: Float| -> Normal3f {
    Normal3f {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    }
});

impl_op_ex!(*= |a: &mut Normal3f, b: Float| {
    a.x *= b;
    a.y *= b;
    a.z *= b;
});

impl_op_ex!(/ |a: &Normal3f, b: Float| -> Normal3f {
    let inv = 1.0 as Float / b;
    Normal3f {
        x: a.x * inv,
        y: a.y * inv,
        z: a.z * inv,
    }
});

impl_op_ex!(/= |a: &mut Normal3f, b: Float| {
    let inv = 1.0 as Float / b;
    a.x *= inv;
    a.y *= inv;
    a.z *= inv;
});

impl_op_ex!(-|a: &Normal3f| -> Normal3f {
    Normal3f {
        x: -a.x,
        y: -a.y,
        z: -a.z,
    }
});
