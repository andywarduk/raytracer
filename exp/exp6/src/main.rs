//! Camera inside mirror box containing a spherical light source
//! Mirror slightly yellow in colour
//! Spherical light source is invisible to rays originating from the camera

use std::error::Error;

use binlib::{bin_main, MainParms};
use raytracer_lib::{
    camera::Camera,
    hits::hittable_list::HittableList,
    materials::{diffuse_light::DiffuseLight, material::Material, metal::Metal},
    shapes::{quad::Quad, sphere::Sphere},
    transforms::invisible_for::InvisibleFor,
    triple::{Colour, Point3, Vec3},
};

const C1: f64 = 0.99;
const C2: f64 = 0.97;
const C3: f64 = 0.83;

fn main() -> Result<(), Box<dyn Error>> {
    // Materials
    let light = DiffuseLight::new_with_colour(Colour::new(1.0, 1.0, 1.0));
    let m1 = Metal::new(Colour::new(C1, C2, C3), 0.0);
    let m2 = Metal::new(Colour::new(C1, C3, C2), 0.0);
    let m3 = Metal::new(Colour::new(C2, C1, C3), 0.0);
    let m4 = Metal::new(Colour::new(C2, C3, C1), 0.0);
    let m5 = Metal::new(Colour::new(C3, C1, C2), 0.0);
    let m6 = Metal::new(Colour::new(C3, C2, C1), 0.0);

    // World
    let mut world = HittableList::new();

    const MD: f64 = 10.0;

    // Box of mirrors
    world.add(mirror_box(
        Point3::new(-(MD / 2.0), -(MD / 2.0), -(MD / 2.0)),
        Point3::new(MD / 2.0, MD / 2.0, MD / 2.0),
        &m1,
        &m2,
        &m3,
        &m4,
        &m5,
        &m6,
    ));

    // Light
    world.add(InvisibleFor::new(
        0,
        Sphere::new(Point3::new(0.0, 0.0, 0.0), 0.8, &light),
    ));

    // Camera
    let mut cam = Camera::new(800, 1.0, 25, 250);

    cam.set_view(
        Point3::new(0.0, 0.0, -(MD / 2.0)),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    cam.set_vfov(143.0);

    // Call common bin main
    bin_main(MainParms::new(cam, world))
}

#[allow(clippy::too_many_arguments)]
fn mirror_box<'a>(
    a: Point3,
    b: Point3,
    m1: &'a dyn Material,
    m2: &'a dyn Material,
    m3: &'a dyn Material,
    m4: &'a dyn Material,
    m5: &'a dyn Material,
    m6: &'a dyn Material,
) -> HittableList<'a> {
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
        m1,
    )); // front
    sides.add(Quad::new(
        Point3::new(max.x(), min.y(), max.z()),
        -(&dz),
        dy.clone(),
        m2,
    )); // right
    sides.add(Quad::new(
        Point3::new(max.x(), min.y(), min.z()),
        -(&dx),
        dy.clone(),
        m3,
    )); // back
    sides.add(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dz.clone(),
        dy,
        m4,
    )); // left
    sides.add(Quad::new(
        Point3::new(min.x(), max.y(), max.z()),
        dx.clone(),
        -(&dz),
        m5,
    )); // top
    sides.add(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dx,
        dz,
        m6,
    )); // bottom

    sides
}
