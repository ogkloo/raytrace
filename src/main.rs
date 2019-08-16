// use nalgebra::base::unit::Unit;
use nalgebra::geometry::Isometry3;
use nalgebra::geometry::Translation3;
use nalgebra::geometry::UnitQuaternion;
use nalgebra::Vector3;
use ncollide3d::math::Point;
use ncollide3d::shape::*;
use raytrace::*;

fn main() {
    let xsize = 640;
    let ysize = 480;
    let view = Viewport::new(
        Point::new(0.0, 10.0, 10.0),
        Vector3::new(0.0, -2.0, -1.0),
        Vector3::new(0.0, 0.0, 1.0),
        std::f64::consts::PI / 2.0,
        (xsize, ysize),
    );
    // Example isometries. This is the way you express position of an object-- Collision is
    // checked between a ray (our camera's light ray) and a RayCast object (ie a
    // sphere, cuboid, etc) under a certain isometry.
    // The UnitQuaternion means no rotation. Quaternions are used widely in computer graphics to
    // represent rotation. The Wikipedia: https://en.wikipedia.org/wiki/Quaternions_and_spatial_rotation
    let cube = Polyhedron::new(
        Box::new(Cuboid::new(Vector3::new(1.0, 1.0, 1.0))),
        image::Rgb([255, 0, 255]),
        Isometry3::from_parts(
            Translation3::from(Vector3::new(-4.0, 0.0, 0.0)),
            UnitQuaternion::identity(),
        ),
    );
    let ground = Polyhedron::new(
        Box::new(Cuboid::new(Vector3::new(20.0, 20.0, 0.0))),
        image::Rgb([0, 0, 255]),
        Isometry3::from_parts(
            Translation3::from(Vector3::new(0.0, 0.0, -1.0)),
            UnitQuaternion::identity(),
        ),
    );
    let sphere = Polyhedron::new(
        Box::new(Ball::new(2.0)),
        image::Rgb([0, 120, 0]),
        Isometry3::from_parts(
            Translation3::from(Vector3::new(6.0, 0.0, 9.0)),
            UnitQuaternion::identity(),
        ),
    );
    let sphere2 = Polyhedron::new(
        Box::new(Ball::new(2.0)),
        image::Rgb([0, 0, 0]),
        Isometry3::from_parts(
            Translation3::from(Vector3::new(0.0, 3.0, 9.0)),
            UnitQuaternion::identity(),
        ),
    );
    // This is an example scene
    let scene = Scene::new(
        vec![ground, sphere, sphere2, cube],
        view,
        image::Rgb([120, 120, 120]),
        1.0,
        vec![],
    );
    // Render the scene, consuming it.
    scene.render("output.png".to_string());
}
