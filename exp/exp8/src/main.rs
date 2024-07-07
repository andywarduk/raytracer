//! Spheres made of ConstantMedium
//! Ambient light is ray normal vector

use std::error::Error;

use binlib::{bin_main, MainParms};
use raytracer_lib::{
    ambient::ambient_light::AmbientLight,
    camera::Camera,
    float::*,
    hits::hittable_list::HittableList,
    materials::{
        dielectric::Dielectric, diffuse::Diffuse, diffuse_light::DiffuseLight,
        isotropic::Isotropic, lambertian::Lambertian, metal::Metal, normal::Normal,
    },
    shapes::sphere::Sphere,
    transforms::constant_medium::ConstantMedium,
    triple::{Colour, Point3, Vec3},
};

fn main() -> Result<(), Box<dyn Error>> {
    // Materials
    let mat1 = Isotropic::new_with_colour(Colour::new(1.0, 1.0, 1.0));
    let mat2 = Diffuse::new(Colour::new(1.0, 1.0, 1.0));
    let mat3 = Lambertian::new_with_colour(Colour::new(1.0, 1.0, 1.0));
    let mat4 = Dielectric::new(1.5);
    let mat5 = DiffuseLight::new_with_colour(Colour::new(1.0, 1.0, 1.0));
    let mat6 = Metal::new(Colour::new(1.0, 1.0, 1.0), 0.0);
    let mat7 = Normal::new();

    println!("Materials:");
    println!("  Isotropic    Diffuse        Lambertian");
    println!("  Dielectric  [DiffuseLight]  Metal");
    println!("  Normal       DiffuseLight");

    // World
    let mut world = HittableList::new();

    const DENSITY: FltPrim = 0.5;

    // Balls
    world.add(ConstantMedium::new_with_material(
        Sphere::new(Point3::new(-1.0, 1.0, 0.0), 0.4, &mat1),
        DENSITY,
        &mat1,
    ));
    world.add(ConstantMedium::new_with_material(
        Sphere::new(Point3::new(0.0, 1.0, 0.0), 0.4, &mat2),
        DENSITY,
        &mat2,
    ));
    world.add(ConstantMedium::new_with_material(
        Sphere::new(Point3::new(1.0, 1.0, 0.0), 0.4, &mat3),
        DENSITY,
        &mat3,
    ));
    world.add(ConstantMedium::new_with_material(
        Sphere::new(Point3::new(-1.0, 0.0, 0.0), 0.4, &mat4),
        DENSITY,
        &mat4,
    ));
    world.add(Sphere::new(Point3::new(0.0, 0.0, 0.0), 0.4, &mat5));
    world.add(ConstantMedium::new_with_material(
        Sphere::new(Point3::new(1.0, 0.0, 0.0), 0.4, &mat6),
        DENSITY,
        &mat6,
    ));
    world.add(ConstantMedium::new_with_material(
        Sphere::new(Point3::new(-1.0, -1.0, 0.0), 0.4, &mat7),
        DENSITY,
        &mat7,
    ));
    world.add(ConstantMedium::new_with_material(
        Sphere::new(Point3::new(0.0, -1.0, 0.0), 0.4, &mat5),
        DENSITY,
        &mat5,
    ));

    // Ambient light
    let ambience = AmbientLight::new(Colour::default());

    // Camera
    let mut cam = Camera::new(800, 1.0, 500, 10);

    cam.set_view(
        Point3::new(0.0, 0.0, 4.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    cam.set_vfov(45.0);

    // Call common bin main
    bin_main(MainParms::new_ambience(cam, world, ambience))
}
