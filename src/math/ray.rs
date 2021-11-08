use super::{Fma, Vec3s, f32s, Ray};

impl Ray {
    #[inline]
    pub fn new(origin: Vec3s, dir: Vec3s) -> Self {
        Self { origin, dir }
    }

    #[inline]
    pub fn at(&self, t: f32s) -> Vec3s {
        // self.origin + self.dir * t
        t.fma(self.dir, self.origin)
    }
}
