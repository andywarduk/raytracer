use std::error::Error;

use binlib::bin_main;
use raytracer_lib::{
    ambient::ray_light::RayLight,
    camera::Camera,
    hits::hittable_list::HittableList,
    materials::metal::Metal,
    shapes::sphere::Sphere,
    triple::{Colour, Point3, Vec3},
};

fn main() -> Result<(), Box<dyn Error>> {
    // Materials
    let material = Metal::new(Colour::new(1.0, 1.0, 1.0), 0.0);

    // World
    let mut world = HittableList::new();

    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.7, &material));

    // Camera
    let mut cam = Camera::new(800, 16.0 / 9.0, 10, 2);

    cam.set_view(
        Point3::new(0.0, 0.0, -2.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    // Call common bin main
    bin_main(cam, world, RayLight::new())
}
