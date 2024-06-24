use std::path::Path;

use raytracer_lib::{
    ambient::gradient_light::GradientLight,
    camera::Camera,
    hits::hittable_list::HittableList,
    materials::lambertian::Lambertian,
    shapes::sphere::Sphere,
    textures::turbulence::Turbulence,
    triple::{Colour, Point3, Vec3},
};

fn main() {
    // Textures
    let pertext = Turbulence::new(1.0, 7);
    let marble = Lambertian::new_with_texture(&pertext);

    // Objects
    let mut world = HittableList::new();

    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, &marble));
    world.add(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, &marble));

    // Camera
    let mut cam = Camera::new(400, 16.0 / 9.0, 100, 50);

    cam.set_vfov(20.0);

    // Render
    cam.set_view(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    cam.render_to_png(
        &world,
        &GradientLight::new(Colour::new(1.0, 1.0, 1.0), Colour::new(0.5, 0.7, 1.0)),
        Path::new("part5.png"),
    );
}
