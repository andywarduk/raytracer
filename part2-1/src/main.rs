use std::error::Error;

use binlib::bin_main;
use rand::{thread_rng, Rng};
use raytracer_lib::{
    ambient::gradient_light::GradientLight,
    camera::Camera,
    hits::{bvh::BvhNode, hittable_list::HittableList},
    materials::{dielectric::Dielectric, lambertian::Lambertian, material::MatRef, metal::Metal},
    shapes::sphere::Sphere,
    triple::{Colour, Point3, Vec3},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();

    // -- Materials --
    let ground_material = Lambertian::new_with_colour(Colour::new(0.5, 0.5, 0.5));

    let material1 = Dielectric::new(1.5);
    let material2 = Lambertian::new_with_colour(Colour::new(0.4, 0.2, 0.1));
    let material3 = Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0);

    // -- Objects --

    let mut world = HittableList::new();

    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        &ground_material,
    ));

    let avoid: Point3 = Point3::new(4.0, 0.2, 0.0);

    for a in -10..=10 {
        for b in -10..=10 {
            let choose_mat = rng.gen_range(0.0..1.0);

            let center0 = Point3::new(
                a as f64 + rng.gen_range(-0.4..0.4),
                0.2,
                b as f64 + rng.gen_range(-0.4..0.4),
            );

            let mut center1 = center0.clone();

            if avoid.vec_to(&center0).length() > 0.9 {
                let sphere_material: MatRef = if choose_mat < 0.8 {
                    // diffuse
                    center1 += Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    let albedo = Colour::new_random(&mut rng) * Colour::new_random(&mut rng);
                    MatRef::boxed(Lambertian::new_with_colour(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Colour::new_random_clamped(&mut rng, 0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    MatRef::boxed(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    MatRef::boxed(Dielectric::new(1.5))
                };

                world.add(Sphere::new_moving_with_matref(
                    center0,
                    center1,
                    0.2,
                    sphere_material,
                ));
            }
        }
    }

    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, &material2));
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, &material1));
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, &material3));

    // Create BVH world view
    let mut bvh_world = HittableList::new();
    bvh_world.add(BvhNode::new(world.into_objects()));

    // Camera
    let mut cam = Camera::new(400, 16.0 / 9.0, 200, 50);

    cam.set_view(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    cam.set_vfov(20.0);

    cam.set_focus(0.6, 10.0);

    cam.set_time_span(1.0);

    // Call common bin main
    bin_main(
        cam,
        bvh_world,
        GradientLight::new(Colour::new(1.0, 1.0, 1.0), Colour::new(0.5, 0.7, 1.0)),
    )
}
