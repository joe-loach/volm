#![feature(portable_simd)]

use volm::uv::{simd::*, Vec3x16};
use volm::{prelude::*, sphere::Sphere, PixelsBuilder};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

fn main() {
    let camera = PointCamera::new(Vec3x16::new(
        f32x16::splat(0.0),
        f32x16::splat(0.0),
        f32x16::splat(5.0),
    ));

    let s = Sphere::new(1.0);
    let scene = |p| s.dist(&p);

    let pixels = PixelsBuilder::new()
        .with_width(WIDTH)
        .with_height(HEIGHT)
        .build_with_aa::<1>();

    let start = std::time::Instant::now();

    let buf = pixels.render(&camera, |ray| {
        let trace = volm::algo::march(&scene, &ray);
        let dist = trace.distances();

        let pos = ray.at(dist);

        pos
    });

    println!("Rendered image in {:?}\nSaving Image", start.elapsed());

    image::save_buffer(
        "simple.png",
        buf.as_slice(),
        WIDTH,
        HEIGHT,
        image::ColorType::Rgb8,
    )
    .unwrap();
}
