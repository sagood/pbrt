use std::ops::{Index, IndexMut};

use crate::core::pbrt::Float;

use super::{
    misc::{point2f_inside_bounds2f, point3f_inside_bounds3f},
    point::{Point2f, Point3f},
    ray::Ray,
    vector::{Vector2f, Vector3f},
};

// Bounds2f
#[derive(Debug, Copy, Clone)]
pub struct Bounds2f {
    pub p_min: Point2f,
    pub p_max: Point2f,
}

impl Bounds2f {
    pub fn new_with_points(p1: Point2f, p2: Point2f) -> Bounds2f {
        Bounds2f {
            p_min: Point2f {
                x: p1.x.min(p2.x),
                y: p1.y.min(p2.y),
            },
            p_max: Point2f {
                x: p1.x.max(p2.x),
                y: p1.y.max(p2.y),
            },
        }
    }

    pub fn diagonal(&self) -> Vector2f {
        self.p_max - self.p_min
    }

    pub fn area(&self) -> Float {
        let d = self.p_max - self.p_min;
        d.x * d.y
    }

    pub fn maximum_extend(&self) -> usize {
        let d = self.diagonal();
        if d.x > d.y {
            0
        } else {
            1
        }
    }

    pub fn lerp(&self, t: &Point2f) -> Point2f {
        Point2f {
            x: super::super::pbrt::lerp(t.x, self.p_min.x, self.p_max.x),
            y: super::super::pbrt::lerp(t.y, self.p_min.y, self.p_max.y),
        }
    }

    pub fn offset(&self, p: &Point2f) -> Vector2f {
        let mut o = *p - self.p_min;
        if self.p_max.x > self.p_min.x {
            o.x /= self.p_max.x - self.p_min.x;
        }
        if self.p_max.y > self.p_min.y {
            o.y /= self.p_max.y - self.p_min.y;
        }
        o
    }

    pub fn bounding_sphere(&self) -> (Point2f, Float) {
        let center = (self.p_min + self.p_max) * 0.5;
        let radius = if point2f_inside_bounds2f(&center, &self) {
            center.distance(&self.p_max)
        } else {
            0.0
        };

        (center, radius)
    }
}

impl Default for Bounds2f {
    fn default() -> Bounds2f {
        let min_num = Float::MIN;
        let max_num = Float::MAX;
        Bounds2f {
            p_min: Point2f::new(max_num, max_num),
            p_max: Point2f::new(min_num, min_num),
        }
    }
}

impl From<Point2f> for Bounds2f {
    fn from(p: Point2f) -> Bounds2f {
        Bounds2f { p_min: p, p_max: p }
    }
}

impl Index<usize> for Bounds2f {
    type Output = Point2f;

    fn index(&self, i: usize) -> &Point2f {
        match i {
            0 => &self.p_min,
            1 => &self.p_max,
            _ => panic!("Bounds2f index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Bounds2f {
    fn index_mut(&mut self, i: usize) -> &mut Point2f {
        match i {
            0 => &mut self.p_min,
            1 => &mut self.p_max,
            _ => panic!("Bounds2f index out of bounds"),
        }
    }
}

// Bounds3f
#[derive(Debug, Copy, Clone)]
pub struct Bounds3f {
    pub p_min: Point3f,
    pub p_max: Point3f,
}

impl Bounds3f {
    pub fn new_with_points(p1: Point3f, p2: Point3f) -> Bounds3f {
        Bounds3f {
            p_min: Point3f {
                x: p1.x.min(p2.x),
                y: p1.y.min(p2.y),
                z: p1.z.min(p2.z),
            },
            p_max: Point3f {
                x: p1.x.max(p2.x),
                y: p1.y.max(p2.y),
                z: p1.z.max(p2.z),
            },
        }
    }

    pub fn corner(&self, corner: usize) -> Point3f {
        let index_x = corner & 1;
        let index_y = if corner & 2 == 0 { 0 } else { 1 };
        let index_z = if corner & 4 == 0 { 0 } else { 1 };
        Point3f {
            x: self[index_x].x,
            y: self[index_y].y,
            z: self[index_z].z,
        }
    }

    pub fn union_point3f(&self, p: &Point3f) -> Bounds3f {
        Bounds3f {
            p_min: Point3f {
                x: self.p_min.x.min(p.x),
                y: self.p_min.y.min(p.y),
                z: self.p_min.z.min(p.z),
            },
            p_max: Point3f {
                x: self.p_max.x.max(p.x),
                y: self.p_max.y.max(p.y),
                z: self.p_max.z.max(p.z),
            },
        }
    }

    pub fn union(&self, b: &Bounds3f) -> Bounds3f {
        Bounds3f {
            p_min: Point3f {
                x: self.p_min.x.min(b.p_min.x),
                y: self.p_min.y.min(b.p_min.y),
                z: self.p_min.z.min(b.p_min.z),
            },
            p_max: Point3f {
                x: self.p_max.x.max(b.p_max.x),
                y: self.p_max.y.max(b.p_max.y),
                z: self.p_max.z.max(b.p_max.z),
            },
        }
    }

    pub fn diagonal(&self) -> Vector3f {
        self.p_max - self.p_min
    }

    pub fn surface_area(&self) -> Float {
        let d = self.diagonal();
        2.0 * (d.x * d.y + d.x * d.z + d.y * d.z)
    }

    pub fn volume(&self) -> Float {
        let d = self.diagonal();
        d.x * d.y * d.z
    }

    pub fn maximum_extend(&self) -> usize {
        let d = self.diagonal();
        if d.x > d.y && d.x > d.z {
            0
        } else if d.y > d.z {
            1
        } else {
            2
        }
    }

    pub fn lerp(&self, t: &Point3f) -> Point3f {
        Point3f {
            x: super::super::pbrt::lerp(t.x, self.p_min.x, self.p_max.x),
            y: super::super::pbrt::lerp(t.y, self.p_min.y, self.p_max.y),
            z: super::super::pbrt::lerp(t.z, self.p_min.z, self.p_max.z),
        }
    }

    pub fn offset(&self, p: &Point3f) -> Vector3f {
        let mut o = *p - self.p_min;
        if self.p_max.x > self.p_min.x {
            o.x /= self.p_max.x - self.p_min.x;
        }
        if self.p_max.y > self.p_min.y {
            o.y /= self.p_max.y - self.p_min.y;
        }
        if self.p_max.z > self.p_min.z {
            o.z /= self.p_max.z - self.p_min.z;
        }
        o
    }

    pub fn bounding_sphere(&self) -> (Point3f, Float) {
        let center = (self.p_min + self.p_max) * 0.5;
        let radius = if point3f_inside_bounds3f(&center, &self) {
            center.distance(&self.p_max)
        } else {
            0.0
        };

        (center, radius)
    }

    pub fn intersect(&self, b: &Bounds3f) -> Bounds3f {
        Bounds3f {
            p_min: Point3f {
                x: self.p_min.x.max(b.p_min.x),
                y: self.p_min.y.max(b.p_min.y),
                z: self.p_min.z.max(b.p_min.z),
            },
            p_max: Point3f {
                x: self.p_max.x.min(b.p_max.x),
                y: self.p_max.y.min(b.p_max.y),
                z: self.p_max.z.min(b.p_max.z),
            },
        }
    }

    pub fn overlaps(&self, b: &Bounds3f) -> bool {
        let x = (self.p_max.x >= b.p_min.x) && (self.p_min.x <= b.p_max.x);
        let y = (self.p_max.y >= b.p_min.y) && (self.p_min.y <= b.p_max.y);
        let z = (self.p_max.z >= b.p_min.z) && (self.p_min.z <= b.p_max.z);
        x && y && z
    }

    pub fn expand(&self, delta: Float) -> Bounds3f {
        Bounds3f {
            p_min: self.p_min - Vector3f::new(delta, delta, delta),
            p_max: self.p_max + Vector3f::new(delta, delta, delta),
        }
    }

    pub fn intersect_b(&self, ray: &Ray, hitt0: &mut Float, hitt1: &mut Float) -> bool {
        unimplemented!()
    }

    pub fn intersect_p(&self, ray: &Ray, inv_dir: &Vector3f, dir_is_neg: &[u8; 3]) -> bool {
        unimplemented!()
    }
}

impl Default for Bounds3f {
    fn default() -> Bounds3f {
        let min_num = Float::MIN;
        let max_num = Float::MAX;
        Bounds3f {
            p_min: Point3f::new(max_num, max_num, max_num),
            p_max: Point3f::new(min_num, min_num, min_num),
        }
    }
}

impl From<Point3f> for Bounds3f {
    fn from(p: Point3f) -> Bounds3f {
        Bounds3f { p_min: p, p_max: p }
    }
}

impl Index<usize> for Bounds3f {
    type Output = Point3f;

    fn index(&self, i: usize) -> &Point3f {
        match i {
            0 => &self.p_min,
            1 => &self.p_max,
            _ => panic!("Bounds3f index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Bounds3f {
    fn index_mut(&mut self, i: usize) -> &mut Point3f {
        match i {
            0 => &mut self.p_min,
            1 => &mut self.p_max,
            _ => panic!("Bounds3f index out of bounds"),
        }
    }
}
