pub mod raytrace {
    use nalgebra::{Matrix, VecStorage, U1, U3};
    pub type Ray = Matrix<f64, U3, U1, VecStorage<f64, U3, U1>>;

    /// Defines a viewport for a specific eye node
    /// with the eye node as the origin.
    pub struct Viewport {
        pub eye: Ray,
        pub up: Ray,
        pub fov: f64,
        pub dimensions: (u64, u64),
        pub depth: u64,
    }

    impl Viewport {}

    pub enum Polyhedron {}

    pub struct Scene {}
}
