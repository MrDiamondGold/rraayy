use crate::{ray::Ray, shape::Shape, vector::SteppedVector};

#[derive(Clone, Copy)]
pub struct BoxShape(SteppedVector, SteppedVector);

#[allow(dead_code)]
impl BoxShape {
    pub fn new(start: SteppedVector, end: SteppedVector) -> Self {
        Self (
            start,
            end,
        )
    }

    pub fn start(&self) -> SteppedVector {
        self.0
    }

    pub fn end(&self) -> SteppedVector {
        self.1
    }
}

impl Shape for BoxShape {
    fn intersects_ray(&self, ray: &Ray) -> (bool, f32) {
        let t_min = -f32::INFINITY;
        let t_max = f32::INFINITY;

        let t_x1 = (self.0.x as f32 - ray.origin().x) * ray.inv_dir().x;
        let t_x2 = (self.1.x as f32 - ray.origin().x) * ray.inv_dir().x;

        let t_min = t_min.max(t_x1.min(t_x2));
        let t_max = t_max.min(t_x1.max(t_x2));

        let t_y1 = (self.0.y as f32 - ray.origin().y) * ray.inv_dir().y;
        let t_y2 = (self.1.y as f32 - ray.origin().y) * ray.inv_dir().y;

        let t_min = t_min.max(t_y1.min(t_y2));
        let t_max = t_max.min(t_y1.max(t_y2));

        let t_z1 = (self.0.z as f32 - ray.origin().z) * ray.inv_dir().z;
        let t_z2 = (self.1.z as f32 - ray.origin().z) * ray.inv_dir().z;
    
        let t_min = t_min.max(t_z1.min(t_z2));
        let t_max = t_max.min(t_z1.max(t_z2));

        (
            t_max >= t_min && t_max >= 0.0,
            t_min,
        )
    }
}