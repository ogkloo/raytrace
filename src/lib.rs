use nalgebra::{Matrix, VecStorage, U1, U3};
type Ray = Matrix<f64, U3, U1, VecStorage<f64, U3, U1>>;

/// Defines a viewport for a specific eye node
/// with the eye node as the origin.
pub struct Viewport {
    eye: Ray,
    fov: f64,
    dimensions: (u64, u64),
    up: Ray,
    depth: u64,
}

impl Viewport {}

pub enum Polyhedron {}

pub struct Scene {}
