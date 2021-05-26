use cgmath::*;

#[derive(Debug, Clone, Copy)]
pub struct LocalTransform {
    pub translation: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: Vector3<f32>,
}

#[allow(dead_code)]
impl LocalTransform {
    pub fn new(translation: Vector3<f32>, rotation: Quaternion<f32>, scale: Vector3<f32>) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }

    pub fn from_translation(translation: Vector3<f32>) -> Self {
        Self::default().with_translation(translation)
    }

    pub fn from_rotation(rotation: Quaternion<f32>) -> Self {
        Self::default().with_rotation(rotation)
    }

    pub fn from_scale(scale: Vector3<f32>) -> Self {
        Self::default().with_scale(scale)
    }

    pub fn from_translation_rotation(translation: Vector3<f32>, rotation: Quaternion<f32>) -> Self {
        Self::default().with_translation(translation).with_rotation(rotation)
    }

    pub fn from_translation_rotation_scale(translation: Vector3<f32>, rotation: Quaternion<f32>, scale: Vector3<f32>) -> Self {
        Self::default().with_translation(translation).with_rotation(rotation).with_scale(scale)
    }

    pub fn look_at(&mut self, position: Vector3<f32>) {
        self.set_rotation(Rotation::look_at((position - self.translation).normalize(), Vector3::unit_y()));
    }

    pub fn with_translation(mut self, translation: Vector3<f32>) -> Self {
        self.set_translation(translation);
        self
    }

    pub fn with_rotation(mut self, rotation: Quaternion<f32>) -> Self {
        self.set_rotation(rotation);
        self
    }

    pub fn with_scale(mut self, scale: Vector3<f32>) -> Self {
        self.set_scale(scale);
        self
    }

    pub fn set_translation(&mut self, translation: Vector3<f32>) {
        self.translation = translation;
    }

    pub fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.rotation = rotation;
    }

    pub fn set_scale(&mut self, scale: Vector3<f32>) {
        self.scale = scale;
    }

    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.translation += translation;
    }

    pub fn rotate(&mut self, rotation: Quaternion<f32>) {
        self.rotation = self.rotation * rotation;
    }

    pub fn resize(&mut self, scale: Vector3<f32>) {
        self.scale.x *= scale.x;
        self.scale.y *= scale.y;
        self.scale.z *= scale.z;
    }

    pub fn matrix(&self) -> Matrix4<f32> {
         Matrix4::from(self.rotation) * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z) * Matrix4::from_translation(self.translation)
    } 
}

impl Default for LocalTransform {
    fn default() -> Self {
        Self {
            translation: Vector3::zero(),
            rotation: Quaternion::one(),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GlobalTransform {
    pub matrix: Matrix4<f32>,
}

#[allow(dead_code)]
impl GlobalTransform {
    pub fn new(matrix: Matrix4<f32>) -> Self {
        Self {
            matrix,
        }
    }

    pub fn from_translation(translation: Vector3<f32>) -> Self {
        Self::new(Matrix4::from_translation(translation))
    }

    pub fn from_rotation(rotation: Quaternion<f32>) -> Self {
        Self::new(Matrix4::from(rotation))
    }

    pub fn from_scale(scale: Vector3<f32>) -> Self {
        Self::new(Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z))
    }

    pub fn from_translation_rotation(translation: Vector3<f32>, rotation: Quaternion<f32>) -> Self {
        Self::from_translation_rotation_scale(translation, rotation, Vector3::new(1.0, 1.0, 1.0))
    }

    pub fn from_translation_rotation_scale(translation: Vector3<f32>, rotation: Quaternion<f32>, scale: Vector3<f32>) -> Self {
        Self::new(Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z) * Matrix4::from(rotation) * Matrix4::from_translation(translation))
    }

    pub fn identity() -> Self {
        Self {
            matrix: Matrix4::identity(),
        }
    }

    pub fn look_at(&mut self, position: Vector3<f32>) {
        self.set_rotation(Rotation::look_at((position - self.translation()).normalize(), Vector3::unit_y()));
    }

    pub fn translation(&self) -> Vector3<f32> {
        Vector4::from(self.matrix.w).truncate()
    }

    pub fn rotation(&self) -> Quaternion<f32> {
        let scale = self.scale();

        let matrix = Matrix3::from_cols(
            Vector3::from(self.matrix.x.truncate()) / scale.x,
            Vector3::from(self.matrix.y.truncate()) / scale.y,
            Vector3::from(self.matrix.z.truncate()) / scale.z,
        );

        Quaternion::from(matrix)
    }

    pub fn scale(&self) -> cgmath::Vector3<f32> {
        cgmath::Vector3::new(
            self.matrix.x.truncate().magnitude(),
            self.matrix.y.truncate().magnitude(),
            self.matrix.z.truncate().magnitude()
        )
    }

    pub fn with_translation(mut self, translation: Vector3<f32>) -> Self {
        self.set_translation(translation);
        self
    }

    pub fn with_rotation(mut self, rotation: Quaternion<f32>) -> Self {
        self.set_rotation(rotation);
        self
    }

    pub fn with_scale(mut self, scale: Vector3<f32>) -> Self {
        self.set_scale(scale);
        self
    }

    pub fn set_translation(&mut self, translation: Vector3<f32>) {
        self.matrix.w = translation.extend(1.0);
    }

    pub fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.matrix = Self::from_translation_rotation_scale(self.translation(), rotation, self.scale()).matrix;
    }

    pub fn set_scale(&mut self, scale: Vector3<f32>) {
        self.matrix = Self::from_translation_rotation_scale(self.translation(), self.rotation(), scale).matrix;
    }

    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.matrix.w += translation.extend(0.0);
    }

    pub fn rotate(&mut self, rotation: Quaternion<f32>) {
        self.matrix = Matrix4::from(rotation) * self.matrix;
    }

    pub fn resize(&mut self, scale: Vector3<f32>) {
        self.matrix = Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z) * self.matrix;
    }
}

impl Default for GlobalTransform {
    fn default() -> Self {
        Self::identity()
    }
}