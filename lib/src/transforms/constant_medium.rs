use std::{ops::Range, sync::Arc};

use rand::{thread_rng, Rng};

use crate::{
    colour::Colour,
    hittable::{
        aabb::Aabb,
        hit::Hit,
        hittable::{Hittable, T_MIN},
    },
    materials::{isotropic::Isotropic, material::Material},
    ray::Ray,
    textures::texture::Texture,
    vec3::Vec3,
};

#[derive(Debug)]
pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_funtion: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new_with_colour(boundary: Arc<dyn Hittable>, density: f64, colour: Colour) -> Self {
        Self::new_with_phase_function(
            boundary,
            density,
            Arc::new(Isotropic::new_with_colour(colour)),
        )
    }

    pub fn new_with_texture(
        boundary: Arc<dyn Hittable>,
        density: f64,
        texture: Arc<dyn Texture>,
    ) -> Self {
        Self::new_with_phase_function(
            boundary,
            density,
            Arc::new(Isotropic::new_with_texture(texture)),
        )
    }

    pub fn new_with_phase_function(
        boundary: Arc<dyn Hittable>,
        density: f64,
        phase_funtion: Arc<dyn Material>,
    ) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_funtion,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<Hit> {
        // Does this ray enter the boundary?
        let mut hit1 = match self.boundary.hit(ray, f64::MIN..f64::MAX) {
            None => return None,
            Some(hit) => hit,
        };

        // Does the ray exit the boundary again?
        let mut hit2 = match self.boundary.hit(ray, (hit1.t + T_MIN)..f64::MAX) {
            None => return None,
            Some(hit) => hit,
        };

        // Sanitise the ranges in the hits to max and min
        if hit1.t < t_range.start {
            hit1.t = t_range.start
        };
        if hit2.t > t_range.end {
            hit2.t = t_range.end
        };

        // Check hit order
        if hit1.t >= hit2.t {
            return None;
        }

        // Check hit is not before the boundary
        if hit1.t < 0.0 {
            hit1.t = 0.0
        };

        let ray_length = ray.direction().length();
        let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
        let hit_distance = self.neg_inv_density * thread_rng().gen::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = hit1.t + hit_distance / ray_length;

        Some(Hit::new(
            ray.at(t),
            t,
            0.0,
            0.0,
            ray,
            &Vec3::new(1.0, 0.0, 0.0),
            self.phase_funtion.clone(),
        ))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.boundary.bounding_box()
    }
}
