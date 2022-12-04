use std::{cell::Cell, sync::Arc};

use crate::core::{medium::Medium, pbrt::Float};

use super::{point::Point3f, vector::Vector3f};

// Ray
#[derive(Default, Clone)]
pub struct Ray {
    pub o: Point3f,
    pub d: Vector3f,
    pub t_max: Cell<Float>,
    pub time: Float,
    pub medium: Option<Arc<Medium>>,
    pub differential: Option<RayDifferential>,
}

impl Ray {
    pub fn has_nans(&self) -> bool {
        self.o.has_nans() || self.d.has_nans() || self.time.is_nan()
    }

    pub fn position(&self, t: Float) -> Point3f {
        self.o + self.d * t
    }

    pub fn scale_differentials(&mut self, s: Float) {
        if let Some(ref mut diff) = self.differential {
            diff.rx_origin = self.o + (diff.rx_origin - self.o) * s;
            diff.ry_origin = self.o + (diff.ry_origin - self.o) * s;
            diff.rx_direction = self.d + (diff.rx_direction - self.d) * s;
            diff.ry_direction = self.d + (diff.ry_direction - self.d) * s;
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct RayDifferential {
    pub rx_origin: Point3f,
    pub ry_origin: Point3f,
    pub rx_direction: Vector3f,
    pub ry_direction: Vector3f,
}
