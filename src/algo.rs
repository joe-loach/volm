use uv::simd::*;
use uv::Vec3x16;

use crate::ray::Ray;
use crate::Splat;

/// Trace information for Ray Marching.
#[derive(Debug)]
pub struct Trace {
    dist: f32x16,
    steps: u32x16,
    hit: mask32x16,
}

impl Trace {
    /// Get the mask of the rays that hit.
    #[inline]
    pub fn hit(&self) -> mask32x16 {
        self.hit
    }

    /// Get the distances that each ray travelled.
    #[inline]
    pub fn distances(&self) -> f32x16 {
        self.dist
    }
}

/// The maximum distance that the ray can travel.
pub const MAX_DIST: f32 = 1000.0;
/// The maximum amount of steps that the algorithm can take.
pub const MAX_ITERS: u32 = 1000;
/// Epsilon value (very small).
const EP: f32 = 1e-6;

/// Ray marches the `scene` and returns a trace.
pub fn march<S>(scene: &S, ray: &Ray) -> Trace
where
    S: Fn(Vec3x16) -> f32x16,
{
    const H: f32x16 = f32x16::splat(EP);
    let mut t = Trace {
        dist: f32x16::splat(0.0),
        steps: u32x16::splat(0),
        hit: mask32x16::splat(false),
    };

    // NOTE: (according to profiling)
    // using two separate "break t"s as such:
    // loop { break t; ... break t; }
    // is faster than:
    // loop { break; ... break; } t

    // The ray marching algorithm stops looping when:
    //    The ray has hit
    // OR The ray has travelled too far
    // OR The ray has taken too many steps
    loop {
        // get the step size
        let step = scene(ray.at(t.dist));
        // find lanes that hit something
        t.hit = step.lanes_lt(H * t.dist.abs());
        // break early if all rays have hit
        if t.hit.all() {
            break t;
        }
        // now, step foward in the march
        // don't care if we add hitted distances
        // they're too small to worry about anyways
        t.dist += step;
        // find the lanes that have finished marching
        let finished = t.hit
            | t.dist.lanes_gt(f32x16::splat(MAX_DIST))
            | t.steps.lanes_eq(u32x16::splat(MAX_ITERS));
        // Computes `finished.select(u32s::splat(0), u32s::splat(1))`, but faster...
        // SAFETY:
        // 1. The mask is converted to the mask32s's inner [i32;MAXLANES] (noop).
        // 2. We add one so: FALSE = 1 (0 + 1) and TRUE = 0 (-1 + 1)
        //    both of these are unsigned integers.
        // 3. The u32s is a wrapper around [u32;MAXLANES].
        // 4. The sizes and alignments of i32 and u32 are the same.
        // 5. The transmute from i32s to u32s compiles down to a noop.
        let inc: u32x16 = unsafe { core::mem::transmute(finished.to_int() + 1_i32.splat()) };
        // increment the iter count for the rays that remain
        t.steps += inc;
        // if none can iter any further, stop looping
        if finished.all() {
            break t;
        }
    }
}
