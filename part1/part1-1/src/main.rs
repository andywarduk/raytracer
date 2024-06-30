use std::error::Error;

use binlib::{bin_main, MainParms};
use raytracer_lib::{
    ambient::gradient_light::GradientLight, camera::Camera, hits::hittable_list::HittableList,
    triple::Colour,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Call common bin main
    bin_main(MainParms::new_ambience(
        Camera::new(400, 16.0 / 9.0, 100, 10),
        HittableList::new(),
        GradientLight::new(Colour::new(1.0, 1.0, 1.0), Colour::new(0.5, 0.7, 1.0)),
    ))
}
