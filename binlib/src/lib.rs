use minifb::{InputCallback, Key, Window, WindowOptions};
use std::{
    cell::RefCell,
    error::Error,
    path::{Path, PathBuf},
    rc::Rc,
};

use clap::Parser;
use raytracer_lib::{
    camera::{CamProgressCb, Camera},
    triple::Colour,
};

pub trait Renderer {
    fn default_camera(&self) -> Camera;
    fn render(&self, cam: &Camera, progresscb: CamProgressCb) -> Vec<Vec<Colour>>;
}

#[derive(Parser, Default)]
#[clap(author, version, about)]
struct Args {
    /// Output file
    #[clap(short = 'o', long = "output")]
    output: Option<PathBuf>,

    /// Verbose output
    #[clap(short = 'v', long = "verbose")]
    verbose: bool,
}

pub fn bin_main(state: impl Renderer) -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Get default camera
    let cam = state.default_camera();

    match args.output {
        Some(file) => {
            // Output to image
            render_to_image(cam, state, &file)?;
        }
        None => {
            // Output to window
            render_to_window(cam, state)?;
        }
    }

    Ok(())
}

fn render_to_image(cam: Camera, state: impl Renderer, file: &Path) -> Result<(), Box<dyn Error>> {
    dump_camera_parameters(&cam, true);

    let image = state.render(
        &cam,
        Some(|l, h| {
            // Print progress
            eprint!("\r{} / {}  ", l, h);
        }),
    );

    eprintln!("\nDone!");

    save_image(image, file)?;

    Ok(())
}

/// Renders the scene to an image
pub fn save_image(image: Vec<Vec<Colour>>, output: &Path) -> Result<(), Box<dyn Error>> {
    let h = image.len();
    let w = image[0].len();

    // Create image buffer
    let mut imgbuf = image::ImageBuffer::new(w as u32, h as u32);

    // For each line...
    (0..image.len()).for_each(|j| {
        let line = &image[j];

        // For each column...
        (0..line.len()).for_each(|i| {
            // Convert to RGB with gamma correction
            let (r, g, b) = line[i].to_rgb_gamma();

            // Add to image data buffer
            let pixel = imgbuf.get_pixel_mut(i as u32, j as u32);
            *pixel = image::Rgb([r, g, b]);
        });
    });

    // Save image
    imgbuf.save(output)?;

    Ok(())
}

type KeyVec = Rc<RefCell<Vec<u32>>>;

struct Input {
    keys: KeyVec,
}

impl InputCallback for Input {
    /// Will be called every time a character key is pressed
    fn add_char(&mut self, uni_char: u32) {
        self.keys.borrow_mut().push(uni_char);
    }
}

fn render_to_window(mut cam: Camera, state: impl Renderer) -> Result<(), Box<dyn Error>> {
    dump_camera_parameters(&cam, false);

    let (w, h) = cam.dimensions();

    let mut output_buffer: Vec<u32> = vec![0; w as usize * h as usize];

    // Set samples per pixel to 1
    cam.set_samples_per_pixel(1);

    // Create window
    let mut window = Window::new(
        "Rendering - ESC to exit",
        w as usize,
        h as usize,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )?;

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    // Set up key buffer
    let keys = KeyVec::new(RefCell::new(Vec::new()));
    window.set_input_callback(Box::new(Input { keys: keys.clone() }));

    // Render first frame
    let mut frame = state.render(&cam, None);
    let mut frame_no = 1;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Build output buffer
        let mut outelem = 0;

        for l in frame.iter() {
            for c in l {
                let (r, g, b) = c.to_rgb_gamma();
                output_buffer[outelem] = ((r as u32) << 16) + ((g as u32) << 8) + b as u32;
                outelem += 1;
            }
        }

        // Display output buffer
        window.set_title(&format!("Rendering (pass {frame_no}) - ESC to exit"));
        window.update_with_buffer(&output_buffer, w as usize, h as usize)?;

        // Process key presses
        let mut keys = keys.borrow_mut();

        if process_keys(&mut cam, &mut keys) {
            dump_camera_parameters(&cam, false);

            // Clear frame buffer
            for fl in frame.iter_mut() {
                for fc in fl.iter_mut() {
                    *fc = Colour::default();
                }
            }

            // Reset frame count
            frame_no = 0;
        }

        // Get next frame
        let next_frame = state.render(&cam, None);

        // Merge with current frame
        for (fl, nl) in frame.iter_mut().zip(next_frame.into_iter()) {
            for (fc, nc) in fl.iter_mut().zip(nl.into_iter()) {
                let cnt: f64 = frame_no as f64;
                *fc *= cnt;
                *fc += nc;
                *fc /= cnt + 1.0;
            }
        }

        frame_no += 1;
    }

    Ok(())
}

fn process_keys(cam: &mut Camera, keys: &mut Vec<u32>) -> bool {
    let mut clear = false;

    for t in keys.iter() {
        match char::from_u32(*t) {
            // Vertical FOV
            Some('f') => adjust_vfov(cam, 1.0, &mut clear),
            Some('F') => adjust_vfov(cam, 5.0, &mut clear),
            Some('v') => adjust_vfov(cam, -1.0, &mut clear),
            Some('V') => adjust_vfov(cam, -5.0, &mut clear),
            // Focus
            Some('g') => adjust_focus(cam, 0.1, 0.0, &mut clear),
            Some('G') => adjust_focus(cam, 1.0, 0.0, &mut clear),
            Some('b') => adjust_focus(cam, -0.1, 0.0, &mut clear),
            Some('B') => adjust_focus(cam, -1.0, 0.0, &mut clear),
            Some('h') => adjust_focus(cam, 0.0, 0.1, &mut clear),
            Some('H') => adjust_focus(cam, 0.0, 1.0, &mut clear),
            Some('n') => adjust_focus(cam, 0.0, -0.1, &mut clear),
            Some('N') => adjust_focus(cam, 0.0, -1.0, &mut clear),
            // Depth
            Some('z') => adjust_depth(cam, -1, &mut clear),
            Some('Z') => adjust_depth(cam, -5, &mut clear),
            Some('x') => adjust_depth(cam, 1, &mut clear),
            Some('X') => adjust_depth(cam, 5, &mut clear),
            // Catch others
            _ => (),
        }
    }

    keys.clear();

    clear
}

fn adjust_vfov(cam: &mut Camera, degrees: f64, clear: &mut bool) {
    let vfov = cam.vfov();
    let new_vfov = (vfov + degrees).max(0.0).min(180.0);

    if new_vfov != vfov {
        cam.set_vfov(new_vfov);
        *clear = true;
    }
}

fn adjust_focus(cam: &mut Camera, degrees: f64, dist: f64, clear: &mut bool) {
    let (defocus_angle, focus_distance) = cam.focus();

    let new_defocus_angle = (defocus_angle + degrees).max(0.0).min(180.0);
    let new_focus_distance = (focus_distance + dist).max(0.0);

    if new_defocus_angle != defocus_angle || new_focus_distance != focus_distance {
        cam.set_focus(new_defocus_angle, new_focus_distance);
        *clear = true;
    }
}

fn adjust_depth(cam: &mut Camera, adjustment: i64, clear: &mut bool) {
    let depth = cam.max_depth();

    let new_depth = depth.saturating_add_signed(adjustment).max(1);

    if new_depth != depth {
        cam.set_max_depth(new_depth);
        *clear = true;
    }
}

fn dump_camera_parameters(cam: &Camera, show_samples: bool) {
    println!("Camera parameters:");

    let (w, h) = cam.dimensions();
    let (look_from, look_at, vup) = cam.view();
    let vfov = cam.vfov();
    let (defocus_angle, focus_dist) = cam.focus();
    let time_span = cam.time_span();
    let samples_per_pixel = cam.samples_per_pixel();
    let max_depth = cam.max_depth();

    let view_vec = look_from.vec_to(&look_at);

    println!("          Image dimensions : {w} x {h}");
    println!("                 Look from : {look_from}");
    println!("                   Look to : {look_at}");
    println!("            -> Look vector : {view_vec}, distance {}", view_vec.length());
    println!("                        Up : {vup}");
    println!("  Vertical field of vision : {vfov}°");
    println!("             Defocus angle : {defocus_angle}°");
    println!("            Focus distance : {focus_dist}");
    println!("                 Time span : {time_span}");
    println!("            Maxiumum depth : {max_depth}");

    if show_samples {
        println!("         Samples per pixel : {samples_per_pixel}");
    }
}
