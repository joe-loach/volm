use super::f32s;

/// A type that can be splatted into a vector.
pub trait Splattable {
    type Output;

    fn splat(self) -> Self::Output;
}

impl Splattable for f32 {
    type Output = f32s;

    #[inline]
    fn splat(self) -> Self::Output {
        f32s::splat(self)
    }
}

