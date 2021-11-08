use super::*;

/// Fused Multiply Add operation.
pub(crate) trait Fma<M = Self, A = Self> {
    type Output;

    /// Computes `self * m + a` in a single instruction.
    fn fma(&self, m: M, a: A) -> Self::Output;
}

impl Fma for f32s {
    type Output = Self;

    #[inline]
    fn fma(&self, m: Self, a: Self) -> Self::Output {
        self.mul_add(m, a)
    }
}

impl Fma<Vec3s, Vec3s> for f32s {
    type Output = Vec3s;

    #[inline]
    fn fma(&self, m: Vec3s, a: Vec3s) -> Self::Output {
        Vec3s::new(
            self.mul_add(m.x, a.x),
            self.mul_add(m.y, a.y),
            self.mul_add(m.z, a.z),
        )
    }
}

impl Fma<f32s, Vec3s> for Vec3s {
    type Output = Vec3s;

    #[inline]
    fn fma(&self, m: f32s, a: Vec3s) -> Self::Output {
        Vec3s::new(
            self.x.mul_add(m, a.x),
            self.y.mul_add(m, a.y),
            self.z.mul_add(m, a.z),
        )
    }
}
