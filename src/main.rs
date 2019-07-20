use nalgebra::Vector3;
use ncollide3d::math::Point;
use ncollide3d::shape::*;
use raytrace::*;

fn main() {
    let xsize = 500;
    let ysize = 500;
    let view = Viewport::new(
        Point::new(0.0, 5.0, 5.0),
        Vector3::new(0.0, -1.0, -1.0),
        Vector3::new(0.0, 0.0, 1.0),
        std::f64::consts::PI / 2.0,
        (xsize, ysize),
    );
    // Note on building polyhedrons:
    // The position of the polyhedron must be noted as an isometry in 3d
    // The RayCast objects needs to be implemented as an f64
    let cube: Polyhedron = Polyhedron::new(
        Box::new(Cuboid::new(Vector3::new(1.0, 1.0, 1.0))),
        image::Rgb([0, 0, 0]),
    );
    let sphere: Polyhedron = Polyhedron::new(Box::new(Ball::new(2.0)), image::Rgb([125, 0, 0]));
    // This is an example scene
    let scene = Scene::new(vec![sphere, cube], view, image::Rgb([120, 120, 120]));
    scene.render("output.png".to_string());
}
