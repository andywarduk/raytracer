use std::error::Error;

use binlib::{bin_main, MainParms};
use raytracer_lib::{
    ambient::gradient_light::GradientLight,
    camera::Camera,
    hits::hittable_list::HittableList,
    materials::lambertian::Lambertian,
    shapes::sphere::Sphere,
    triple::{Colour, Point3},
};

fn main() -> Result<(), Box<dyn Error>> {
    // Material
    let mat = Lambertian::new_with_colour(Colour::new(0.5, 0.5, 0.5));

    // World
    let mut world = HittableList::new();

    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, &mat));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, &mat));

    // Call common bin main
    bin_main(MainParms::new_ambience(
        Camera::new(400, 16.0 / 9.0, 100, 10),
        world,
        GradientLight::new(Colour::new(1.0, 1.0, 1.0), Colour::new(0.5, 0.7, 1.0)),
    ))
}
