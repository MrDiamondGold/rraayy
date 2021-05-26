use crate::ray::Ray;



pub trait Shape: Send + Sync {
    fn intersects_ray(&self, ray: &Ray) -> (bool, f32);
}