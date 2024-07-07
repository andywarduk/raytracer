//! Ramadan lantern

use std::{error::Error, path::Path};

use binlib::{bin_main, MainParms};
use raytracer_lib::{
    ambient::ambient_light::AmbientLight,
    camera::Camera,
    float::*,
    hits::{hittable::Hittable, hittable_list::HittableList},
    materials::{diffuse_light::DiffuseLight, lambertian::Lambertian, material::Material},
    shapes::{boxcomp::BoxComp, quad::Quad, sphere::Sphere},
    textures::image::Image,
    transforms::constant_medium::ConstantMedium,
    triple::{Colour, Point3, Vec3},
};

fn main() -> Result<(), Box<dyn Error>> {
    // Textures
    let lantern_tex = Image::new_from_file(&Path::new("lantern.png"));

    // Materials
    let light = DiffuseLight::new_with_colour(Colour::new(0.94, 0.73, 0.02));
    let lantern_mat = Lambertian::new_with_texture(&lantern_tex);
    let dummy = Lambertian::new_with_colour(Colour::new(0.0, 1.0, 0.0));
    let black = Lambertian::new_with_colour(Colour::default());

    // World
    let mut world = HittableList::new();

    // Light
    world.add(BoxComp::new(
        Point3::new(-1.999, -2.999, 1.999),
        Point3::new(1.999, 2.999, -1.999),
        &light,
    ));

    let lantern = build_lantern(
        Point3::new(-2.0, -3.0, 2.0),
        Point3::new(2.0, 3.0, -2.0),
        &lantern_mat,
        &black,
    );

    let lantern_bbox = lantern.bounding_box().clone();

    world.add(lantern);

    // Environment white smoke
    let env_smoke_boundary = Sphere::new(Point3::new(0.0, 0.0, 0.0), 100.0, &dummy);

    let env_smoke =
        ConstantMedium::new_with_colour(env_smoke_boundary, 0.02, Colour::new(1.0, 1.0, 1.0));

    world.add(env_smoke);

    // Ambient light
    let ambience = AmbientLight::new(Colour::default());

    // Camera
    let mut cam = Camera::new(800, 1.0, 10000, 40);

    cam.set_view(
        Point3::new(-8.0, 0.0, 8.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    cam.set_vfov(45.0);

    // Call common bin main
    let mut parms = MainParms::new_ambience(cam, world, ambience);

    parms.set_main_bbox(lantern_bbox);

    bin_main(parms)
}

fn build_lantern<'a>(
    a: Point3,
    b: Point3,
    material: &'a dyn Material,
    end: &'a dyn Material,
) -> HittableList<'a> {
    let mut sides = HittableList::new();

    // Construct the two opposite vertices with the minimum and maximum coordinates.
    let min = Point3::new_flt(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new_flt(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new_flt(max.x() - min.x(), flt(0.0), flt(0.0));
    let dy = Vec3::new_flt(flt(0.0), max.y() - min.y(), flt(0.0));
    let dz = Vec3::new_flt(flt(0.0), flt(0.0), max.z() - min.z());

    sides.add(Quad::new(
        Point3::new_flt(min.x(), min.y(), max.z()),
        dx.clone(),
        dy.clone(),
        material,
    )); // front
    sides.add(Quad::new(
        Point3::new_flt(max.x(), min.y(), max.z()),
        -(&dz),
        dy.clone(),
        material,
    )); // right
    sides.add(Quad::new(
        Point3::new_flt(max.x(), min.y(), min.z()),
        -(&dx),
        dy.clone(),
        material,
    )); // back
    sides.add(Quad::new(
        Point3::new_flt(min.x(), min.y(), min.z()),
        dz.clone(),
        dy,
        material,
    )); // left
    sides.add(Quad::new(
        Point3::new_flt(min.x(), max.y(), max.z()),
        dx.clone(),
        -(&dz),
        end,
    )); // top
    sides.add(Quad::new(
        Point3::new_flt(min.x(), min.y(), min.z()),
        dx,
        dz,
        end,
    )); // bottom

    sides
}
