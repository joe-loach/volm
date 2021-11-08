use super::*;

impl Vec2s {
    pub const ZERO: Self = Self::splat(f32s::splat(0.0));
    pub const ONE: Self = Self::splat(f32s::splat(0.0));

    #[inline]
    pub const fn new(x: f32s, y: f32s) -> Self {
        Self { x, y }
    }

    /// Splats a single value across all 3 components.
    #[inline]
    pub const fn splat(val: f32s) -> Self {
        Self::new(val, val)
    }
}

mod ops {
    use super::*;
    use core::ops::*;

    impl Add for Vec2s {
        type Output = Self;

        #[inline]
        fn add(self, rhs: Self) -> Self::Output {
            Vec2s::new(self.x + rhs.x, self.y + rhs.y)
        }
    }

    impl Add<f32s> for Vec2s {
        type Output = Self;

        #[inline]
        fn add(self, rhs: f32s) -> Self::Output {
            Vec2s::new(self.x + rhs, self.y + rhs)
        }
    }

    impl AddAssign for Vec2s {
        #[inline]
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }

    impl AddAssign<f32s> for Vec2s {
        #[inline]
        fn add_assign(&mut self, rhs: f32s) {
            self.x += rhs;
            self.y += rhs;
        }
    }

    impl Sub for Vec2s {
        type Output = Self;

        #[inline]
        fn sub(self, rhs: Self) -> Self::Output {
            Vec2s::new(self.x - rhs.x, self.y - rhs.y)
        }
    }

    impl Sub<f32s> for Vec2s {
        type Output = Self;

        #[inline]
        fn sub(self, rhs: f32s) -> Self::Output {
            Vec2s::new(self.x - rhs, self.y - rhs)
        }
    }

    impl SubAssign for Vec2s {
        #[inline]
        fn sub_assign(&mut self, rhs: Self) {
            self.x -= rhs.x;
            self.y -= rhs.y;
        }
    }

    impl SubAssign<f32s> for Vec2s {
        #[inline]
        fn sub_assign(&mut self, rhs: f32s) {
            self.x -= rhs;
            self.y -= rhs;
        }
    }

    impl Mul for Vec2s {
        type Output = Self;

        #[inline]
        fn mul(self, rhs: Self) -> Self::Output {
            Vec2s::new(self.x * rhs.x, self.y * rhs.y)
        }
    }

    impl Mul<f32s> for Vec2s {
        type Output = Self;

        #[inline]
        fn mul(self, rhs: f32s) -> Self::Output {
            Vec2s::new(self.x * rhs, self.y * rhs)
        }
    }

    impl MulAssign for Vec2s {
        #[inline]
        fn mul_assign(&mut self, rhs: Self) {
            self.x *= rhs.x;
            self.y *= rhs.y;
        }
    }

    impl MulAssign<f32s> for Vec2s {
        #[inline]
        fn mul_assign(&mut self, rhs: f32s) {
            self.x *= rhs;
            self.y *= rhs;
        }
    }

    impl Div for Vec2s {
        type Output = Self;

        #[inline]
        fn div(self, rhs: Self) -> Self::Output {
            Vec2s::new(self.x / rhs.x, self.y / rhs.y)
        }
    }

    impl Div<f32s> for Vec2s {
        type Output = Self;

        #[inline]
        fn div(self, rhs: f32s) -> Self::Output {
            Vec2s::new(self.x / rhs, self.y / rhs)
        }
    }

    impl DivAssign for Vec2s {
        #[inline]
        fn div_assign(&mut self, rhs: Self) {
            self.x /= rhs.x;
            self.y /= rhs.y;
        }
    }

    impl DivAssign<f32s> for Vec2s {
        #[inline]
        fn div_assign(&mut self, rhs: f32s) {
            self.x /= rhs;
            self.y /= rhs;
        }
    }

    impl Neg for Vec2s {
        type Output = Self;

        #[inline]
        fn neg(self) -> Self::Output {
            Vec2s::new(-self.x, -self.y)
        }
    }
}
