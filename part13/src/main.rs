use std::path::Path;
use std::sync::Arc;

use raytracer_lib::ambient::ambient_light::AmbientLight;
use raytracer_lib::camera::Camera;
use raytracer_lib::colour::Colour;
use raytracer_lib::hits::hittable_list::HittableList;
use raytracer_lib::materials::diffuse_light::DiffuseLight;
use raytracer_lib::materials::lambertian::Lambertian;
use raytracer_lib::shapes::boxcomp::BoxComp;
use raytracer_lib::shapes::quad::Quad;
use raytracer_lib::transforms::constant_medium::ConstantMedium;
use raytracer_lib::transforms::rotate_y::RotateY;
use raytracer_lib::transforms::translate::Translate;
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
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
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
        white.clone(),
    ));

    let box1 = BoxComp::new(
        Point3::default(),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = RotateY::new(15.0, Arc::new(box1));
    let box1 = Translate::new(Vec3::new(265.0, 0.0, 295.0), Arc::new(box1));
    let box1 = ConstantMedium::new_with_colour(Arc::new(box1), 0.01, Colour::default());
    world.add(box1);

    let box2 = BoxComp::new(Point3::default(), Point3::new(165.0, 165.0, 165.0), white);
    let box2 = RotateY::new(-18.0, Arc::new(box2));
    let box2 = Translate::new(Vec3::new(130.0, 0.0, 65.0), Arc::new(box2));
    let box2 = ConstantMedium::new_with_colour(Arc::new(box2), 0.01, Colour::new(1.0, 1.0, 1.0));
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

    cam.render(
        &world,
        &AmbientLight::new(Colour::default()),
        Path::new("part13.png"),
    );
}
