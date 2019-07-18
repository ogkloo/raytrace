use image::ImageBuffer;
use nalgebra::geometry::Isometry3;
use nalgebra::Vector3;
use ncollide3d::math::Point;
use ncollide3d::query::Ray;
use ncollide3d::query::RayCast;

/// Camera object for raytracing.
// Ray is a pair of a Vector3 and a point.
#[derive(Debug)]
pub struct Viewport {
    position: Point<f64>,
    eye: Ray<f64>,
    up: Ray<f64>,
    fov: f64,
    pub dimensions: (u32, u32),
    depth: u64,
}

impl Viewport {
    pub fn new(
        p: Point<f64>,
        e: Vector3<f64>,
        u: Vector3<f64>,
        f: f64,
        dim: (u32, u32),
        dep: u64,
    ) -> Self {
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
    pub fn draw_ray<R: RayCast<f64>>(
        ray: &Ray<f64>,
        object: &Polyhedron<R>,
    ) -> Option<image::Rgb<u8>> {
        if object.shape.intersects_ray(&Isometry3::identity(), &ray) {
            Some(object.color)
        } else {
            None
        }
    }
}

/// A shape and its material properties
// Other properties go here as we progress
pub struct Polyhedron<R: RayCast<f64>> {
    shape: R,
    color: image::Rgb<u8>,
    // reflectivity: f64,
    // refractivity: f64,
}

// Struct for a scene containing objects and 1 camera. When lights are added they go here. Part of
// me thinks there's a better way to do this than force a user to look at... >>> <-- this ugly
// thing. But I can't think of it right now and no one was really sure as trait aliasing is
// experimental and I'd prefer to keep it to normal code right now.
// For reference, RayCast<f64> is from ncollide.
pub struct Scene<R: RayCast<f64>> {
    objects: Vec<Box<Polyhedron<R>>>,
    camera: Viewport,
}

impl<R: RayCast<f64>> Scene<R> {
    pub fn new(objs: Vec<Box<Polyhedron<R>>>, eye: Viewport) -> Self {
        Scene::<R> {
            objects: objs,
            camera: eye,
        }
    }

    // This is the full-on rendering function, complete with output to an image. Is it more
    // reasonable to just give an image back? Perhaps. It might make stringing things together into
    // a video a bit easier if we decided to implement that, but given the simplicity of doing that
    // (all we'd need to do is remove the write and return the buffer) I'm keeping it this way
    // until we come up with something better.
    pub fn render(&self, filename: String) {
        let mut img: image::RgbImage =
            ImageBuffer::new(self.camera.dimensions.0, self.camera.dimensions.1);
        for (_x, _y, pixel) in img.enumerate_pixels_mut() {
            for object in self.objects.iter() {
                *pixel = Viewport::draw_ray(&self.camera.eye, &object).unwrap();
            }
        }
        img.save(filename).unwrap();
    }
}
