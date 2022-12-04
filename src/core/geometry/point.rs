use std::ops;
use std::ops::{Index, IndexMut};

use crate::core::pbrt::Float;

use super::vector::{Vector2f, Vector3f};

// Point2f
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Point2f {
    pub x: Float,
    pub y: Float,
}

impl Point2f {
    pub fn new(x: Float, y: Float) -> Point2f {
        Point2f { x, y }
    }

    pub fn has_nans(&self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }

    pub fn distance(&self, p: &Point2f) -> Float {
        (*self - *p).length()
    }
}

impl Index<usize> for Point2f {
    type Output = Float;

    fn index(&self, i: usize) -> &Float {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Point2f index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Point2f {
    fn index_mut(&mut self, i: usize) -> &mut Float {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Point2f index out of bounds"),
        }
    }
}

impl From<Point3f> for Point2f {
    fn from(p: Point3f) -> Point2f {
        Point2f { x: p.x, y: p.y }
    }
}

impl_op_ex!(+ |a: &Point2f, b: &Point2f| -> Point2f {
    Point2f {
        x: a.x + b.x,
        y: a.y + b.y,
    }
});

impl_op_ex!(+= |a: &mut Point2f, b: &Point2f| {
    a.x += b.x;
    a.y += b.y;
});

impl_op_ex!(-|a: &Point2f, b: &Point2f| -> Vector2f {
    Vector2f {
        x: a.x - b.x,
        y: a.y - b.y,
    }
});

impl_op_ex!(-= |a: &mut Point2f, b: &Vector2f| {
    a.x -= b.x;
    a.y -= b.y;
});

impl_op_ex!(-= |a: &mut Point2f, b: &Point2f| {
    a.x -= b.x;
    a.y -= b.y;
});

impl_op_ex!(*|a: &Point2f, b: Float| -> Point2f {
    Point2f {
        x: a.x * b,
        y: a.y * b,
    }
});

impl_op_ex!(*= |a: &mut Point2f, b: Float| {
    a.x *= b;
    a.y *= b;
});

impl_op_ex!(/ |a: &Point2f, b: Float| -> Point2f {
    let inv = 1.0 as Float / b;
    Point2f {
        x: a.x * inv,
        y: a.y * inv,
    }
});

impl_op_ex!(/= |a: &mut Point2f, b: Float| {
    let inv = 1.0 as Float / b;
    a.x *= inv;
    a.y *= inv;
});

// Point3f

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Point3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Point3f {
    pub fn new(x: Float, y: Float, z: Float) -> Point3f {
        Point3f { x, y, z }
    }

    pub fn has_nans(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    pub fn distance(&self, p: &Point3f) -> Float {
        (*self - *p).length()
    }

    pub fn distance_squared(&self, p: &Point3f) -> Float {
        (*self - *p).length_squared()
    }

    pub fn min(&self, p: &Point3f) -> Point3f {
        Point3f {
            x: self.x.min(p.x),
            y: self.y.min(p.y),
            z: self.z.min(p.z),
        }
    }

    pub fn max(&self, p: &Point3f) -> Point3f {
        Point3f {
            x: self.x.max(p.x),
            y: self.y.max(p.y),
            z: self.z.max(p.z),
        }
    }

    pub fn floor(&self) -> Point3f {
        Point3f {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }

    pub fn ceil(&self) -> Point3f {
        Point3f {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }

    pub fn abs(&self) -> Point3f {
        Point3f {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn permute(&self, x: usize, y: usize, z: usize) -> Point3f {
        Point3f {
            x: self[x],
            y: self[y],
            z: self[z],
        }
    }
}

impl From<Vector3f> for Point3f {
    fn from(v: Vector3f) -> Point3f {
        Point3f {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl Index<usize> for Point3f {
    type Output = Float;

    fn index(&self, i: usize) -> &Float {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Point3f index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Point3f {
    fn index_mut(&mut self, i: usize) -> &mut Float {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Point3f index out of bounds"),
        }
    }
}

impl_op_ex!(+ |a: &Point3f, b: &Point3f| -> Point3f {
    Point3f {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
});

impl_op_ex!(+= |a: &mut Point3f, b: &Point3f| {
    a.x += b.x;
    a.y += b.y;
    a.z += b.z;
});

impl_op_ex!(-|a: &Point3f, b: &Point3f| -> Vector3f {
    Vector3f {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
});

impl_op_ex!(-= |a: &mut Point3f, b: &Point3f| {
    a.x -= b.x;
    a.y -= b.y;
    a.z -= b.z;
});

impl_op_ex!(*|a: &Point3f, b: Float| -> Point3f {
    Point3f {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    }
});

impl_op_ex!(*|a: Float, b: Point3f| -> Point3f {
    Point3f {
        x: a * b.x,
        y: a * b.y,
        z: a * b.z,
    }
});

impl_op_ex!(*= |a: &mut Point3f, b: Float| {
    a.x *= b;
    a.y *= b;
    a.z *= b;
});

impl_op_ex!(/ |a: &Point3f, b: Float| -> Point3f {
    let inv = 1.0 as Float / b;
    Point3f {
        x: a.x * inv,
        y: a.y * inv,
        z: a.z * inv,
    }
});

impl_op_ex!(/= |a: &mut Point3f, b: Float| {
    let inv = 1.0 as Float / b;
    a.x *= inv;
    a.y *= inv;
    a.z *= inv;
});

impl_op_ex!(+= |a: &mut Point3f, b: &Vector3f| {
    a.x += b.x;
    a.y += b.y;
    a.z += b.z;
});

impl_op_ex!(-|a: &Point3f, b: &Vector3f| -> Point3f {
    Point3f {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
});

impl_op_ex!(-= |a: &mut Point3f, b: &Vector3f| {
    a.x -= b.x;
    a.y -= b.y;
    a.z -= b.z;
});

impl_op_ex!(+ |a: &Point2f, b: &Vector2f| -> Point2f {
    Point2f {
        x: a.x + b.x,
        y: a.y + b.y,
    }
});

impl_op_ex!(+= |a: &mut Point2f, b: &Vector2f| {
    a.x += b.x;
    a.y += b.y;
});

pub fn lerp(t: Float, p0: &Point3f, p1: &Point3f) -> Point3f {
    (1.0 as Float - t) * *p0 + t * *p1
}
