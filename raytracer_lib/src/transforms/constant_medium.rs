//! Constant medium

use std::ops::Range;

use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::{
    float::*,
    hits::{
        aabb::Aabb,
        hit::Hit,
        hittable::{Hittable, HittableRef, T_MIN},
    },
    materials::{
        isotropic::Isotropic,
        material::{MatRef, Material},
    },
    ray::Ray,
    triple::{Colour, Vec3},
};

/// Constant medium class
#[derive(Debug)]
pub struct ConstantMedium<'a> {
    boundary: HittableRef<'a>,
    neg_inv_density: Flt,
    phase_function: MatRef<'a>,
}

impl<'a> ConstantMedium<'a> {
    /// Creates a new constant medium with colour
    pub fn new_with_colour(
        boundary: impl Hittable<'a> + 'a,
        density: FltPrim,
        colour: Colour,
    ) -> Self {
        Self::new_with_matref(
            HittableRef::boxed(boundary),
            density,
            MatRef::boxed(Isotropic::new_with_colour(colour)),
        )
    }

    /// Creates a new constant medium with material
    pub fn new_with_material(
        boundary: impl Hittable<'a> + 'a,
        density: FltPrim,
        material: &'a dyn Material,
    ) -> Self {
        Self::new_with_matref(
            HittableRef::boxed(boundary),
            density,
            MatRef::Borrow(material),
        )
    }

    /// Creates a new constant medium with material reference
    pub fn new_with_matref(
        boundary: HittableRef<'a>,
        density: FltPrim,
        phase_function: MatRef<'a>,
    ) -> Self {
        Self {
            boundary,
            neg_inv_density: flt(-1.0) / flt(density),
            phase_function,
        }
    }
}

impl<'a> Hittable<'a> for ConstantMedium<'a> {
    fn hit(&self, rng: &mut ThreadRng, ray: &Ray, t_range: Range<Flt>) -> Option<Hit> {
        // Does this ray enter the boundary?
        let mut hit1 = match self.boundary.hit(rng, ray, flt_min()..flt_max()) {
            None => return None,
            Some(hit) => hit,
        };

        // Does the ray exit the boundary again?
        let mut hit2 = match self.boundary.hit(rng, ray, (hit1.t + T_MIN)..flt_max()) {
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
            hit1.t = flt(0.0)
        };

        let ray_length = ray.direction().length();
        let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
        let hit_distance = self.neg_inv_density * thread_rng().gen::<FltPrim>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = hit1.t + hit_distance / ray_length;

        Some(Hit::new(
            ray.at(t),
            t,
            flt(0.0),
            flt(0.0),
            ray,
            &Vec3::new(1.0, 0.0, 0.0),
            self.phase_function.get_ref(),
        ))
    }

    fn bounding_box(&self) -> &Aabb {
        self.boundary.bounding_box()
    }
}
