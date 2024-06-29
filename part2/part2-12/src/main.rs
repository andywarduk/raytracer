use std::error::Error;

use binlib::bin_main;

use raytracer_lib::{
    ambient::ambient_light::AmbientLight,
    camera::Camera,
    hits::hittable_list::HittableList,
    materials::{diffuse_light::DiffuseLight, lambertian::Lambertian},
    shapes::{boxcomp::BoxComp, quad::Quad},
    transforms::{rotate_y::RotateY, translate::Translate},
    triple::{Colour, Point3, Vec3},
};

fn main() -> Result<(), Box<dyn Error>> {
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

    let box1 = BoxComp::new(Point3::default(), Point3::new(165.0, 330.0, 165.0), &white);
    let box1 = RotateY::new(15.0, box1);
    let box1 = Translate::new(Vec3::new(265.0, 0.0, 295.0), box1);
    world.add(box1);

    let box2 = BoxComp::new(Point3::default(), Point3::new(165.0, 165.0, 165.0), &white);
    let box2 = RotateY::new(-18.0, box2);
    let box2 = Translate::new(Vec3::new(130.0, 0.0, 65.0), box2);
    world.add(box2);

    // Camera
    let mut cam = Camera::new(600, 1.0, 500, 50);

    cam.set_vfov(40.0);

    // Render
    cam.set_view(
        Point3::new(278.0, 278.0, -800.0),
        Point3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    // Call common bin main
    bin_main(cam, world, AmbientLight::new(Colour::default()))
}
