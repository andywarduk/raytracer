use std::path::Path;

use rand::{rngs::ThreadRng, thread_rng, Rng};
use rayon::prelude::*;

use crate::{
    colour::Colour,
    hittable::Hittable,
    hittable_list::HittableList,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Default)]
pub struct Camera {
    /// Image width
    image_width: u64,
    /// Image height
    image_height: u64,
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
    pub fn new(
        image_width: u64,
        aspect_ratio: f64,
        samples_per_pixel: u64,
        max_depth: u64,
    ) -> Self {
        // Calculate the image height
        let image_height = (image_width as f64 / aspect_ratio) as u64;

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        // Build result
        let mut result = Self {
            vup: Vec3::new(0.0, 1.0, 0.0),
            vfov: 90.0,
            look_at: Point3::new(0.0, 0.0, -1.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,

            image_width,
            image_height,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,

            ..Default::default()
        };

        result.recalculate();

        result
    }

    pub fn set_view(&mut self, look_from: Point3, look_at: Point3, vup: Vec3) {
        self.look_from = look_from;
        self.look_at = look_at;
        self.vup = vup;

        self.recalculate();
    }

    pub fn set_vfov(&mut self, vfov: f64) {
        self.vfov = vfov;

        self.recalculate();
    }

    pub fn set_focus(&mut self, defocus_angle: f64, focus_dist: f64) {
        self.defocus_angle = defocus_angle;
        self.focus_dist = focus_dist;

        self.recalculate();
    }

    pub fn set_time_span(&mut self, time_span: f64) {
        self.time_span = time_span;
    }

    pub fn render(&self, world: &HittableList, output: &Path) {
        // Create image buffer
        let mut imgbuf = image::ImageBuffer::new(self.image_width as u32, self.image_height as u32);

        // For each scan line...
        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);

            // For each column...
            for i in 0..self.image_width {
                // Calculate pixel colour
                let pixel_colour: Colour = (0..self.samples_per_pixel)
                    .into_par_iter()
                    .map(|_| {
                        let mut rng = thread_rng();
                        let ray = self.get_ray(i, j, &mut rng);
                        Self::ray_colour(&mut rng, &ray, world, self.max_depth)
                    })
                    .sum();

                // Covnert to RGB with gamma correction
                let (r, g, b) = (self.pixel_samples_scale * pixel_colour).to_rgb_gamma();

                // Add to image data buffer
                let pixel = imgbuf.get_pixel_mut(i as u32, j as u32);
                *pixel = image::Rgb([r, g, b]);
            }
        }

        imgbuf.save(output).expect("Error saving image");

        eprintln!("\nDone");
    }

    fn recalculate(&mut self) {
        // Camera
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width: f64 =
            viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = (&self.look_from - &self.look_at).unit_vector();
        self.u = self.vup.cross(&self.w).unit_vector();
        self.v = self.w.cross(&self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * &self.u;
        let viewport_v = viewport_height * -&self.v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = &viewport_u / self.image_width as f64;
        self.pixel_delta_v = &viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            &self.look_from - (self.focus_dist * &self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (&self.pixel_delta_u + &self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = &self.u * defocus_radius;
        self.defocus_disk_v = &self.v * defocus_radius;
    }

    fn get_ray(&self, i: u64, j: u64, rng: &mut ThreadRng) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.

        let offset = self.sample_square(rng);

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
        let ray_direction = pixel_sample - &ray_origin;

        // Ray time
        let time = if self.time_span > 0.0 {
            rng.gen_range(0.0..self.time_span)
        } else {
            0.0
        };

        Ray::new(ray_origin, ray_direction, time)
    }

    fn sample_square(&self, rng: &mut ThreadRng) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(rng.gen_range(-0.5..=0.5), rng.gen_range(-0.5..=0.5), 0.0)
    }

    fn defocus_disk_sample(&self, rng: &mut ThreadRng) -> Vec3 {
        // Returns a random point in the camera defocus disk.
        let p = Vec3::new_random_in_unit_disk(rng);
        &self.look_from + (p.x() * &self.defocus_disk_u) + (p.y() * &self.defocus_disk_v)
    }

    fn ray_colour(rng: &mut ThreadRng, ray: &Ray, world: &HittableList, depth: u64) -> Colour {
        if depth == 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        match world.hit(ray, 0.001..f64::MAX) {
            None => {
                // Get unit vector of ray direction
                let unit_direction = ray.direction().unit_vector();

                // Convert y component from (-1..1) to (0..1)
                let a = 0.5 * (unit_direction.y() + 1.0);

                // Blend white with light blue
                (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
            }
            Some(hit) => match hit.material.scatter(rng, ray, &hit) {
                None => Colour::new(0.0, 0.0, 0.0),
                Some((attenuation, None)) => attenuation,
                Some((attenuation, Some(next_ray))) => {
                    attenuation * Self::ray_colour(rng, &next_ray, world, depth - 1)
                }
            },
        }
    }
}
