use nalgebra::geometry::Isometry3;
use nalgebra::Vector3;
use ncollide3d::math::Point;
use ncollide3d::query::Ray;
use ncollide3d::query::RayCast;

/// Defines a viewport for a specific eye node
/// with the eye node as the origin.
/// FoV is in radians.
#[derive(Debug)]
pub struct Viewport {
    position: Point<f64>,
    eye: Ray<f64>,
    up: Ray<f64>,
    fov: f64,
    dimensions: (u64, u64),
    depth: u64,
}

/// A shape and its material properties
/// Other properties go here as we progress
pub struct Polyhedron<T: RayCast<f64>> {
    shape: T,
    color: image::Rgb<u64>,
}

impl Viewport {
    /// Constructor
    pub fn new(
        p: Point<f64>,
        e: Vector3<f64>,
        u: Vector3<f64>,
        f: f64,
        dim: (u64, u64),
        dep: u64,
    ) -> Viewport {
        Viewport {
            position: p,
            eye: Ray::new(p, e),
            up: Ray::new(p, u),
            fov: f,
            dimensions: dim,
            depth: dep,
        }
    }

    /// Draws a ray at a certain angle and returns the color
    /// of whatever it intersects. Will later draw recursively.
    // Maybe it should actually just place them in a big pixel buffer, or maybe this should be
    // called by a private method that does that.
    pub fn draw_ray<T: RayCast<f64>>(ray: &Ray<f64>, object: &T) -> Option<image::Rgb<u64>> {
        if object.intersects_ray(&Isometry3::identity(), &ray) {
            Some(image::Rgb([0, 0, 0]))
        } else {
            None
        }
    }
}

pub struct Scene {}
