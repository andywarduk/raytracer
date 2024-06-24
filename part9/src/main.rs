use std::path::Path;

use raytracer_lib::{
    ambient::ambient_light::AmbientLight,
    camera::Camera,
    hits::hittable_list::HittableList,
    materials::{diffuse_light::DiffuseLight, lambertian::Lambertian},
    shapes::{quad::Quad, sphere::Sphere},
    textures::marble::Marble,
    triple::{Colour, Point3, Vec3},
};

fn main() {
    // Materials
    let pertext = Marble::new(4.0, 7, 2);
    let difflight = DiffuseLight::new_with_colour(Colour::new(4.0, 4.0, 4.0));
    let marble = Lambertian::new_with_texture(&pertext);

    // Objects
    let mut world = HittableList::new();

    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, &marble));
    world.add(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, &marble));
    world.add(Sphere::new(Point3::new(0.0, 7.0, 0.0), 2.0, &difflight));
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

    cam.render_to_png(
        &world,
        &AmbientLight::new(Colour::default()),
        Path::new("part9.png"),
    );
}
