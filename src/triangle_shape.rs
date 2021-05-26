use cgmath::InnerSpace;

use crate::{plane_shape::PlaneShape, ray::Ray, shape::Shape, vector::Vector};


#[derive(Clone, Copy)]
pub struct TriangleShape {
    a: Vector,
    b: Vector,
    c: Vector,
}

#[allow(dead_code)]
impl TriangleShape {
    pub fn new(a: Vector, b: Vector, c: Vector) -> Self {
        Self {
            a,
            b,
            c,
        }
    }
}

impl Shape for TriangleShape {
    fn intersects_ray(&self, ray: &Ray) -> (bool, f32) {
        let a = self.b - self.a;
        let b = self.c - self.a;
        let normal = a.cross(b);

        let origin = self.a;
        let dir = normal.normalize();
        let plane = PlaneShape::new(origin, dir);
        
        let (result, t) = PlaneShape::intersects_ray(&plane, ray);
        if result {
            if t < 0.0 {
                return (false, 0.0);
            }

            let p = ray.origin() + ray.dir() * t;

            let e = self.b - self.a;
            let v = p - self.a;
            let c = e.cross(v);
            if normal.dot(c) < 0.0 {
                return (false, 0.0);
            }

            let e = self.c - self.b;
            let v = p - self.b;
            let c = e.cross(v);
            if normal.dot(c) < 0.0 {
                return (false, 0.0);
            }

            let e = self.a - self.c;
            let v = p - self.c;
            let c = e.cross(v);
            if normal.dot(c) < 0.0 {
                return (false, 0.0);
            }

            return (true, t);
        }

        return (false, 0.0);
    }
}