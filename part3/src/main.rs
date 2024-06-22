use std::path::Path;
use std::sync::Arc;

use raytracer_lib::ambience::ambient_light::AmbientLight;
use raytracer_lib::camera::Camera;
use raytracer_lib::colour::Colour;
use raytracer_lib::hittable_list::HittableList;
use raytracer_lib::materials::lambertian::Lambertian;
use raytracer_lib::shapes::sphere::Sphere;
use raytracer_lib::textures::image::Image;
use raytracer_lib::vec3::{Point3, Vec3};

fn main() {
    // Textures
    let earth_texture = Image::new_from_file(Path::new("earthmap.jpg"));
    let earth_surface = Lambertian::new_with_texture(Arc::new(earth_texture));

    // Objects
    let mut world = HittableList::new();

    let globe = Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, Arc::new(earth_surface));

    world.add(globe);

    // Ambient light
    let ambiance = AmbientLight::new(Colour::new(1.0, 1.0, 1.0));

    // Camera
    let mut cam = Camera::new(400, 16.0 / 9.0, 100, 50);

    cam.set_vfov(20.0);

    // Render
    cam.set_view(
        Point3::new(0.0, 0.0, 12.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    cam.render(&world, &ambiance, Path::new("part3-1.png"));

    // Render
    cam.set_view(
        Point3::new(12.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    cam.render(&world, &ambiance, Path::new("part3-2.png"));

    // Render
    cam.set_view(
        Point3::new(0.0, 0.0, -12.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    cam.render(&world, &ambiance, Path::new("part3-3.png"));

    // Render
    cam.set_view(
        Point3::new(-12.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    cam.render(&world, &ambiance, Path::new("part3-4.png"));

    // Render
    cam.set_view(
        Point3::new(0.0, 12.0, 0.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
    );

    cam.render(&world, &ambiance, Path::new("part3-5.png"));

    // Render
    cam.set_view(
        Point3::new(0.0, -12.0, 0.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
    );

    cam.render(&world, &ambiance, Path::new("part3-6.png"));
}
