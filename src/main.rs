fn main() {
    use nalgebra::Vector3;
    use ncollide3d::math::Point;
    use raytrace::*;
    let xsize = 1920;
    let ysize = 1080;
    let view = Viewport::new(
        Point::new(0.0, 0.0, 0.0),
        Vector3::new(1.0, 1.0, 1.0),
        Vector3::new(0.0, 0.0, 1.0),
        std::f64::consts::PI / 2.0,
        (xsize, ysize),
        1000,
    );
    println!("{:?}", view);
}
