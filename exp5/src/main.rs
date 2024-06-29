//! Camera inside mirror box containing a spherical light source
//! Mirror slightly yellow in colour
//! Spherical light source is invisible to rays originating from the camera

use std::error::Error;

use binlib::bin_main;
use raytracer_lib::{
    ambient::ambient_light::AmbientLight,
    camera::Camera,
    hits::hittable_list::HittableList,
    materials::{diffuse_light::DiffuseLight, metal::Metal},
    shapes::{boxcomp::BoxComp, sphere::Sphere},
    transforms::invisible_for::InvisibleFor,
    triple::{Colour, Point3, Vec3},
};

fn main() -> Result<(), Box<dyn Error>> {
    // Materials
    let light = DiffuseLight::new_with_colour(Colour::new(1.0, 1.0, 1.0));
    let mirror = Metal::new(Colour::new(0.99, 0.97, 0.83), 0.0);

    // World
    let mut world = HittableList::new();

    const MD: f64 = 10.0;

    // Box of mirrors
    world.add(BoxComp::new(
        Point3::new(-(MD / 2.0), -(MD / 2.0), -(MD / 2.0)),
        Point3::new(MD / 2.0, MD / 2.0, MD / 2.0),
        &mirror,
    ));

    // Light
    world.add(InvisibleFor::new(
        1,
        Sphere::new(Point3::new(0.0, 0.0, 0.0), 0.4, &light),
    ));

    // Ambient light
    let ambience = AmbientLight::new(Colour::default());

    // Camera
    let mut cam = Camera::new(800, 1.0, 25, 200);

    cam.set_view(
        Point3::new(0.0, 0.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    cam.set_vfov(90.0);

    // Call common bin main
    bin_main(cam, world, ambience)
}
