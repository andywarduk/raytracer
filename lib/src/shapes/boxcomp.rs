use std::{ops::Range, sync::Arc};

use crate::{
    hittable::aabb::Aabb,
    hittable::hit::Hit,
    hittable::hittable::Hittable,
    hittable::hittable_list::HittableList,
    materials::material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

use super::quad::Quad;

#[derive(Debug)]
pub struct BoxComp {
    sides: HittableList,
}

impl BoxComp {
    /// Returns the 3D box (six sides) that contains the two opposite vertices a & b.
    pub fn new(a: Point3, b: Point3, material: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();

        // Construct the two opposite vertices with the minimum and maximum coordinates.
        let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
        let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

        sides.add(Quad::new(
            Point3::new(min.x(), min.y(), max.z()),
            dx.clone(),
            dy.clone(),
            material.clone(),
        )); // front
        sides.add(Quad::new(
            Point3::new(max.x(), min.y(), max.z()),
            -(&dz),
            dy.clone(),
            material.clone(),
        )); // right
        sides.add(Quad::new(
            Point3::new(max.x(), min.y(), min.z()),
            -(&dx),
            dy.clone(),
            material.clone(),
        )); // back
        sides.add(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            dz.clone(),
            dy,
            material.clone(),
        )); // left
        sides.add(Quad::new(
            Point3::new(min.x(), max.y(), max.z()),
            dx.clone(),
            -(&dz),
            material.clone(),
        )); // top
        sides.add(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            dx,
            dz,
            material,
        )); // bottom

        Self { sides }
    }
}

impl Hittable for BoxComp {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<Hit> {
        self.sides.hit(ray, t_range)
    }

    fn bounding_box(&self) -> &Aabb {
        self.sides.bounding_box()
    }
}
