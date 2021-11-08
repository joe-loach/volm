use super::*;

impl Vec3s {
    pub const ZERO: Self = Self::splat(f32s::splat(0.0));
    pub const ONE: Self = Self::splat(f32s::splat(1.0));

    /// Creates a new [`Vec3s`].
    #[inline]
    pub const fn new(x: f32s, y: f32s, z: f32s) -> Self {
        Self { x, y, z }
    }

    /// Splats a single value across all 3 components.
    #[inline]
    pub const fn splat(val: f32s) -> Self {
        Self::new(val, val, val)
    }

    /// Widens each non-vector component for a shorthand constructor.
    #[inline]
    pub const fn widen(x: f32, y: f32, z: f32) -> Self {
        Self::new(f32s::splat(x), f32s::splat(y), f32s::splat(z))
    }

    /// Dot product of two vectors.
    #[inline]
    pub fn dot(&self, other: &Self) -> f32s {
        //self.x * other.x + self.y * other.y + self.z * other.z
        let mut acc = self.x * self.x;
        acc = self.y.mul_add(other.y, acc);
        acc = self.z.mul_add(other.z, acc);
        acc
    }

    /// Cross product of two vectors.
    #[inline]
    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Squared magnitude of the vector.
    #[inline]
    pub fn mag_sq(&self) -> f32s {
        self.dot(self)
    }

    /// Magnitude of the vector.
    #[inline]
    pub fn mag(&self) -> f32s {
        self.mag_sq().sqrt()
    }

    /// Normalise the vector.
    #[inline]
    pub fn normalize(&mut self) {
        let rmag = self.mag().recip();
        self.x *= rmag;
        self.y *= rmag;
        self.z *= rmag;
    }

    /// Normalise and return a new Vector.
    #[inline]
    #[must_use = "Did you mean to use `normalize()` to normalize `self` in place?"]
    pub fn normalized(&self) -> Self {
        let mut v = *self;
        v.normalize();
        v
    }

    #[inline]
    pub fn min(&self, val: f32s) -> Vec3s {
        Vec3s::new(
            self.x.max(val),
            self.y.max(val),
            self.z.max(val)
        )
    }

    #[inline]
    pub fn max(&self, val: f32s) -> Vec3s {
        Vec3s::new(
            self.x.max(val),
            self.y.max(val),
            self.z.max(val)
        )
    }

    #[inline]
    pub fn min_comp(&self) -> f32s {
        self.reduce(f32s::min)
    }

    #[inline]
    pub fn max_comp(&self) -> f32s {
        self.reduce(f32s::max)
    }

    /// Restrict each component in between the range of min and max.
    #[inline]
    pub fn clamp(&mut self, min: f32s, max: f32s) {
        self.x = self.x.clamp(min, max);
        self.y = self.y.clamp(min, max);
        self.z = self.z.clamp(min, max);
    }

    /// Clamp and return a new Vector.
    #[inline]
    pub fn clamped(&self, min: f32s, max: f32s) -> Self {
        let mut v = *self;
        v.clamp(min, max);
        v
    }

    #[inline]
    pub fn abs(self) -> Self {
        self.map(f32s::abs)
    }

    #[inline]
    pub fn sqrt(self) -> Self {
        self.map(f32s::sqrt)
    }

    #[inline]
    pub fn map<F: Fn(f32s) -> f32s>(self, op: F) -> Self {
        let Self { x, y, z } = self;
        Self::new(op(x), op(y), op(z))
    }

    #[inline]
    pub fn apply<F: Fn(f32s) -> f32s>(&mut self, op: F) {
        self.x = op(self.x);
        self.y = op(self.y);
        self.z = op(self.z);
    }

    #[inline]
    pub fn fold<F: FnMut(f32s, f32s) -> f32s>(&self, init: f32s, mut f: F) -> f32s {
        let mut accum = init;
        accum = f(accum, self.x);
        accum = f(accum, self.y);
        accum = f(accum, self.z);
        accum
    }

    #[inline]
    pub fn reduce<F: FnMut(f32s, f32s) -> f32s>(&self, mut f: F) -> f32s {
        let mut accum = self.x;
        accum = f(accum, self.y);
        accum = f(accum, self.z);
        accum
    }
}

mod ops {
    use super::*;
    use core::ops::*;

    impl Add for Vec3s {
        type Output = Self;

        #[inline]
        fn add(self, rhs: Self) -> Self::Output {
            Vec3s::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
        }
    }

    impl Add<f32s> for Vec3s {
        type Output = Self;

        #[inline]
        fn add(self, rhs: f32s) -> Self::Output {
            Vec3s::new(self.x + rhs, self.y + rhs, self.z + rhs)
        }
    }

    impl AddAssign for Vec3s {
        #[inline]
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
            self.z += rhs.z;
        }
    }

    impl AddAssign<f32s> for Vec3s {
        #[inline]
        fn add_assign(&mut self, rhs: f32s) {
            self.x += rhs;
            self.y += rhs;
            self.z += rhs;
        }
    }

    impl Sub for Vec3s {
        type Output = Self;

        #[inline]
        fn sub(self, rhs: Self) -> Self::Output {
            Vec3s::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
        }
    }

    impl Sub<f32s> for Vec3s {
        type Output = Self;

        #[inline]
        fn sub(self, rhs: f32s) -> Self::Output {
            Vec3s::new(self.x - rhs, self.y - rhs, self.z - rhs)
        }
    }

    impl SubAssign for Vec3s {
        #[inline]
        fn sub_assign(&mut self, rhs: Self) {
            self.x -= rhs.x;
            self.y -= rhs.y;
            self.z -= rhs.z;
        }
    }

    impl SubAssign<f32s> for Vec3s {
        #[inline]
        fn sub_assign(&mut self, rhs: f32s) {
            self.x -= rhs;
            self.y -= rhs;
            self.z -= rhs;
        }
    }

    impl Mul for Vec3s {
        type Output = Self;

        #[inline]
        fn mul(self, rhs: Self) -> Self::Output {
            Vec3s::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
        }
    }

    impl Mul<f32s> for Vec3s {
        type Output = Self;

        #[inline]
        fn mul(self, rhs: f32s) -> Self::Output {
            Vec3s::new(self.x * rhs, self.y * rhs, self.z * rhs)
        }
    }

    impl MulAssign for Vec3s {
        #[inline]
        fn mul_assign(&mut self, rhs: Self) {
            self.x *= rhs.x;
            self.y *= rhs.y;
            self.z *= rhs.z;
        }
    }

    impl MulAssign<f32s> for Vec3s {
        #[inline]
        fn mul_assign(&mut self, rhs: f32s) {
            self.x *= rhs;
            self.y *= rhs;
            self.z *= rhs;
        }
    }

    impl Div for Vec3s {
        type Output = Self;

        #[inline]
        fn div(self, rhs: Self) -> Self::Output {
            Vec3s::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
        }
    }

    impl Div<f32s> for Vec3s {
        type Output = Self;

        #[inline]
        fn div(self, rhs: f32s) -> Self::Output {
            Vec3s::new(self.x / rhs, self.y / rhs, self.z / rhs)
        }
    }

    impl DivAssign for Vec3s {
        #[inline]
        fn div_assign(&mut self, rhs: Self) {
            self.x /= rhs.x;
            self.y /= rhs.y;
            self.z /= rhs.z;
        }
    }

    impl DivAssign<f32s> for Vec3s {
        #[inline]
        fn div_assign(&mut self, rhs: f32s) {
            self.x /= rhs;
            self.y /= rhs;
            self.z /= rhs;
        }
    }

    impl Neg for Vec3s {
        type Output = Self;

        #[inline]
        fn neg(self) -> Self::Output {
            Vec3s::new(-self.x, -self.y, -self.z)
        }
    }
}
