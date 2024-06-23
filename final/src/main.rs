use std::path::Path;
use std::sync::Arc;

use rand::{thread_rng, Rng};
use raytracer_lib::ambience::ambient_light::AmbientLight;
use raytracer_lib::camera::Camera;
use raytracer_lib::colour::Colour;
use raytracer_lib::hittable::bvh::BvhNode;
use raytracer_lib::hittable::hittable_list::HittableList;
use raytracer_lib::materials::dielectric::Dielectric;
use raytracer_lib::materials::diffuse_light::DiffuseLight;
use raytracer_lib::materials::lambertian::Lambertian;
use raytracer_lib::materials::metal::Metal;
use raytracer_lib::shapes::boxcomp::BoxComp;
use raytracer_lib::shapes::quad::Quad;
use raytracer_lib::shapes::sphere::Sphere;
use raytracer_lib::textures::image::Image;
use raytracer_lib::textures::marble::Marble;
use raytracer_lib::transforms::constant_medium::ConstantMedium;
use raytracer_lib::transforms::rotate_y::RotateY;
use raytracer_lib::transforms::translate::Translate;
use raytracer_lib::vec3::{Point3, Vec3};

fn main() {
    let mut rng = thread_rng();

    // Materials
    let ground = Arc::new(Lambertian::new_with_colour(Colour::new(0.48, 0.83, 0.53)));
    let light = Arc::new(DiffuseLight::new_with_colour(Colour::new(7.0, 7.0, 7.0)));
    let sphere1_material = Arc::new(Lambertian::new_with_colour(Colour::new(0.7, 0.3, 0.1)));
    let sphere2_material = Arc::new(Dielectric::new(1.5));
    let sphere3_material = Arc::new(Metal::new(Colour::new(0.8, 0.8, 0.9), 1.0));
    let sphere4_material = Arc::new(Dielectric::new(1.5));
    let sphere5_material = Arc::new(Dielectric::new(1.5));
    let emat = Arc::new(Lambertian::new_with_texture(Arc::new(
        Image::new_from_file(Path::new("earthmap.jpg")),
    )));
    let pertext = Arc::new(Marble::new(0.2, 7, 2));
    let lambpertext = Arc::new(Lambertian::new_with_texture(pertext));
    let white = Arc::new(Lambertian::new_with_colour(Colour::new(0.73, 0.73, 0.73)));

    // -- Objects --

    // Floor
    let mut world = HittableList::new();

    let mut boxes1 = HittableList::new();

    const BOXES_PER_SIDE: usize = 20;

    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.0;

            let x0 = -1000.0 + ((i as f64) * w);
            let y0 = 0.0;
            let z0 = -1000.0 + ((j as f64) * w);

            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0..101.0);
            let z1 = z0 + w;

            boxes1.add(BoxComp::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }

    let bvh = BvhNode::new(boxes1.into_objects());
    world.add(bvh);

    // Light
    world.add(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    ));

    // Moving sphere
    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = &center1 + Vec3::new(30.0, 0.0, 0.0);
    world.add(Sphere::new_moving(center1, center2, 50.0, sphere1_material));

    // Glass sphere
    world.add(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        sphere2_material,
    ));

    // White metal sphere
    world.add(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        sphere3_material,
    ));

    // Glass sphere containing blue smoke
    let boundary = Sphere::new(Point3::new(360.0, 150.0, 145.0), 70.0, sphere4_material);
    world.add(boundary.clone());
    world.add(ConstantMedium::new_with_colour(
        Arc::new(boundary),
        0.2,
        Colour::new(0.2, 0.4, 0.9),
    ));

    // Environment white smoke
    let boundary = Sphere::new(Point3::new(0.0, 0.0, 0.0), 5000.0, sphere5_material);
    world.add(ConstantMedium::new_with_colour(
        Arc::new(boundary),
        0.0001,
        Colour::new(1.0, 1.0, 1.0),
    ));

    // Earth sphere
    world.add(Sphere::new(Point3::new(400.0, 200.0, 400.0), 100.0, emat));

    // Marble sphere
    world.add(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        lambpertext,
    ));

    // Collection of spheres in a cube
    let mut boxes2 = HittableList::new();

    for _ in 0..1000 {
        boxes2.add(Sphere::new(
            Point3::new_random_clamped(&mut rng, 0.0, 165.0),
            10.0,
            white.clone(),
        ));
    }

    world.add(Translate::new(
        Vec3::new(-100.0, 270.0, 395.0),
        Arc::new(RotateY::new(
            15.0,
            Arc::new(BvhNode::new(boxes2.into_objects())),
        )),
    ));

    // Convert to bvh
    let mut bvh_world = HittableList::new();
    bvh_world.add(BvhNode::new(world.into_objects()));

    // Camera
    let mut cam = Camera::new(800, 1.0, 10_000, 40);

    cam.set_vfov(40.0);

    // Render
    cam.set_view(
        Point3::new(478.0, 278.0, -600.0),
        Point3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    cam.set_time_span(1.0);

    cam.render(
        &bvh_world,
        &AmbientLight::new(Colour::default()),
        Path::new("final.png"),
    );
}
