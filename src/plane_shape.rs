use cgmath::InnerSpace;

use crate::{ray::Ray, shape::Shape, vector::Vector};

#[derive(Clone, Copy)]
pub struct PlaneShape {
    origin: Vector,
    dir: Vector,
}

#[allow(dead_code)]
impl PlaneShape {
    pub fn new(origin: Vector, dir: Vector) -> Self {
        Self {
            origin,
            dir,
        }
    }

    pub fn origin(&self) -> Vector {
        self.origin
    }

    pub fn dir(&self) -> Vector {
        self.dir
    }
}

impl Shape for PlaneShape {
    fn intersects_ray(&self, ray: &Ray) -> (bool, f32) {
        let denom = self.dir.dot(ray.dir());
        if denom >= 1e-6 {
            let centered = self.origin - ray.origin();
            let t = centered.dot(self.dir) / denom;

            return (t >= 0.0, t);
        }
        
        return (false, 0.0);
    }
}