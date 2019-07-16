fn main() {
    use raytrace::*;
    let view = Viewport {
        eye: Ray::new(1.0, 1.0, 1.0),
        up: Ray::new(0.0, 0.0, 1.0),
        fov: std::f64::consts::PI / 2.0,
        dimensions: (1920, 1080),
        depth: 1000,
    };
    println!("Hello, world!");
}
