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
    triple::{Colour, Vec3},
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

    const SMALL_MOVE: f64 = 10.0;
    const BIG_MOVE: f64 = 100.0;

    for t in keys.iter() {
        match char::from_u32(*t) {
            // Vertical FOV
            Some('z') => adjust_vfov(cam, -1.0, &mut clear),
            Some('Z') => adjust_vfov(cam, -5.0, &mut clear),
            Some('x') => adjust_vfov(cam, 1.0, &mut clear),
            Some('X') => adjust_vfov(cam, 5.0, &mut clear),
            // Focus
            Some('c') => adjust_focus(cam, -0.1, 0.0, &mut clear),
            Some('C') => adjust_focus(cam, -1.0, 0.0, &mut clear),
            Some('v') => adjust_focus(cam, 0.1, 0.0, &mut clear),
            Some('V') => adjust_focus(cam, 1.0, 0.0, &mut clear),
            Some('b') => adjust_focus(cam, 0.0, -0.1, &mut clear),
            Some('B') => adjust_focus(cam, 0.0, -1.0, &mut clear),
            Some('n') => adjust_focus(cam, 0.0, 0.1, &mut clear),
            Some('N') => adjust_focus(cam, 0.0, 1.0, &mut clear),
            // Depth
            Some('[') => adjust_depth(cam, -1, &mut clear),
            Some('{') => adjust_depth(cam, -5, &mut clear),
            Some(']') => adjust_depth(cam, 1, &mut clear),
            Some('}') => adjust_depth(cam, 5, &mut clear),
            // Camera position
            Some('q') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, -SMALL_MOVE),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('Q') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, -BIG_MOVE),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('w') => adjust_view(
                cam,
                Vec3::new(0.0, SMALL_MOVE, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('W') => adjust_view(
                cam,
                Vec3::new(0.0, BIG_MOVE, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('e') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, SMALL_MOVE),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('E') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, BIG_MOVE),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('a') => adjust_view(
                cam,
                Vec3::new(-SMALL_MOVE, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('A') => adjust_view(
                cam,
                Vec3::new(-BIG_MOVE, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('s') => adjust_view(
                cam,
                Vec3::new(0.0, -SMALL_MOVE, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('S') => adjust_view(
                cam,
                Vec3::new(0.0, -BIG_MOVE, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('d') => adjust_view(
                cam,
                Vec3::new(SMALL_MOVE, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('D') => adjust_view(
                cam,
                Vec3::new(BIG_MOVE, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            // Camera target
            Some('r') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, -SMALL_MOVE),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('R') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, -BIG_MOVE),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('t') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, SMALL_MOVE, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('T') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, BIG_MOVE, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('y') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, SMALL_MOVE),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('Y') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, BIG_MOVE),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('f') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(-SMALL_MOVE, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('F') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(-BIG_MOVE, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('g') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, -SMALL_MOVE, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('G') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, -BIG_MOVE, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('h') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(SMALL_MOVE, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('H') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(BIG_MOVE, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            // Camera orienation
            Some('u') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, -SMALL_MOVE),
                &mut clear,
            ),
            Some('U') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, -BIG_MOVE),
                &mut clear,
            ),
            Some('i') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, SMALL_MOVE, 0.0),
                &mut clear,
            ),
            Some('I') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, BIG_MOVE, 0.0),
                &mut clear,
            ),
            Some('o') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, SMALL_MOVE),
                &mut clear,
            ),
            Some('O') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, BIG_MOVE),
                &mut clear,
            ),
            Some('j') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(-SMALL_MOVE, 0.0, 0.0),
                &mut clear,
            ),
            Some('J') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(-BIG_MOVE, 0.0, 0.0),
                &mut clear,
            ),
            Some('k') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, -SMALL_MOVE, 0.0),
                &mut clear,
            ),
            Some('K') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, -BIG_MOVE, 0.0),
                &mut clear,
            ),
            Some('l') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(SMALL_MOVE, 0.0, 0.0),
                &mut clear,
            ),
            Some('L') => adjust_view(
                cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(BIG_MOVE, 0.0, 0.0),
                &mut clear,
            ),
            Some('?') => {
                println!("Key bindings:");
                println!(" Camera position:");
                println!("   Q/q W/w E/e => Z- Y+ Z+");
                println!("   A/a S/s D/d => X- Y- X+ ");
                println!(" Camera target:");
                println!("   R/r T/t Y/y => Z- Y+ Z+");
                println!("   F/f G/g H/h => X- Y- X+ ");
                println!(" Camera orientation:");
                println!("   U/u I/i O/o => Z- Y+ Z+");
                println!("   J/j K/k L/l => X- Y- X+ ");
                println!(" Vertical FOV:");
                println!("   z/Z x/X => Decrease / increase vertical field of vision angle");
                println!(" Focus:");
                println!("   c/C x/X => Decrease / increase defocus angle");
                println!("   v/V b/B => Decrease / increase focus distance");
                println!(" Depth:");
                println!("   [/{{ ]/}} => Decrease / increase ray depth (number of bounces)");
            }
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

fn adjust_view(
    cam: &mut Camera,
    look_from_adj: Vec3,
    look_at_adj: Vec3,
    vup_adj: Vec3,
    clear: &mut bool,
) {
    let (look_from, look_at, vup) = cam.view();

    let new_look_from = look_from + look_from_adj;
    let new_look_at = look_at + look_at_adj;
    let new_vup = vup + vup_adj;

    cam.set_view(new_look_from, new_look_at, new_vup);
    *clear = true;
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
    println!(
        "            -> Look vector : {view_vec}, distance {}",
        view_vec.length()
    );
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
