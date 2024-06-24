use std::path::Path;

use raytracer_lib::{
    ambient::ambient_light::AmbientLight,
    camera::Camera,
    hits::hittable_list::HittableList,
    materials::{diffuse_light::DiffuseLight, lambertian::Lambertian},
    shapes::quad::Quad,
    triple::{Colour, Point3, Vec3},
};

fn main() {
    // Materials
    let red = Lambertian::new_with_colour(Colour::new(0.65, 0.05, 0.05));
    let white = Lambertian::new_with_colour(Colour::new(0.73, 0.73, 0.73));
    let green = Lambertian::new_with_colour(Colour::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new_with_colour(Colour::new(15.0, 15.0, 15.0));

    // Objects
    let mut world = HittableList::new();

    world.add(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        &green,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        &red,
    ));
    world.add(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        &light,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        &white,
    ));
    world.add(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        &white,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        &white,
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

    cam.render_to_png(
        &world,
        &AmbientLight::new(Colour::default()),
        Path::new("part10.png"),
    );
}
