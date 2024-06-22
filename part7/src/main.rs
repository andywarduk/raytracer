use std::path::Path;
use std::sync::Arc;

use raytracer_lib::ambience::gradient_light::GradientLight;
use raytracer_lib::camera::Camera;
use raytracer_lib::colour::Colour;
use raytracer_lib::hittable_list::HittableList;
use raytracer_lib::materials::lambertian::Lambertian;
use raytracer_lib::shapes::quad::Quad;
use raytracer_lib::vec3::{Point3, Vec3};

fn main() {
    // Materials
    let left_red = Arc::new(Lambertian::new_with_colour(Colour::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new_with_colour(Colour::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new_with_colour(Colour::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new_with_colour(Colour::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new_with_colour(Colour::new(0.2, 0.8, 0.8)));

    // Objects
    let mut world = HittableList::new();

    world.add(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    ));
    world.add(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    ));

    // Camera
    let mut cam = Camera::new(400, 1.0, 100, 50);

    cam.set_vfov(80.0);

    // Render
    cam.set_view(
        Point3::new(0.0, 0.0, 9.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    cam.render(
        &world,
        &GradientLight::new(Colour::new(1.0, 1.0, 1.0), Colour::new(0.5, 0.7, 1.0)),
        Path::new("part7.png"),
    );
}
