use nalgebra::Vector3;
use ncollide3d::math::Point;
use ncollide3d::query::Ray;

/// Defines a viewport for a specific eye node
/// with the eye node as the origin.
/// FoV is in radians.
#[derive(Debug)]
pub struct Viewport {
    point: Point<f64>,
    eye: Ray<f64>,
    up: Ray<f64>,
    fov: f64,
    dimensions: (u64, u64),
    depth: u64,
}

impl Viewport {
    pub fn new(
        p: Point<f64>,
        e: Ray<f64>,
        u: Ray<f64>,
        f: f64,
        dim: (u64, u64),
        dep: u64,
    ) -> Viewport {
        Viewport {
            point: p,
            eye: e,
            up: u,
            fov: f,
            dimensions: dim,
            depth: dep,
        }
    }
}

pub enum Polyhedron {}

pub struct Scene {}
