use std::error::Error;

use binlib::{bin_main, Renderer};
use raytracer_lib::{
    ambient::gradient_light::GradientLight,
    camera::{CamProgressCb, Camera},
    hits::hittable_list::HittableList,
    materials::normal::Normal,
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
        Camera::new(400, 16.0 / 9.0, 100, 10)
    }

    fn render(&self, cam: &Camera, progresscb: CamProgressCb) -> Vec<Vec<Colour>> {
        // Render
        cam.render(&self.world, &self.ambience, progresscb)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Materials
    let material = Normal::new();

    // World
    let mut world = HittableList::new();

    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, &material));

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        &material,
    ));

    // Call common bin main
    bin_main(State {
        world,
        ambience: GradientLight::new(Colour::new(1.0, 1.0, 1.0), Colour::new(0.5, 0.7, 1.0)),
    })
}
