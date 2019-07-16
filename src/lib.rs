use nalgebra::Vector3;
pub type Ray = Vector3<f64>;
/// Defines a viewport for a specific eye node
/// with the eye node as the origin.
pub struct Viewport {
    eye: Ray,
    up: Ray,
    fov: f64,
    dimensions: (u64, u64),
    depth: u64,
}

impl Viewport {}

pub enum Polyhedron {}

pub struct Scene {}
