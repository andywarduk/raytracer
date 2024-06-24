use std::error::Error;

use binlib::{bin_main, Renderer};
use raytracer_lib::{
    ambient::gradient_light::GradientLight,
    camera::{CamProgressCb, Camera},
    hits::hittable_list::HittableList,
    materials::{lambertian::Lambertian, material::MatRef, metal::Metal},
    shapes::sphere::Sphere,
    triple::{Colour, Point3, Vec3},
};

const COUNT: u64 = 8;
const RADIUS: f64 = 0.3;

struct State<'a> {
    // World
    world: HittableList<'a>,

    // Ambience
    ambience: GradientLight,
}

impl<'a> Renderer for State<'a> {
    fn default_camera(&self) -> Camera {
        // Camera
        let mut cam = Camera::new(1200, 1.0, 500, 50);

        cam.set_view(
            Point3::new(COUNT as f64 * 1.4, COUNT as f64 * 1.5, COUNT as f64 * 1.6),
            Point3::new(COUNT as f64 / 2.0, COUNT as f64 * 0.55, COUNT as f64 / 2.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        cam.set_vfov(30.0);

        cam.set_focus(0.6, 10.0);

        cam
    }

    fn render(&self, cam: &Camera, progresscb: CamProgressCb) -> Vec<Vec<Colour>> {
        // Render
        cam.render(&self.world, &self.ambience, progresscb)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Materials
    let ground_material = Lambertian::new_with_colour(Colour::new(0.5, 0.5, 0.5));

    // World
    let mut world = HittableList::new();

    // Ground
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0 - RADIUS,
        &ground_material,
    ));

    // Metal spheres
    for x in 0..COUNT {
        for y in 0..COUNT {
            for z in 0..COUNT {
                let centre = Point3::new(x as f64, y as f64, z as f64);
                let colour = Colour::new(centre.x(), centre.y(), centre.z()) / COUNT as f64;

                let material = MatRef::boxed(Metal::new(colour, 0.0));

                world.add(Sphere::new_with_matref(centre, RADIUS, material));
            }
        }
    }

    // Call common bin main
    bin_main(State {
        world,
        ambience: GradientLight::new(Colour::new(1.0, 1.0, 1.0), Colour::new(0.5, 0.7, 1.0)),
    })
}
