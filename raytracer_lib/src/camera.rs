use std::sync::atomic::{AtomicU64, Ordering};

use rand::{rngs::ThreadRng, thread_rng, Rng};
use rayon::prelude::*;

use crate::{
    ambient::ambience::Ambience,
    hits::{
        hittable::{Hittable, T_MIN},
        hittable_list::HittableList,
    },
    ray::Ray,
    triple::{Colour, Point3, Vec3},
};

pub type CamProgressCb = Option<fn(l: u64, h: u64)>;

#[derive(Debug, Default)]
pub struct Camera {
    /// Image width
    image_width: u64,
    /// Image height
    image_height: u64,
    /// Image aspect ratio
    aspect_ratio: f64,
    /// Point camera is looking from
    look_from: Point3,
    /// Point camera is looking at
    look_at: Point3,
    /// Camera-relative "up" direction
    vup: Vec3,
    /// Camera frame basis vector u
    u: Vec3,
    /// Camera frame basis vector v
    v: Vec3,
    /// Camera frame basis vector w
    w: Vec3,
    /// Vertical view angle (field of view)
    vfov: f64,
    /// Location of pixel (0,0,0)
    pixel00_loc: Point3,
    /// Offset to pixel to the right
    pixel_delta_u: Vec3,
    /// Offset to pixel below
    pixel_delta_v: Vec3,
    /// Count of random samples for each pixel
    samples_per_pixel: u64,
    /// Color scale factor for a sum of pixel samples
    pixel_samples_scale: f64,
    /// Maximum number of ray bounces into scene
    max_depth: u64,
    /// Variation angle of rays through each pixel
    defocus_angle: f64,
    /// Distance from camera look from point to plane of perfect focus
    focus_dist: f64,
    /// Defocus disk horizontal radius
    defocus_disk_u: Vec3,
    /// Defocus disk vertical radius
    defocus_disk_v: Vec3,
    /// Time span
    time_span: f64,
}

impl Camera {
    /// Creates a new camera
    pub fn new(
        image_width: u64,
        aspect_ratio: f64,
        samples_per_pixel: u64,
        max_depth: u64,
    ) -> Self {
        // Calculate the image height
        let image_height = (image_width as f64 / aspect_ratio) as u64;

        // Build result
        let mut result = Self {
            vup: Vec3::new(0.0, 1.0, 0.0),
            vfov: 90.0,
            look_at: Point3::new(0.0, 0.0, -1.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,

            image_width,
            image_height,
            aspect_ratio,
            samples_per_pixel,
            max_depth,

            ..Default::default()
        };

        result.recalculate();

        result
    }

    /// Sets the image size
    pub fn set_dimensions(&mut self, width: u64, height: u64) {
        self.image_width = width;
        self.image_height = height;

        self.aspect_ratio = width as f64 / height as f64;

        self.recalculate();
    }

    /// Sets the image width and recalculates the height from the aspect ratio
    pub fn set_width(&mut self, width: u64) {
        self.image_width = width;

        // Calculate the image height
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u64;

        self.recalculate();
    }

    /// Sets the image height and recalculates the width from the aspect ratio
    pub fn set_height(&mut self, height: u64) {
        self.image_height = height;

        // Calculate the image width
        self.image_width = (self.image_height as f64 * self.aspect_ratio) as u64;

        self.recalculate();
    }

    /// Sets the camera's view parameters
    pub fn set_view(&mut self, look_from: Point3, look_at: Point3, vup: Vec3) {
        self.look_from = look_from;
        self.look_at = look_at;
        self.vup = vup;

        self.recalculate();
    }

    /// Set the vertical field of vision in degrees
    pub fn set_vfov(&mut self, vfov: f64) {
        self.vfov = vfov;

        self.recalculate();
    }

    /// Set camera focus parameters
    pub fn set_focus(&mut self, defocus_angle: f64, focus_dist: f64) {
        self.defocus_angle = defocus_angle;
        self.focus_dist = focus_dist;

        self.recalculate();
    }

    /// Sets the render time span
    pub fn set_time_span(&mut self, time_span: f64) {
        // Values > 1.0 will mean bounding boxes for moving shapes are incorrect
        assert!((0.0..=1.0).contains(&time_span));

        self.time_span = time_span;
    }

    /// Sets the samples per pixel
    pub fn set_samples_per_pixel(&mut self, samples_per_pixel: u64) {
        self.samples_per_pixel = samples_per_pixel;

        self.recalculate();
    }

    /// Gets maximum ray depth
    pub fn set_max_depth(&mut self, max_depth: u64) {
        self.max_depth = max_depth;
    }

    /// Gets the image width
    pub fn dimensions(&self) -> (u64, u64) {
        (self.image_width, self.image_height)
    }

    /// Get view
    pub fn view(&self) -> (Point3, Point3, Vec3) {
        (
            self.look_from.clone(),
            self.look_at.clone(),
            self.vup.clone(),
        )
    }

    /// Get the vertical field of vision in degrees
    pub fn vfov(&self) -> f64 {
        self.vfov
    }

    /// Get camera focus parameters
    pub fn focus(&self) -> (f64, f64) {
        (self.defocus_angle, self.focus_dist)
    }

    /// Gets the render time span
    pub fn time_span(&self) -> f64 {
        self.time_span
    }

    /// Gets the samples per pixel
    pub fn samples_per_pixel(&self) -> u64 {
        self.samples_per_pixel
    }

    /// Gets maximum ray depth
    pub fn max_depth(&self) -> u64 {
        self.max_depth
    }

    /// Renders the scene
    pub fn render(
        &self,
        world: &HittableList,
        ambience: &dyn Ambience,
        progresscb: CamProgressCb,
    ) -> Vec<Vec<Colour>> {
        let left = AtomicU64::new(1);

        // For each scan line...
        (0..self.image_height)
            .into_par_iter()
            .map(|j| {
                // Get random number generator
                let mut rng = thread_rng();

                // For each column...
                let line = (0..self.image_width)
                    .map(|i| {
                        // Calculate pixel colour
                        (0..self.samples_per_pixel)
                            .map(|_| {
                                // Construct a random ray
                                let ray = self.get_ray(i, j, &mut rng);

                                // Get the ray's colour
                                Self::ray_colour(&mut rng, &ray, world, ambience, self.max_depth)
                            })
                            .sum::<Colour>()
                            * self.pixel_samples_scale
                    })
                    .collect::<Vec<_>>();

                // Report progress
                if let Some(progresscb) = progresscb {
                    progresscb(left.fetch_add(1, Ordering::Relaxed), self.image_height);
                }

                line
            })
            .collect::<Vec<_>>()
    }

    /// Recalculate camera parameters
    fn recalculate(&mut self) {
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        // Calculate viewport dimensions
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width: f64 =
            viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = self.look_at.vec_to(&self.look_from).unit_vector();
        self.u = self.vup.cross(&self.w).unit_vector();
        self.v = self.w.cross(&self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * &self.u;
        let viewport_v = viewport_height * -&self.v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = &viewport_u / self.image_width as f64;
        self.pixel_delta_v = &viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            &self.look_from - (self.focus_dist * &self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (&self.pixel_delta_u + &self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = &self.u * defocus_radius;
        self.defocus_disk_v = &self.v * defocus_radius;
    }

    /// Construct a camera ray originating from the defocus disk and directed
    /// at a randomly sampled point around the pixel location i, j
    fn get_ray(&self, i: u64, j: u64, rng: &mut ThreadRng) -> Ray {
        // Calculate random offset in the pixel square
        let offset = self.sample_square(rng);

        // Calculate the point in the viewport to sample
        let pixel_sample = &self.pixel00_loc
            + ((i as f64 + offset.x()) * &self.pixel_delta_u)
            + ((j as f64 + offset.y()) * &self.pixel_delta_v);

        // Ray origin
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.look_from.clone()
        } else {
            self.defocus_disk_sample(rng)
        };

        // Ray direction
        let ray_direction = ray_origin.vec_to(&pixel_sample);

        // Ray time
        let time = if self.time_span > 0.0 {
            rng.gen_range(0.0..self.time_span)
        } else {
            0.0
        };

        Ray::new(ray_origin, ray_direction, time)
    }

    /// Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square
    fn sample_square(&self, rng: &mut ThreadRng) -> Vec3 {
        Vec3::new(rng.gen_range(-0.5..=0.5), rng.gen_range(-0.5..=0.5), 0.0)
    }

    /// Returns a random point in the camera defocus disk
    fn defocus_disk_sample(&self, rng: &mut ThreadRng) -> Point3 {
        let p = Vec3::new_random_in_unit_disk(rng);
        &self.look_from + ((p.x() * &self.defocus_disk_u) + (p.y() * &self.defocus_disk_v))
    }

    /// Returns the colour of a geven ray
    fn ray_colour(
        rng: &mut ThreadRng,
        ray: &Ray,
        world: &HittableList,
        ambience: &dyn Ambience,
        max_depth: u64,
    ) -> Colour {
        let cur_depth = ray.get_depth();

        if cur_depth >= max_depth {
            // Reached maximum ray bounces - return black
            return Colour::default();
        }

        match world.hit(ray, T_MIN..f64::MAX) {
            None => {
                // Ray hit nothing - return background colour
                ambience.value(ray)
            }
            Some(hit) => {
                // Ray hit an object

                // Get colour attenuation, emitted colour and the next ray (all optional)
                let (attenuation, emitted, next_ray) = hit.material.scatter(rng, ray, &hit);

                // Get attenuation colour, or black if none
                let mut attenuation = attenuation.unwrap_or_else(Colour::default);

                // Is there a next ray?
                if let Some(mut ray) = next_ray {
                    // Yes - mix the attenuation colour with the new ray's colour
                    ray.set_depth(cur_depth + 1);
                    attenuation *= Self::ray_colour(rng, &ray, world, ambience, max_depth);
                }

                // Any colour emitted?
                match emitted {
                    None => {
                        // No - just return attenuation colour
                        attenuation
                    }
                    Some(emitted) => {
                        // Yes - add emitted colour to ray colour
                        emitted + attenuation
                    }
                }
            }
        }
    }
}
