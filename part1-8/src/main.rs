use std::{error::Error, f64::consts::PI};

use binlib::{bin_main, Renderer};
use raytracer_lib::{
    ambient::gradient_light::GradientLight,
    camera::{CamProgressCb, Camera},
    hits::hittable_list::HittableList,
    materials::lambertian::Lambertian,
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
    let material_left = Lambertian::new_with_colour(Colour::new(0.0, 0.0, 1.0));
    let material_right = Lambertian::new_with_colour(Colour::new(1.0, 0.0, 0.0));

    // World
    let mut world = HittableList::new();

    let r = (PI / 4.0).cos();

    world.add(Sphere::new(Point3::new(-r, 0.0, -1.0), r, &material_left));
    world.add(Sphere::new(Point3::new(r, 0.0, -1.0), r, &material_right));

    // Call common bin main
    bin_main(State {
        world,
        ambience: GradientLight::new(Colour::new(1.0, 1.0, 1.0), Colour::new(0.5, 0.7, 1.0)),
    })
}
