/// A trait to represent valid Aliasing levels.
///
/// This trait is sealed and can't be implemented for any other type.
pub trait AliasLevel<const N: usize>: private::Sealed {
    /// Subpixel integer offsets for the current AA level.
    /// Copied from `D3D11_STANDARD_MULTISAMPLE_QUALITY_LEVELS`.
    const OFFSETS: [(i32, i32); N];

    const MAX: u32 = 8;
}

impl AliasLevel<1> for () {
    #[rustfmt::skip]
    const OFFSETS: [(i32, i32); 1] = [
        ( 0, 0)
    ];
}
impl AliasLevel<2> for () {
    #[rustfmt::skip]
    const OFFSETS: [(i32, i32); 2] = [
        ( 4, 4),(-4,-4)
    ];
}
impl AliasLevel<4> for () {
    #[rustfmt::skip]
    const OFFSETS: [(i32, i32); 4] = [
        (-2,-6),( 6,-2),(-6, 2),(2, 6)
    ];
}
impl AliasLevel<8> for () {
    #[rustfmt::skip]
    const OFFSETS: [(i32, i32); 8] = [
        ( 1, 3),(-1, 3),( 5, 1),(-3,-5),
        (-5, 5),(-7,-1),( 3, 7),( 7,-7)
    ];
}
impl AliasLevel<16> for () {
    #[rustfmt::skip]
    const OFFSETS: [(i32, i32); 16] = [
        ( 1, 1),(-1,-3),(-3, 2),( 4,-1),
        (-5,-2),( 2, 5),( 5, 3),( 3,-5),
        (-2, 6),( 0,-7),(-4,-6),(-6, 4),
        (-8, 0),( 7,-4),( 6, 7),(-7,-8),
    ];
}

mod private {
    pub trait Sealed {}
    impl Sealed for () {}
}

use uv::{f32x16, Vec2x16};

use crate::Splat;

pub(crate) fn get_offsets<const AA: usize>(height: u32) -> [Vec2x16; AA]
where
    (): AliasLevel<AA>,
{
    let k = <()>::MAX * height;
    let k = f32x16::splat(k as f32);

    let mut arr = [Vec2x16::broadcast((0.0).splat()); AA];
    for (offset, (x, y)) in arr.iter_mut().zip(<()>::OFFSETS) {
        let p = Vec2x16::new(f32x16::splat(x as f32), f32x16::splat(y as f32));
        *offset = p / k;
    }
    arr
}
