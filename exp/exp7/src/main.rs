use std::error::Error;

use binlib::{bin_main, MainParms};
use raytracer_lib::{
    ambient::ambient_light::AmbientLight,
    camera::Camera,
    float::*,
    hits::{bvh::BvhNode, hittable::Hittable, hittable_list::HittableList},
    materials::{lambertian::Lambertian, material::MatRef},
    shapes::sphere::Sphere,
    triple::{Colour, Point3, Vec3},
};

const COUNT: u64 = 8;
const RADIUS: FltPrim = 0.3;

fn main() -> Result<(), Box<dyn Error>> {
    // Materials
    let ground_material = Lambertian::new_with_colour(Colour::new(0.5, 0.5, 0.5));

    // Spheres
    let mut spheres = HittableList::new();

    // Metal spheres
    for x in 0..COUNT {
        for y in 0..COUNT {
            for z in 0..COUNT {
                let centre = Point3::new(x as FltPrim, y as FltPrim, z as FltPrim);
                let colour = Colour::new_flt(centre.x(), centre.y(), centre.z())
                    / (flt(COUNT as FltPrim - 1.0));

                let material = MatRef::boxed(Lambertian::new_with_colour(colour));

                spheres.add(Sphere::new_with_matref(centre, RADIUS, material));
            }
        }
    }

    let spheres_bbox = spheres.bounding_box().clone();

    // Create world
    let mut world = HittableList::new();

    // Convert spheres to bvh
    world.add(BvhNode::new(spheres));

    // Ground
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0 - RADIUS,
        &ground_material,
    ));

    // Camera
    let mut cam = Camera::new(1200, 1.0, 500, 50);

    cam.set_view(
        Point3::new(
            COUNT as FltPrim * 1.4,
            COUNT as FltPrim * 1.5,
            COUNT as FltPrim * 1.6,
        ),
        Point3::new(
            COUNT as FltPrim / 2.0,
            COUNT as FltPrim * 0.55,
            COUNT as FltPrim / 2.0,
        ),
        Vec3::new(0.0, 1.0, 0.0),
    );

    cam.set_vfov(30.0);

    cam.set_focus(0.6, 10.0);

    // Set up parameters
    let mut parms = MainParms::new_ambience(cam, world, AmbientLight::new(Colour::new_white()));

    // Set main bounding box to the spheres
    parms.set_main_bbox(spheres_bbox);

    // Call common bin main
    bin_main(parms)
}
