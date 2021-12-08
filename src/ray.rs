pub struct Ray {
    origin: uv::Vec3x8,
    dir: uv::Vec3x8,
}

impl Ray {
    #[inline]
    pub fn new(origin: uv::Vec3x8, dir: uv::Vec3x8) -> Self {
        Self { origin, dir }
    }

    #[inline]
    pub fn at(&self, t: uv::f32x8) -> uv::Vec3x8 {
        // self.origin + self.dir * t
        self.dir.mul_add(uv::Vec3x8::broadcast(t), self.origin)
    }
}
