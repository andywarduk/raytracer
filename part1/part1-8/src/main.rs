use std::error::Error;

use binlib::{bin_main, MainParms};
use raytracer_lib::{
    ambient::gradient_light::GradientLight,
    camera::Camera,
    float::*,
    hits::hittable_list::HittableList,
    materials::lambertian::Lambertian,
    shapes::sphere::Sphere,
    triple::{Colour, Point3},
};

fn main() -> Result<(), Box<dyn Error>> {
    // Material
    let material_left = Lambertian::new_with_colour(Colour::new(0.0, 0.0, 1.0));
    let material_right = Lambertian::new_with_colour(Colour::new(1.0, 0.0, 0.0));

    // World
    let mut world = HittableList::new();

    let r = (PI / 4.0).cos();

    world.add(Sphere::new(Point3::new(-r, 0.0, -1.0), r, &material_left));
    world.add(Sphere::new(Point3::new(r, 0.0, -1.0), r, &material_right));

    // Call common bin main
    bin_main(MainParms::new_ambience(
        Camera::new(400, 16.0 / 9.0, 200, 10),
        world,
        GradientLight::new(Colour::new(1.0, 1.0, 1.0), Colour::new(0.5, 0.7, 1.0)),
    ))
}
