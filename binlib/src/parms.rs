use raytracer_lib::{
    ambient::{ambience::Ambience, ambient_light::AmbientLight},
    camera::Camera,
    gamma::Gamma,
    hits::{aabb::Aabb, hittable_list::HittableList},
    triple::Colour,
};

/// Main entry point parameters
pub struct MainParms<'a> {
    /// The camera to use
    pub cam: Camera,
    /// The object list to render
    pub world: HittableList<'a>,
    /// The ambient light to use
    pub ambience: Box<dyn Ambience>,
    /// Gamma correction to use
    pub gamma: Gamma,
    /// The bounding box of the main scene feature
    pub main_bbox: Option<Aabb>,
}

impl<'a> MainParms<'a> {
    pub fn dump_camera_parameters(&self, show_samples: bool) {
        println!("Camera parameters:");

        let cam = &self.cam;

        // Get camera details
        let (w, h) = cam.dimensions();
        let (look_from, look_at, vup) = cam.view();
        let vfov = cam.vfov();
        let (defocus_angle, focus_dist) = cam.focus();
        let time_span = cam.time_span();
        let samples_per_pixel = cam.samples_per_pixel();
        let max_depth = cam.max_depth();

        // Calculate vector from the camera to the point we're looking at
        let view_vec = look_from.vec_to(&look_at);

        // Print the details
        println!("  Image dimensions         : {w} x {h}");
        println!("  Look from                : {look_from}");
        println!("  Look to                  : {look_at}");
        println!(
            "  -> Look vector           : {view_vec}, distance {}",
            view_vec.length()
        );
        println!("  Up                       : {vup}");
        println!("  Vertical field of vision : {vfov}°");
        println!("  Defocus angle            : {defocus_angle}°");
        println!("  Focus distance           : {focus_dist}");
        println!("  Time span                : {time_span}");
        println!("  Maxiumum depth           : {max_depth}");

        if show_samples {
            println!("  Samples per pixel        : {samples_per_pixel}");
        }
    }
}

impl<'a> MainParms<'a> {
    pub fn new(cam: Camera, world: HittableList<'a>) -> Self {
        Self {
            cam,
            gamma: Gamma::new(0.0),
            world,
            ambience: Box::new(AmbientLight::new(Colour::default())),
            main_bbox: None,
        }
    }

    pub fn new_ambience(
        cam: Camera,
        world: HittableList<'a>,
        ambience: impl Ambience + 'static,
    ) -> Self {
        Self {
            cam,
            gamma: Gamma::new(0.0),
            world,
            ambience: Box::new(ambience),
            main_bbox: None,
        }
    }

    pub fn set_gamma(&mut self, factor: f64) {
        self.gamma = Gamma::new(factor)
    }

    pub fn set_main_bbox(&mut self, bbox: Aabb) {
        self.main_bbox = Some(bbox)
    }
}
