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

/// A light with no direction to it. Sits at a point in space. Intensity is added onto the color of
/// the object that the light is applied to.
pub struct Light {
    position: Point<f64>,
    intensity: u8,
}

impl Light {
    /// Applies the intensity of the light to an object and return the new color.
    pub fn apply_intensity(&self, object: &Polyhedron) -> image::Rgb<u8> {
        image::Rgb([
            object.color[0] + self.intensity,
            object.color[1] + self.intensity,
            object.color[2] + self.intensity,
        ])
    }

    /// Checks if there are objects in the way of the position and the light.
    ///
    /// # Note:
    /// Ideally this would only check some subset of the objects in the scene. Binary space
    /// partitioning would do that. Since we're primarily interested in readability and having
    /// actually working code at the moment, this is left to another time.
    pub fn draw_ray_to<'a>(&self, point: Point<f64>, objects: &[Polyhedron<'a>]) -> bool {
        false
    }
}

/// Describes a scene, including what objects are in the scene, a camera, ambient lighting, and a
/// background color.
///
/// # Note
/// Ambient light currently adds. It should not. It should instead multiply. This is because we
/// need to make sure the API part of lighting actually works and it's a lot harder to do if
/// everything is just blacked out regularly.
pub struct Scene<'a> {
    objects: Vec<Polyhedron<'a>>,
    camera: Viewport,
    default_color: image::Rgb<u8>,
    ambient_light: u8,
}

impl<'a> Scene<'a> {
    #[inline]
    pub fn new(
        objects: Vec<Polyhedron<'a>>,
        camera: Viewport,
        default_color: image::Rgb<u8>,
        ambient_light: u8,
    ) -> Self {
        Scene::<'a> {
            objects,
            camera,
            default_color,
            ambient_light,
        }
    }

    /// Safe application of ambient lighting to a color while avoiding overflow.
    fn apply_ambient(&self, color: image::Rgb<u8>) -> image::Rgb<u8> {
        let red = match color[0].checked_add(self.ambient_light) {
            Some(res) => res,
            None => 255,
        };
        let green = match color[1].checked_add(self.ambient_light) {
            Some(res) => res,
            None => 255,
        };
        let blue = match color[2].checked_add(self.ambient_light) {
            Some(res) => res,
            None => 255,
        };
        image::Rgb([red, green, blue])
    }

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
            *pixel = self.apply_ambient(self.default_color);
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
                        *pixel = self.apply_ambient(color);
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
