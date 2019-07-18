fn main() {
    use nalgebra::Vector3;
    use ncollide3d::math::Point;
    use ncollide3d::shape::*;
    use raytrace::*;
    let xsize = 10;
    let ysize = 10;
    let view = Viewport::new(
        Point::new(0.0, 9.0, 0.0),
        Vector3::new(0.0, -1.0, 0.0),
        Vector3::new(0.0, 0.0, 1.0),
        std::f64::consts::PI / 2.0,
        (xsize, ysize),
    );
    // Note on building polyhedrons:
    // The position of the polyhedron must be noted as an isometry in 3d
    // The RayCast objects needs to be implemented as an f64
    let cube: Polyhedron<Cuboid<f64>> = Polyhedron::new(
        Cuboid::new(Vector3::new(1.0, 1.0, 1.0)),
        image::Rgb([0, 0, 0]),
    );
    // This is an example scene
    let scene: Scene<Cuboid<f64>> = Scene::new(vec![cube], view, image::Rgb([255, 255, 255]));
    scene.render("output.png".to_string());
}
