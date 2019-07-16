use nalgebra::Vector3;
/// Defines a viewport for a specific eye node
/// with the eye node as the origin.
pub struct Viewport {
    eye: Vector3<f64>,
    up: Vector3<f64>,
    fov: f64,
    dimensions: (u64, u64),
    depth: u64,
}

impl Viewport {
    pub fn new(e: Vector3<f64>, u: Vector3<f64>, f: f64, dim: (u64, u64), dep: u64) -> Viewport {
        Viewport {
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
