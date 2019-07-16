fn main() {
    use nalgebra::Vector3;
    use raytrace::*;
    let view = Viewport::new(
        Vector3::new(1.0, 1.0, 1.0),
        Vector3::new(0.0, 0.0, 1.0),
        std::f64::consts::PI / 2.0,
        (1920, 1080),
        1000,
    );
    println!("Hello, world!");
}
