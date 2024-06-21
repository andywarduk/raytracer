use std::sync::Arc;

use rand::{thread_rng, Rng};
use raytracer_lib::bvh::BvhNode;
use raytracer_lib::camera::Camera;
use raytracer_lib::colour::Colour;
use raytracer_lib::hittable_list::HittableList;
use raytracer_lib::materials::dielectric::Dielectric;
use raytracer_lib::materials::lambertian::Lambertian;
use raytracer_lib::materials::material::Material;
use raytracer_lib::materials::metal::Metal;
use raytracer_lib::shapes::sphere::Sphere;
use raytracer_lib::textures::checker::Checker;
use raytracer_lib::vec3::{Point3, Vec3};

fn main() {
    let mut rng = thread_rng();

    let mut world = HittableList::new();

    let checker = Arc::new(Checker::new_with_colours(0.32, Colour::new(0.2, 0.3, 0.1), Colour::new(0.9, 0.9, 0.9)));
    let ground_material = Arc::new(Lambertian::new_with_texture(checker));

    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -10..=10 {
        for b in -10..=10 {
            let choose_mat = rng.gen_range(0.0..1.0);

            let center0 = Point3::new(
                a as f64 + rng.gen_range(-0.4..0.4),
                0.2,
                b as f64 + rng.gen_range(-0.4..0.4),
            );

            let mut center1 = center0.clone();

            if (&center0 - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Colour::new_random(&mut rng) * Colour::new_random(&mut rng);
                    center1 += Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    Arc::new(Lambertian::new_with_colour(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Colour::new_random_clamped(&mut rng, 0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Arc::new(Dielectric::new(1.5))
                };

                world.add(Sphere::new_moving(center0, center1, 0.2, sphere_material));
            }
        }
    }

    let material2 = Arc::new(Lambertian::new_with_colour(Colour::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material3 = Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

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

    // Render
    cam.render(&bvh_world);
}
