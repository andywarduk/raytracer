use std::error::Error;

use binlib::{bin_main, Renderer};

use raytracer_lib::{
    ambient::gradient_light::GradientLight,
    camera::{CamProgressCb, Camera},
    hits::hittable_list::HittableList,
    materials::lambertian::Lambertian,
    shapes::quad::Quad,
    triple::{Colour, Point3, Vec3},
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
        let mut cam = Camera::new(400, 1.0, 100, 50);

        cam.set_vfov(80.0);

        // Render
        cam.set_view(
            Point3::new(0.0, 0.0, 9.0),
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        cam
    }

    fn render(&self, cam: &Camera, progresscb: CamProgressCb) -> Vec<Vec<Colour>> {
        // Render
        cam.render(&self.world, &self.ambience, progresscb)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Materials
    let left_red = Lambertian::new_with_colour(Colour::new(1.0, 0.2, 0.2));
    let back_green = Lambertian::new_with_colour(Colour::new(0.2, 1.0, 0.2));
    let right_blue = Lambertian::new_with_colour(Colour::new(0.2, 0.2, 1.0));
    let upper_orange = Lambertian::new_with_colour(Colour::new(1.0, 0.5, 0.0));
    let lower_teal = Lambertian::new_with_colour(Colour::new(0.2, 0.8, 0.8));

    // Objects
    let mut world = HittableList::new();

    world.add(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        &left_red,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        &back_green,
    ));
    world.add(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        &right_blue,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        &upper_orange,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        &lower_teal,
    ));

    // Call common bin main
    bin_main(State {
        world,
        ambience: GradientLight::new(Colour::new(1.0, 1.0, 1.0), Colour::new(0.5, 0.7, 1.0)),
    })
}
