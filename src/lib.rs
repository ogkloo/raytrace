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
    dimensions: (u32, u32),
}

impl Viewport {
    pub fn new(p: Point<f64>, e: Vector3<f64>, u: Vector3<f64>, f: f64, dim: (u32, u32)) -> Self {
        Viewport {
            position: p,
            eye: Ray::new(p, e),
            up: Ray::new(p, u),
            fov: f,
            dimensions: dim,
        }
    }

    /// Draws a ray at a certain angle and returns the color of whatever it
    /// intersects. Note that the length of the vector does not matter.
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

    /// Makes an imagebuffer from the dimensions of the viewport.
    pub fn make_imagebuffer_from_viewport(&self) -> image::RgbImage {
        ImageBuffer::new(self.dimensions.0, self.dimensions.1)
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

impl<R: RayCast<f64>> Polyhedron<R> {
    pub fn new(object: R, c: image::Rgb<u8>) -> Self {
        Polyhedron {
            shape: object,
            color: c,
        }
    }
}

// Struct for a scene containing objects and 1 camera. When lights are added they go here. Part of
// me thinks there's a better way to do this than force a user to look at... >>> <-- this ugly
// thing. But I can't think of it right now and no one was really sure as trait aliasing is
// experimental and I'd prefer to keep it to normal code right now.
pub struct Scene<R: RayCast<f64>> {
    objects: Vec<Polyhedron<R>>,
    camera: Viewport,
    default_color: image::Rgb<u8>,
}

impl<R: RayCast<f64>> Scene<R> {
    pub fn new(objs: Vec<Polyhedron<R>>, eye: Viewport, background: image::Rgb<u8>) -> Self {
        Scene::<R> {
            objects: objs,
            camera: eye,
            default_color: background,
        }
    }

    // This is the full-on rendering function, complete with output to an image. Is it more
    // reasonable to just give an image back? Perhaps. It might make stringing things together into
    // a video a bit easier if we decided to implement that, but given the simplicity of doing that
    // (all we'd need to do is remove the write and return the buffer) I'm keeping it this way
    // until we come up with something better.
    // I WILL test this by tomorrow I promise.
    pub fn render(&self, filename: String) {
        let mut img: image::RgbImage = self.camera.make_imagebuffer_from_viewport();
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            for object in self.objects.iter() {
                let res = Viewport::draw_ray(&self.camera.eye, &object);
                println!("{} {} {:?}", x, y, res);
                match res {
                    Some(color) => *pixel = color,
                    None => *pixel = self.default_color,
                }
            }
        }
        img.save(filename).unwrap();
    }
}
