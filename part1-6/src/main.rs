use std::error::Error;

use binlib::{bin_main, Renderer};
use raytracer_lib::{
    ambient::gradient_light::GradientLight,
    camera::{CamProgressCb, Camera},
    hits::hittable_list::HittableList,
    materials::{lambertian::Lambertian, metal::Metal},
    shapes::sphere::Sphere,
    triple::{Colour, Point3},
};

struct State<'a> {
    // World
    world: HittableList<'a>,

    // Ambience
    ambience: GradientLight,
}

impl<'a> Renderer for State<'a> {
    fn default_camera(&self) -> Camera {
        // Camera
        Camera::new(400, 16.0 / 9.0, 200, 10)
    }

    fn render(&self, cam: &Camera, progresscb: CamProgressCb) -> Vec<Vec<Colour>> {
        // Render
        cam.render(&self.world, &self.ambience, progresscb)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Material
    let material_ground = Lambertian::new_with_colour(Colour::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new_with_colour(Colour::new(0.1, 0.2, 0.5));
    let material_left = Metal::new(Colour::new(0.8, 0.8, 0.8), 0.3);
    let material_right = Metal::new(Colour::new(0.8, 0.6, 0.2), 1.0);

    // World
    let mut world = HittableList::new();

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        &material_ground,
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        &material_center,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        &material_left,
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        &material_right,
    ));

    // Call common bin main
    bin_main(State {
        world,
        ambience: GradientLight::new(Colour::new(1.0, 1.0, 1.0), Colour::new(0.5, 0.7, 1.0)),
    })
}
