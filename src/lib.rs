use image::ImageBuffer;
use nalgebra::geometry::Isometry3;
use nalgebra::Vector3;
use ncollide3d::math::Point;
use ncollide3d::query::Ray;
use ncollide3d::query::RayCast;

/// Camera object for raytracing.
// Ray is a pair of a Vector3 and a Point.
#[derive(Debug)]
pub struct Viewport {
    position: Point<f64>,
    eye: Ray<f64>,
    up: Ray<f64>,
    fov: f64,
    dimensions: (u64, u64),
}

impl Viewport {
    pub fn new(
        position: Point<f64>,
        eye: Vector3<f64>,
        up: Vector3<f64>,
        fov: f64,
        dimensions: (u64, u64),
    ) -> Self {
        Viewport {
            position,
            eye: Ray::new(position, eye),
            up: Ray::new(position, up),
            fov,
            dimensions,
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
    pub fn imagebuffer(&self) -> image::RgbImage {
        ImageBuffer::new(self.dimensions.0 as u32, self.dimensions.1 as u32)
    }
}

/// A shape and its material properties
// Other properties go here as we progress
#[derive(Debug)]
pub struct Polyhedron<R: RayCast<f64>> {
    shape: R,
    color: image::Rgb<u8>,
    // reflectivity: f64,
    // refractivity: f64,
}

impl<R: RayCast<f64>> Polyhedron<R> {
    pub fn new(shape: R, color: image::Rgb<u8>) -> Self {
        Polyhedron { shape, color }
    }
}

// Struct for a scene containing objects and 1 camera. When lights are added they go here. Part of
// me thinks there's a better way to do this than force a user to look at... >>> <-- this ugly
// thing. But I can't think of it right now and no one was really sure as trait aliasing is
// experimental and I'd prefer to keep it to normal code right now.
/// Describes a scene, including what objects are in the scene, a camera, and a background color.
#[derive(Debug)]
pub struct Scene<R: RayCast<f64>> {
    // This is annoying but the Vec of Box gives me a warning that I'm not sure on how to fix.
    objects: Vec<Polyhedron<R>>,
    camera: Viewport,
    default_color: image::Rgb<u8>,
}

impl<R: RayCast<f64>> Scene<R> {
    pub fn new(
        objects: Vec<Polyhedron<R>>,
        camera: Viewport,
        default_color: image::Rgb<u8>,
    ) -> Self {
        Scene::<R> {
            objects,
            camera,
            default_color,
        }
    }

    // This is the full-on rendering function, complete with output to an image. Is it more
    // reasonable to just give an image back? Perhaps. It might make stringing things together into
    // a video a bit easier if we decided to implement that, but given the simplicity of doing that
    // (all we'd need to do is remove the write and return the buffer) I'm keeping it this way
    // until we come up with something better.
    /// Renders the full image to an output.
    pub fn render(&self, filename: String) {
        let mut img: image::RgbImage = self.camera.imagebuffer();
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            for object in &self.objects {
                // Generate the ray to a certain pixel. This currently has a bug where it goes from
                // 1..1/size_of_side. This means the image usually won't render correctly.
                let pixel_ray = Ray::new(
                    self.camera.position,
                    Vector3::new(
                        1.0 - (2.0 * (f64::from(x) / (self.camera.dimensions.0 as f64))),
                        0.0,
                        1.0 - (2.0 * (f64::from(y) / (self.camera.dimensions.1 as f64))),
                    ) + self.camera.eye.dir,
                );
                let res = Viewport::draw_ray(&pixel_ray, &object);
                // println!("{} {} {:?} {:?}", x, y, pixel_ray, res);
                match res {
                    Some(color) => *pixel = color,
                    None => *pixel = self.default_color,
                }
            }
        }
        img.save(filename).unwrap();
    }
}
