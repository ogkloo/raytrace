fn main() {
    use nalgebra::Vector3;
    use ncollide3d::math::Point;
    use ncollide3d::query::Ray;
    use raytrace::*;
    let view = Viewport::new(
        Point::new(0.0, 0.0, 0.0),
        Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0)),
        Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0)),
        std::f64::consts::PI / 2.0,
        (1920, 1080),
        1000,
    );
    println!("{:?}", view);
    println!("Hello, world!");
}
