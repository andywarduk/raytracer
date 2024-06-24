use std::{error::Error, path::Path};

use binlib::save_image;
use raytracer_lib::{
    ambient::ambient_light::AmbientLight,
    camera::Camera,
    hits::hittable_list::HittableList,
    materials::lambertian::Lambertian,
    shapes::sphere::Sphere,
    textures::image::Image,
    triple::{Colour, Point3, Vec3},
};

fn main() -> Result<(), Box<dyn Error>> {
    // Textures
    let earth_texture = Image::new_from_file(Path::new("earthmap.jpg"));
    let earth_surface = Lambertian::new_with_texture(&earth_texture);

    // Objects
    let mut world = HittableList::new();

    let globe = Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, &earth_surface);

    world.add(globe);

    // Ambient light
    let ambiance = AmbientLight::new(Colour::new(1.0, 1.0, 1.0));

    // Camera
    let mut cam = Camera::new(400, 16.0 / 9.0, 100, 50);

    cam.set_vfov(20.0);

    let render = |cam: &Camera, output: &Path| -> Result<(), Box<dyn Error>> {
        save_image(cam.render(&world, &ambiance, None), output)
    };

    // Render
    cam.set_view(
        Point3::new(0.0, 0.0, 12.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    render(&cam, Path::new("part2-3-1.png"))?;

    // Render
    cam.set_view(
        Point3::new(12.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    render(&cam, Path::new("part2-3-2.png"))?;

    // Render
    cam.set_view(
        Point3::new(0.0, 0.0, -12.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    render(&cam, Path::new("part2-3-2.png"))?;

    // Render
    cam.set_view(
        Point3::new(-12.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    render(&cam, Path::new("part2-3-3.png"))?;

    // Render
    cam.set_view(
        Point3::new(0.0, 12.0, 0.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
    );

    render(&cam, Path::new("part2-3-4.png"))?;

    // Render
    cam.set_view(
        Point3::new(0.0, -12.0, 0.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
    );

    render(&cam, Path::new("part2-3-5.png"))?;

    Ok(())
}
