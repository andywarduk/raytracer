use std::path::Path;
use std::sync::Arc;

use raytracer_lib::camera::Camera;
use raytracer_lib::hittable_list::HittableList;
use raytracer_lib::materials::lambertian::Lambertian;
use raytracer_lib::shapes::sphere::Sphere;
use raytracer_lib::textures::turbulence::Turbulence;
use raytracer_lib::vec3::{Point3, Vec3};

fn main() {
    // Textures
    let pertext = Arc::new(Turbulence::new(1.0, 7));

    // Objects
    let mut world = HittableList::new();

    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_with_texture(pertext.clone())),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new_with_texture(pertext.clone())),
    ));

    // Camera
    let mut cam = Camera::new(400, 16.0 / 9.0, 100, 50);

    cam.set_vfov(20.0);

    // Render
    cam.set_view(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    cam.render(&world, Path::new("part5.png"));
}
