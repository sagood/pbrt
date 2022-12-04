use std::ops;
use std::ops::{Index, IndexMut};

use crate::core::pbrt::Float;

use super::point::Point3f;

// Vector2f
#[derive(Debug, Default, Copy, Clone)]
pub struct Vector2f {
    pub x: Float,
    pub y: Float,
}

impl Vector2f {
    pub fn new(x: Float, y: Float) -> Self {
        Vector2f { x, y }
    }

    pub fn has_nans(&self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }

    pub fn length_squared(&self) -> Float {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> Float {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, v: &Vector2f) -> Float {
        self.x * v.x + self.y * v.y
    }
}

impl Index<usize> for Vector2f {
    type Output = Float;

    fn index(&self, i: usize) -> &Float {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Vector2f index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vector2f {
    fn index_mut(&mut self, i: usize) -> &mut Float {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Vector2f index out of bounds"),
        }
    }
}

impl_op_ex!(+ |a: &Vector2f, b: &Vector2f| -> Vector2f {
    Vector2f { x: a.x + b.x, y: a.y + b.y }
});

impl_op_ex!(+= |a: &mut Vector2f, b: &Vector2f| {
    a.x += b.x;
    a.y += b.y;
});

impl_op_ex!(-|a: &Vector2f, b: &Vector2f| -> Vector2f {
    Vector2f {
        x: a.x - b.x,
        y: a.y - b.y,
    }
});

impl_op_ex!(-= |a: &mut Vector2f, b: &Vector2f| {
    a.x -= b.x;
    a.y -= b.y;
});

impl_op_ex!(*|a: &Vector2f, b: Float| -> Vector2f {
    Vector2f {
        x: a.x * b,
        y: a.y * b,
    }
});

impl_op_ex!(*= |a: &mut Vector2f, b: Float| {
    a.x *= b;
    a.y *= b;
});

impl_op_ex!(/|a: &Vector2f, b: Float| -> Vector2f {
    Vector2f {
        x: a.x / b,
        y: a.y / b,
    }
});

impl_op_ex!(/= |a: &mut Vector2f, b: Float| {
    a.x /= b;
    a.y /= b;
});

impl_op_ex!(-|a: &Vector2f| -> Vector2f { Vector2f { x: -a.x, y: -a.y } });

// Vector3f
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vector3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Vector3f {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Vector3f { x, y, z }
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
    pub fn normalize(&self) -> Vector3f {
        *self / self.length()
    }

    pub fn dot(&self, v: &Vector3f) -> Float {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn abs_dot(&self, v: &Vector3f) -> Float {
        self.dot(v).abs()
    }

    pub fn cross(&self, v: &Vector3f) -> Vector3f {
        Vector3f {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn min_component(&self) -> Float {
        self.x.min(self.y.min(self.z))
    }

    pub fn max_component(&self) -> Float {
        self.x.max(self.y.max(self.z))
    }

    pub fn max_dimension(&self) -> usize {
        if self.x > self.y {
            if self.x > self.z {
                0
            } else {
                2
            }
        } else {
            if self.y > self.z {
                1
            } else {
                2
            }
        }
    }

    pub fn min(&self, v: &Vector3f) -> Vector3f {
        Vector3f {
            x: self.x.min(v.x),
            y: self.y.min(v.y),
            z: self.z.min(v.z),
        }
    }

    pub fn max(&self, v: &Vector3f) -> Vector3f {
        Vector3f {
            x: self.x.max(v.x),
            y: self.y.max(v.y),
            z: self.z.max(v.z),
        }
    }

    pub fn permute(&self, x: usize, y: usize, z: usize) -> Vector3f {
        Vector3f {
            x: self[x],
            y: self[y],
            z: self[z],
        }
    }
}

impl Index<usize> for Vector3f {
    type Output = Float;

    fn index(&self, i: usize) -> &Float {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vector3f index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vector3f {
    fn index_mut(&mut self, i: usize) -> &mut Float {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vector3f index out of bounds"),
        }
    }
}

impl From<Point3f> for Vector3f {
    fn from(p: Point3f) -> Self {
        Vector3f {
            x: p.x,
            y: p.y,
            z: p.z,
        }
    }
}

impl_op_ex!(+ |a: &Vector3f, b: &Vector3f| -> Vector3f {
    Vector3f {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
});

impl_op_ex!(+= |a: &mut Vector3f, b: &Vector3f| {
    a.x += b.x;
    a.y += b.y;
    a.z += b.z;
});

impl_op_ex!(-|a: &Vector3f, b: &Vector3f| -> Vector3f {
    Vector3f {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
});

impl_op_ex!(-= |a: &mut Vector3f, b: &Vector3f| {
    a.x -= b.x;
    a.y -= b.y;
    a.z -= b.z;
});

impl_op_ex!(*|a: &Vector3f, b: Float| -> Vector3f {
    Vector3f {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    }
});

impl_op_ex!(*= |a: &mut Vector3f, b: Float| {
    a.x *= b;
    a.y *= b;
    a.z *= b;
});

impl_op_ex!(/ |a: &Vector3f, b: Float| -> Vector3f {
    Vector3f {
        x: a.x / b,
        y: a.y / b,
        z: a.z / b,
    }
});

impl_op_ex!(/= |a: &mut Vector3f, b: Float| {
    assert_ne!(b, 0.0 as Float);
    let inv = 1.0 as Float / b;
    a.x *= inv;
    a.y *= inv;
    a.z *= inv;
});

impl_op_ex!(-|a: &Vector3f| -> Vector3f {
    Vector3f {
        x: -a.x,
        y: -a.y,
        z: -a.z,
    }
});

pub fn vec3_coordinate_system(v1: &Vector3f, v2: &mut Vector3f, v3: &mut Vector3f) {
    if v1.x.abs() > v1.y.abs() {
        *v2 = Vector3f {
            x: -v1.z,
            y: 0.0 as Float,
            z: v1.x,
        } / (v1.x * v1.x + v1.z * v1.z).sqrt();
    } else {
        *v2 = Vector3f {
            x: 0.0 as Float,
            y: v1.z,
            z: -v1.y,
        } / (v1.x * v1.x + v1.z * v1.z).sqrt();
    }
    *v3 = v1.cross(&*v2);
}
