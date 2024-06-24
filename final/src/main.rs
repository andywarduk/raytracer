use std::path::Path;

use rand::{thread_rng, Rng};
use raytracer_lib::{
    ambient::ambient_light::AmbientLight,
    camera::Camera,
    hits::{bvh::BvhNode, hittable_list::HittableList},
    materials::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
    },
    shapes::{boxcomp::BoxComp, quad::Quad, sphere::Sphere},
    textures::{image::Image, marble::Marble},
    transforms::{constant_medium::ConstantMedium, rotate_y::RotateY, translate::Translate},
    triple::{Colour, Point3, Vec3},
};

fn main() {
    let mut rng = thread_rng();

    // -- Textures --

    let earth_tex = Image::new_from_file(Path::new("earthmap.jpg"));
    let marble_tex = Marble::new(0.2, 7, 2);

    // -- Materials --

    let ground_cube_mat = Lambertian::new_with_colour(Colour::new(0.48, 0.83, 0.53));
    let light_mat = DiffuseLight::new_with_colour(Colour::new(7.0, 7.0, 7.0));
    let moving_sphere_mat = Lambertian::new_with_colour(Colour::new(0.7, 0.3, 0.1));
    let glass_sphere_mat = Dielectric::new(1.5);
    let metal_sphere_mat = Metal::new(Colour::new(0.8, 0.8, 0.9), 1.0);
    let earth_sphere_mat = Lambertian::new_with_texture(&earth_tex);
    let marble_sphere_mat = Lambertian::new_with_texture(&marble_tex);
    let white_sphere_mat = Lambertian::new_with_colour(Colour::new(0.73, 0.73, 0.73));

    // -- Objects --

    // Floor
    let mut floor_boxes = HittableList::new();

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

            floor_boxes.add(BoxComp::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                &ground_cube_mat,
            ));
        }
    }

    let floor_boxes = BvhNode::new(floor_boxes.into_objects());

    // Light
    let light = Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        &light_mat,
    );

    // Moving sphere
    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = &center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere = Sphere::new_moving(center1, center2, 50.0, &moving_sphere_mat);

    // Glass sphere
    let glass_sphere = Sphere::new(Point3::new(260.0, 150.0, 45.0), 50.0, &glass_sphere_mat);

    // White metal sphere
    let metal_sphere = Sphere::new(Point3::new(0.0, 150.0, 145.0), 50.0, &metal_sphere_mat);

    // Glass sphere containing blue smoke
    let smoke_sphere = Sphere::new(Point3::new(360.0, 150.0, 145.0), 70.0, &glass_sphere_mat);

    let smoke_sphere_boundary =
        Sphere::new(Point3::new(360.0, 150.0, 145.0), 70.0, &glass_sphere_mat);

    let smoke_sphere_medium =
        ConstantMedium::new_with_colour(smoke_sphere_boundary, 0.2, Colour::new(0.2, 0.4, 0.9));

    // Environment white smoke
    let env_smoke_boundary = Sphere::new(Point3::new(0.0, 0.0, 0.0), 5000.0, &glass_sphere_mat);

    let env_smoke =
        ConstantMedium::new_with_colour(env_smoke_boundary, 0.0001, Colour::new(1.0, 1.0, 1.0));

    // Earth sphere
    let earth_sphere = Sphere::new(Point3::new(400.0, 200.0, 400.0), 100.0, &earth_sphere_mat);

    // Marble sphere
    let marble_sphere = Sphere::new(Point3::new(220.0, 280.0, 300.0), 80.0, &marble_sphere_mat);

    // Collection of white spheres in a cube
    let mut white_spheres = HittableList::new();

    for _ in 0..1000 {
        white_spheres.add(Sphere::new(
            Point3::new_random_clamped(&mut rng, 0.0, 165.0),
            10.0,
            &white_sphere_mat,
        ));
    }

    let white_spheres = BvhNode::new(white_spheres.into_objects());
    let white_spheres = RotateY::new(15.0, white_spheres);
    let white_spheres = Translate::new(Vec3::new(-100.0, 270.0, 395.0), white_spheres);

    // -- World --

    let mut world = HittableList::new();

    // Floor
    world.add(floor_boxes);

    // Light
    world.add(light);

    // Moving sphere
    world.add(moving_sphere);

    // Glass sphere
    world.add(glass_sphere);

    // White metal sphere
    world.add(metal_sphere);

    // Glass sphere containing blue smoke
    world.add(smoke_sphere);
    world.add(smoke_sphere_medium);

    // Environment white smoke
    world.add(env_smoke);

    // Earth sphere
    world.add(earth_sphere);

    // Marble sphere
    world.add(marble_sphere);

    // Collection of while spheres in a cube
    world.add(white_spheres);

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

    cam.render_to_png(
        &bvh_world,
        &AmbientLight::new(Colour::default()),
        Path::new("final.png"),
    );
}
