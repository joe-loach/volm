use rayon::prelude::*;
use uv::simd::*;
use uv::Vec2x16;
use uv::Vec3x16;

use crate::aa::AliasLevel;
use crate::camera::Camera;
use crate::ray::Ray;

/// The main context used to render an image.
pub struct Pixels<const AA: usize> {
    resolution: Vec2x16,
    uvs: Vec<Vec2x16>,
    offsets: [Vec2x16; AA],
}

impl<const AA: usize> Pixels<AA>
where
    (): AliasLevel<AA>,
{
    pub fn resize(&mut self, width: u32, height: u32) {
        use core::mem;
        const LANES: usize = 16;

        const INC: u32x16 = {
            let mut arr = [0_u32; LANES];
            let mut i = 0;
            while i < LANES {
                arr[i] = i as u32;
                i += 1;
            }
            unsafe { mem::transmute(arr) }
        };

        self.resolution = Vec2x16::new(f32x16::splat(width as f32), f32x16::splat(height as f32));
        self.offsets = crate::aa::get_offsets(height);
        self.uvs = (0..width * height)
            .into_par_iter()
            .step_by(LANES)
            .map(|i| {
                // convert to cartesian coordinates
                let x = i % width;
                let y = i / width;
                // vectorize the coordinates
                let y = f32x16::splat(y as f32); // all y values are the same
                let x = u32x16::splat(x) + INC; // add INC to produce [x+0,x+1,...,x+7]
                let x = unsafe { mem::transmute(x) }; // convert to i32x8 (noop)
                let pos = Vec2x16::new(f32x16::round_from_int(x), y); // convert the i32 vector to a f32 vector
                ((pos * f32x16::splat(2.0)) - self.resolution) / self.resolution.y
                // return a normalised uv
            })
            .collect();
    }

    pub fn render<F, C>(&self, camera: &C, map: F) -> Vec<u8>
    where
        C: Camera,
        F: Fn(Ray) -> Vec3x16 + Sync + Send,
    {
        use core::mem;

        const DIMENSIONS: usize = 3;
        const BITS: usize = 16 * DIMENSIONS;

        let v: Vec<[u8; BITS]> = self
            .uvs
            .par_iter()
            .map(|uv| {
                let col = if AA <= 1 {
                    // Don't bother aliasing if AA = 1,
                    // we only need to take one sample.
                    map(camera.ray_at(*uv))
                } else {
                    // AA samples > 1,
                    // take multiple samples "inside" a pixel,
                    let mut total = Vec3x16::broadcast(f32x16::splat(0.0));
                    for offset in self.offsets {
                        total += map(camera.ray_at(*uv + offset));
                    }
                    // average out the color
                    total / f32x16::splat(AA as f32)
                };
                // gamma correction
                //col.powf(f32x16::splat(0.4545))
                col.map(|c| c.sqrt()) // sqrt is x^0.5 which is close enough and speeds things up
            })
            // convert from Wec3 -> [u8;BITS]
            .map(|ref mut col| {
                // assume each value in `col` is in the domain [0.0, 1.0]
                // we need to convert this to [0.0, 255.0],
                // so multiply each lane by 255.0.
                *col *= f32x16::splat(255.0);
                // we don't need to explicitly clamp each lane to [0.0, 1.0].
                // if any value is outside the domain (eg. 1.4 ->(*255)-> 357 )
                // then the conversion to `u8` later will normalise it to [0,255].

                // destruct the vector
                let Vec3x16 { x, y, z } = col;
                // make an array of 0s
                let mut bits = [0u8; BITS];
                // x,y,z components from the same lane are grouped
                for (idx, c) in bits.chunks_exact_mut(DIMENSIONS).enumerate() {
                    assert!(c.len() == 3);
                    c[0] = x[idx] as u8;
                    c[1] = y[idx] as u8;
                    c[2] = z[idx] as u8;
                }
                bits
            })
            .collect();

        // convert from Vec<[u8;BITS]> -> Vec<u8>
        // SAFETY:
        // * We dont call the destructor for the vector twice.
        // * The data has already allocated by the iterator.
        // * The alignment of u8 and [u8;BITS] are the same.
        // * The length and capacity are modified to respect the extra items.
        let mut v = mem::ManuallyDrop::new(v);
        unsafe {
            let (ptr, len, cap) = (v.as_mut_ptr(), v.len(), v.capacity());
            Vec::from_raw_parts(ptr.cast(), len * BITS, cap * BITS)
        }
    }
}

/// Builder struct for [`Pixels`].
pub struct PixelsBuilder {
    width: u32,
    height: u32,
}

impl PixelsBuilder {
    /// Creates a new [`PixelsBuilder`] with a resolution of 1080p.
    ///
    /// This is the same as calling [`Self::default`].
    #[inline]
    pub fn new() -> PixelsBuilder {
        Self::default()
    }

    pub fn with_width(self, width: u32) -> PixelsBuilder {
        Self { width, ..self }
    }

    pub fn with_height(self, height: u32) -> PixelsBuilder {
        Self { height, ..self }
    }

    /// Consumes the builder and creates the [`Pixels`] context.
    pub fn build_with_aa<const AA: usize>(self) -> Pixels<AA>
    where
        (): AliasLevel<AA>,
    {
        let mut p = Pixels {
            resolution: Vec2x16::zero(),
            uvs: Vec::new(),
            offsets: [Vec2x16::zero(); AA],
        };
        p.resize(self.width, self.height);
        p
    }
}

impl Default for PixelsBuilder {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
        }
    }
}
