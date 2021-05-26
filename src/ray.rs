use crate::vector::Vector;

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Vector,
    dir: Vector,
    inv_dir: Vector,
    x_sign: bool,
    y_sign: bool,
    z_sign: bool,
}

#[allow(dead_code)]
impl Ray {
    pub fn new(origin: Vector, dir: Vector) -> Self {
        let inv_dir = 1.0 / dir;
        
        Self {
            origin,
            dir,
            inv_dir,
            x_sign: inv_dir.x < 0.0,
            y_sign: inv_dir.y < 0.0,
            z_sign: inv_dir.z < 0.0,
        }
    }

    pub fn origin(&self) -> Vector {
        self.origin
    }

    pub fn dir(&self) -> Vector {
        self.dir
    }

    pub fn inv_dir(&self) -> Vector {
        self.inv_dir
    }

    pub fn x_sign(&self) -> bool {
        self.x_sign
    }

    pub fn y_sign(&self) -> bool {
        self.y_sign
    }

    pub fn z_sign(&self) -> bool {
        self.z_sign
    }
}