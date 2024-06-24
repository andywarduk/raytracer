use std::path::Path;

use raytracer_lib::ambient::ambient_light::AmbientLight;
use raytracer_lib::camera::Camera;
use raytracer_lib::colour::Colour;
use raytracer_lib::hits::hittable_list::HittableList;
use raytracer_lib::materials::diffuse_light::DiffuseLight;
use raytracer_lib::materials::lambertian::Lambertian;
use raytracer_lib::shapes::quad::Quad;
use raytracer_lib::shapes::sphere::Sphere;
use raytracer_lib::textures::marble::Marble;
use raytracer_lib::vec3::{Point3, Vec3};

fn main() {
    // Materials
    let pertext = Marble::new(4.0, 7, 2);
    let difflight = DiffuseLight::new_with_colour(Colour::new(4.0, 4.0, 4.0));
    let marble = Lambertian::new_with_texture(&pertext);

    // Objects
    let mut world = HittableList::new();

    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, &marble));
    world.add(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, &marble));
    world.add(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        &difflight,
    ));

    // Camera
    let mut cam = Camera::new(400, 16.0 / 9.0, 100, 50);

    cam.set_vfov(20.0);

    // Render
    cam.set_view(
        Point3::new(26.0, 3.0, 6.0),
        Point3::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    cam.render(
        &world,
        &AmbientLight::new(Colour::default()),
        Path::new("part8.png"),
    );
}
