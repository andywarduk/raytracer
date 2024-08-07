//! Spheres made of available materials
//! Ambient light is ray normal vector

use std::error::Error;

use binlib::{bin_main, MainParms};
use raytracer_lib::{
    ambient::ray_light::RayLight,
    camera::Camera,
    float::*,
    hits::hittable_list::HittableList,
    materials::{
        dielectric::Dielectric, diffuse::Diffuse, diffuse_light::DiffuseLight, dir_light::DirLight,
        isotropic::Isotropic, lambertian::Lambertian, metal::Metal, normal::Normal,
        polar_light::PolarLight,
    },
    shapes::{quad::Quad, sphere::Sphere},
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
    let mat8 = DirLight::new_with_colour(Colour::new(1.0, 1.0, 1.0));
    let mat9 = PolarLight::new_with_colour(45.0, Colour::new(1.0, 1.0, 1.0));

    println!("Materials:");
    println!("  Isotropic   Diffuse       Lambertian");
    println!("  Dielectric  DiffuseLight  Metal");
    println!("  Normal      DirLight      PolarLight");

    // World
    let mut world = HittableList::new();

    const LD: FltPrim = 4.0;

    // Square light
    world.add(Quad::new(
        Point3::new(-(LD / 2.0), -(LD / 2.0), 6.0),
        Vec3::new(LD, 0.0, 0.0),
        Vec3::new(0.0, LD, 0.0),
        &mat5,
    ));

    // Balls
    world.add(Sphere::new(Point3::new(-1.0, 1.0, 0.0), 0.4, &mat1));
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 0.4, &mat2));
    world.add(Sphere::new(Point3::new(1.0, 1.0, 0.0), 0.4, &mat3));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, 0.0), 0.4, &mat4));
    world.add(Sphere::new(Point3::new(0.0, 0.0, 0.0), 0.4, &mat5));
    world.add(Sphere::new(Point3::new(1.0, 0.0, 0.0), 0.4, &mat6));
    world.add(Sphere::new(Point3::new(-1.0, -1.0, 0.0), 0.4, &mat7));
    world.add(Sphere::new(Point3::new(0.0, -1.0, 0.0), 0.4, &mat8));
    world.add(Sphere::new(Point3::new(1.0, -1.0, 0.0), 0.4, &mat9));

    // Ambient light
    let ambience = RayLight::new();

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
