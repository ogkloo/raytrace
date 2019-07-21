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
    #[inline]
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

    /// Draws a ray at a certain angle and returns the color and distance to whatever it hits.
    /// Note that the length of the ray does not matter (in examples all fields are usually 1.0).
    /// # Arguments:
    /// * `ray` -  The ray that will get drawn through the object. Note that the size of the ray is of
    /// no consequence. Most examples here use 1.0 for all fields.
    /// * `object` - The object that the ray will be drawn through.
    pub fn draw_ray(ray: &Ray<f64>, object: &Polyhedron) -> Option<(f64, image::Rgb<u8>)> {
        match object.shape.toi_with_ray(&object.position, &ray, false) {
            Some(distance) => Some((distance, object.color)),
            None => None,
        }
    }

    /// Makes an imagebuffer from the dimensions of the viewport.
    pub fn imagebuffer(&self) -> image::RgbImage {
        ImageBuffer::new(self.dimensions.0 as u32, self.dimensions.1 as u32)
    }
}

/// A shape and its material properties. Currently includes color and position in addition to the
/// basic shape. The object held must be specified as f64.
pub struct Polyhedron<'a> {
    shape: Box<RayCast<f64> + 'a>,
    color: image::Rgb<u8>,
    position: Isometry3<f64>,
    // These will be turned on when we're done rendering multiple images.
    // reflectivity: f64,
    // refractivity: f64,
}

impl<'a> Polyhedron<'a> {
    #[inline]
    pub fn new(
        shape: Box<RayCast<f64> + 'a>,
        color: image::Rgb<u8>,
        position: Isometry3<f64>,
    ) -> Self {
        Polyhedron {
            shape,
            color,
            position,
        }
    }
}

// Struct for a scene containing objects and 1 camera. When lights are added they go here. Part of
// me thinks there's a better way to do this than force a user to look at... >>> <-- this ugly
// thing. But I can't think of it right now and no one was really sure as trait aliasing is
// experimental and I'd prefer to keep it to normal code right now.
/// Describes a scene, including what objects are in the scene, a camera, and a background color.
pub struct Scene<'a> {
    // This is annoying but the Vec of Box gives me a warning that I'm not sure on how to fix.
    objects: Vec<Polyhedron<'a>>,
    camera: Viewport,
    default_color: image::Rgb<u8>,
}

impl<'a> Scene<'a> {
    #[inline]
    pub fn new(
        objects: Vec<Polyhedron<'a>>,
        camera: Viewport,
        default_color: image::Rgb<u8>,
    ) -> Self {
        Scene::<'a> {
            objects,
            camera,
            default_color,
        }
    }

    // Decisions: We write to the image and this function has a side effect. It cuts down on
    // boilerplate code and is relatievly expected anyways.
    // Current bug: Any write to the image will go over every object and overwrite the space every
    // time. This means only the last object in the list will be rendered.
    // Solution: Loop through each pixel, generate ray which is sent through all objects. The color
    // of closest one is returned. This is a bit more elegant, but likely requries the full vector
    // of objects to be passed to draw_ray() every time.
    /// Renders the full image to an output file.
    ///
    /// # Warning
    /// This may take awhile depending on how large of an image you specify in the camera.
    ///
    /// # Note
    /// Currently, the camera's shape (and size, fov) cannot be meaningfully set,
    /// but this would be a good feature to add!
    pub fn render(&self, filename: String) {
        let mut img: image::RgbImage = self.camera.imagebuffer();
        if self.objects.is_empty() {
            panic!("Please specify objects for rendering.");
        }
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            *pixel = self.default_color;
            let pixel_ray = Ray::new(
                self.camera.position,
                Vector3::new(
                    2.0 - (4.0 * (f64::from(x) / (self.camera.dimensions.0 as f64))),
                    0.0,
                    2.0 - (4.0 * (f64::from(y) / (self.camera.dimensions.1 as f64))),
                ) + self.camera.eye.dir,
            );
            let mut closest = Viewport::draw_ray(&pixel_ray, &self.objects[0]);
            for object in &self.objects {
                let res = Viewport::draw_ray(&pixel_ray, &object);
                if let Some((distance, color)) = res {
                    // This unwrap() is okay because we check for None -- if it's None, we know
                    // that we have to replace it as it's the furthest thing away. We saw
                    // nothing with the last ray we drew with the first object.
                    if closest == None || (distance < closest.unwrap().0) {
                        *pixel = color;
                        closest = Some((distance, color));
                    } else {
                        *pixel = closest.unwrap().1;
                    }
                }
            }
        }
        img.save(filename).unwrap();
    }
}
