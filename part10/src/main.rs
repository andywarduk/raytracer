use std::path::Path;
use std::sync::Arc;

use raytracer_lib::ambience::ambient_light::AmbientLight;
use raytracer_lib::camera::Camera;
use raytracer_lib::colour::Colour;
use raytracer_lib::hittable::hittable_list::HittableList;
use raytracer_lib::materials::diffuse_light::DiffuseLight;
use raytracer_lib::materials::lambertian::Lambertian;
use raytracer_lib::shapes::quad::Quad;
use raytracer_lib::vec3::{Point3, Vec3};

fn main() {
    // Materials
    let red = Arc::new(Lambertian::new_with_colour(Colour::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_with_colour(Colour::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_with_colour(Colour::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_with_colour(Colour::new(15.0, 15.0, 15.0)));

    // Objects
    let mut world = HittableList::new();

    world.add(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    ));
    world.add(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));
    world.add(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white,
    ));

    // Camera
    let mut cam = Camera::new(600, 1.0, 500, 50);

    cam.set_vfov(40.0);

    // Render
    cam.set_view(
        Point3::new(278.0, 278.0, -800.0),
        Point3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    cam.render(
        &world,
        &AmbientLight::new(Colour::default()),
        Path::new("part10.png"),
    );
}
