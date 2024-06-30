use std::error::Error;

use binlib::{bin_main, MainParms};
use raytracer_lib::{
    ambient::gradient_light::GradientLight,
    camera::Camera,
    hits::hittable_list::HittableList,
    materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    shapes::sphere::Sphere,
    triple::{Colour, Point3, Vec3},
};

fn main() -> Result<(), Box<dyn Error>> {
    // Material
    let material_ground = Lambertian::new_with_colour(Colour::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new_with_colour(Colour::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_bubble = Dielectric::new(1.00 / 1.5);
    let material_right = Metal::new(Colour::new(0.8, 0.6, 0.2), 1.0);

    // World
    let mut world = HittableList::new();

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        &material_ground,
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        &material_center,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        &material_left,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        &material_bubble,
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        &material_right,
    ));

    // Camera
    let mut cam = Camera::new(400, 16.0 / 9.0, 200, 10);

    cam.set_view(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    cam.set_vfov(20.0);

    // Call common bin main
    bin_main(MainParms::new_ambience(
        cam,
        world,
        GradientLight::new(Colour::new(1.0, 1.0, 1.0), Colour::new(0.5, 0.7, 1.0)),
    ))
}
