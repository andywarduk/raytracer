use minifb::{InputCallback, Key, ScaleMode, Window, WindowOptions};
use std::{
    cell::RefCell,
    error::Error,
    path::{Path, PathBuf},
    rc::Rc,
};

use clap::Parser;
use raytracer_lib::{
    ambient::ambience::Ambience,
    camera::Camera,
    gamma::Gamma,
    hits::{hittable::Hittable, hittable_list::HittableList},
    triple::{Colour, Vec3},
};

#[derive(Parser, Default)]
#[clap(author, version, about)]
struct Args {
    /// Output file
    #[clap(short = 'o', long = "output")]
    output: Option<PathBuf>,

    /// Image width
    #[clap(short = 'x', long = "width")]
    width: Option<u16>,

    /// Image height
    #[clap(short = 'y', long = "height")]
    height: Option<u16>,

    /// No gamma correction
    #[clap(short = 'g', long = "gamma", default_value_t = 0.0)]
    gamma: f64,
}

struct State<'a> {
    args: Args,
    gamma: Gamma,
    cam: Camera,
    world: HittableList<'a>,
    ambience: Box<dyn Ambience>,
}

pub fn bin_main(
    mut cam: Camera,
    world: HittableList,
    ambience: impl Ambience + 'static,
) -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args = Args::parse();

    // Set image dimensions if overridden
    match (args.width, args.height) {
        (Some(w), None) => cam.set_width(w as u64),
        (None, Some(h)) => cam.set_height(h as u64),
        (Some(w), Some(h)) => cam.set_dimensions(w as u64, h as u64),
        _ => (),
    }

    // Output to image?
    let image = args.output.is_some();

    // Gamma correction
    let gamma = Gamma::new(args.gamma);

    // Build state
    let state = State {
        args,
        gamma,
        cam,
        world,
        ambience: Box::new(ambience),
    };

    if image {
        // Output to image
        render_to_image(state)?;
    } else {
        // Output to window
        render_to_window(state)?;
    }

    Ok(())
}

fn render_to_image(state: State) -> Result<(), Box<dyn Error>> {
    // Get output
    let output = state.args.output.unwrap();

    // Output camera parameters
    dump_camera_parameters(&state.cam, true);

    // Render the image
    let image = state.cam.render(
        &state.world,
        &*state.ambience,
        Some(|l, h| {
            // Print progress
            eprint!("\r{} / {}  ", l, h);
        }),
    );

    eprintln!("\nDone!");

    // Save the image
    save_image(image, &output, &state.gamma)?;

    Ok(())
}

/// Renders the scene to an image (callable from outside)
pub fn save_image(
    image: Vec<Vec<Colour>>,
    output: &Path,
    gamma: &Gamma,
) -> Result<(), Box<dyn Error>> {
    let h = image.len();
    let w = image[0].len();

    // Create output image buffer
    let mut imgbuf = image::ImageBuffer::new(w as u32, h as u32);

    // For each line...
    (0..image.len()).for_each(|j| {
        let line = &image[j];

        // For each column...
        (0..line.len()).for_each(|i| {
            // Convert to RGB
            let (r, g, b) = line[i].to_rgb(gamma);

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

struct WinState {
    move_delta_big: f64,
    move_delta_small: f64,
}

fn render_to_window(mut state: State) -> Result<(), Box<dyn Error>> {
    // Set up window state
    let mut winstate = WinState {
        move_delta_big: 100.0,
        move_delta_small: 10.0,
    };

    // Any objects in the world?
    if state.world.length() > 0 {
        // Get bounding box for the world
        let bbox = state.world.bounding_box();

        println!("World bounding box:");
        println!("  {bbox}");

        // Find longest axis in the bounding box
        let axis = bbox.longest_axis();

        // Set up movement parameters - 1/50th of longest axis for small, 1/5 for big
        winstate.move_delta_small =
            ((bbox.ranges[axis].end - bbox.ranges[axis].start) / 50.0).max(0.1);
        winstate.move_delta_big = winstate.move_delta_small * 10.0;
    }

    // Output camera parameters
    dump_camera_parameters(&state.cam, true);

    // Print help
    print_help();

    // Get image dimensions
    let (w, h) = state.cam.dimensions();

    // Create buffer for the image (0RGB u32)
    let mut output_buffer: Vec<u32> = vec![0; w as usize * h as usize];

    // Use samples per pixel as the max frame number
    let max_frame = state.cam.samples_per_pixel();

    // Set samples per pixel to 1
    state.cam.set_samples_per_pixel(1);

    // Create the window
    let mut window = Window::new(
        "Rendering - ESC to exit",
        w as usize,
        h as usize,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Stretch,
            ..WindowOptions::default()
        },
    )?;

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    // Set up key buffer
    let keys = KeyVec::new(RefCell::new(Vec::new()));
    window.set_input_callback(Box::new(Input { keys: keys.clone() }));

    // Render the first frame
    let mut frame = state.cam.render(&state.world, &*state.ambience, None);
    let mut frame_no = 1;
    let mut iterating = true;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if iterating {
            // Build output buffer from the renderer frame
            let mut outelem = 0;

            for l in frame.iter() {
                for c in l {
                    let (r, g, b) = c.to_rgb(&state.gamma);

                    output_buffer[outelem] = ((r as u32) << 16) + ((g as u32) << 8) + b as u32;
                    outelem += 1;
                }
            }

            // Display the output buffer
            window.set_title(&format!("Rendering (pass {frame_no}) - ESC to exit"));
            window.update_with_buffer(&output_buffer, w as usize, h as usize)?;
        } else {
            // No buffer update
            window.update();
        }

        // Process key presses
        let mut keys = keys.borrow_mut();

        if process_keys(&mut state, &winstate, &mut keys) {
            // Print new camera parameters
            dump_camera_parameters(&state.cam, false);

            // Clear frame buffer
            for fl in frame.iter_mut() {
                for fc in fl.iter_mut() {
                    *fc = Colour::default();
                }
            }

            // Reset frame count and iterating flag
            frame_no = 0;
            iterating = true;
        }

        if iterating {
            // Get the next frame
            let next_frame = state.cam.render(&state.world, &*state.ambience, None);

            // Merge with the current frame
            for (fl, nl) in frame.iter_mut().zip(next_frame.into_iter()) {
                for (fc, nc) in fl.iter_mut().zip(nl.into_iter()) {
                    // Rolling average
                    let cnt: f64 = frame_no as f64;
                    *fc *= cnt;
                    *fc += nc;
                    *fc /= cnt + 1.0;
                }
            }

            // Increment frame number
            frame_no += 1;

            // Reached the end?
            if frame_no >= max_frame {
                iterating = false;
                window.set_title("Render finished - ESC to exit");
            }
        }
    }

    Ok(())
}

fn dump_camera_parameters(cam: &Camera, show_samples: bool) {
    println!("Camera parameters:");

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

fn process_keys(state: &mut State, winstate: &WinState, keys: &mut Vec<u32>) -> bool {
    let mut clear = false;

    for t in keys.iter() {
        match char::from_u32(*t) {
            // Vertical FOV
            Some('z') => adjust_vfov(&mut state.cam, -1.0, &mut clear),
            Some('Z') => adjust_vfov(&mut state.cam, -5.0, &mut clear),
            Some('x') => adjust_vfov(&mut state.cam, 1.0, &mut clear),
            Some('X') => adjust_vfov(&mut state.cam, 5.0, &mut clear),
            // Focus
            Some('c') => adjust_focus(&mut state.cam, -0.1, 0.0, &mut clear),
            Some('C') => adjust_focus(&mut state.cam, -1.0, 0.0, &mut clear),
            Some('v') => adjust_focus(&mut state.cam, 0.1, 0.0, &mut clear),
            Some('V') => adjust_focus(&mut state.cam, 1.0, 0.0, &mut clear),
            Some('b') => adjust_focus(&mut state.cam, 0.0, -1.0, &mut clear),
            Some('B') => adjust_focus(&mut state.cam, 0.0, -10.0, &mut clear),
            Some('n') => adjust_focus(&mut state.cam, 0.0, 1.0, &mut clear),
            Some('N') => adjust_focus(&mut state.cam, 0.0, 10.0, &mut clear),
            // Depth
            Some('[') => adjust_depth(&mut state.cam, -1, &mut clear),
            Some('{') => adjust_depth(&mut state.cam, -5, &mut clear),
            Some(']') => adjust_depth(&mut state.cam, 1, &mut clear),
            Some('}') => adjust_depth(&mut state.cam, 5, &mut clear),
            // Camera position
            Some('q') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, -winstate.move_delta_small),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('Q') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, -winstate.move_delta_big),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('w') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, winstate.move_delta_small, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('W') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, winstate.move_delta_big, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('e') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, winstate.move_delta_small),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('E') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, winstate.move_delta_big),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('a') => adjust_view(
                &mut state.cam,
                Vec3::new(-winstate.move_delta_small, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('A') => adjust_view(
                &mut state.cam,
                Vec3::new(-winstate.move_delta_big, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('s') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, -winstate.move_delta_small, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('S') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, -winstate.move_delta_big, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('d') => adjust_view(
                &mut state.cam,
                Vec3::new(winstate.move_delta_small, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('D') => adjust_view(
                &mut state.cam,
                Vec3::new(winstate.move_delta_big, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            // Camera target
            Some('r') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, -winstate.move_delta_small),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('R') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, -winstate.move_delta_big),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('t') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, winstate.move_delta_small, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('T') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, winstate.move_delta_big, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('y') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, winstate.move_delta_small),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('Y') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, winstate.move_delta_big),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('f') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(-winstate.move_delta_small, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('F') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(-winstate.move_delta_big, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('g') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, -winstate.move_delta_small, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('G') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, -winstate.move_delta_big, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('h') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(winstate.move_delta_small, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('H') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(winstate.move_delta_big, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &mut clear,
            ),
            // Camera orienation
            Some('u') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, -1.0),
                &mut clear,
            ),
            Some('U') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, -10.0),
                &mut clear,
            ),
            Some('i') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                &mut clear,
            ),
            Some('I') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 10.0, 0.0),
                &mut clear,
            ),
            Some('o') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                &mut clear,
            ),
            Some('O') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 10.0),
                &mut clear,
            ),
            Some('j') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(-1.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('J') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(-10.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('k') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, -1.0, 0.0),
                &mut clear,
            ),
            Some('K') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, -10.0, 0.0),
                &mut clear,
            ),
            Some('l') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('L') => adjust_view(
                &mut state.cam,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(10.0, 0.0, 0.0),
                &mut clear,
            ),
            Some('?') => print_help(),
            // Catch others
            _ => (),
        }
    }

    keys.clear();

    clear
}

fn print_help() {
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
    println!("   c/C v/V => Decrease / increase defocus angle");
    println!("   b/B n/N => Decrease / increase focus distance");
    println!(" Depth:");
    println!("   [/{{ ]/}} => Decrease / increase ray depth (number of bounces)");
}

fn adjust_vfov(cam: &mut Camera, degrees: f64, clear: &mut bool) {
    let vfov = cam.vfov();
    let new_vfov = (vfov + degrees).clamp(0.0, 180.0);

    if new_vfov != vfov {
        cam.set_vfov(new_vfov);
        *clear = true;
    }
}

fn adjust_focus(cam: &mut Camera, degrees: f64, dist: f64, clear: &mut bool) {
    let (defocus_angle, focus_distance) = cam.focus();

    let new_defocus_angle = (defocus_angle + degrees).clamp(0.0, 180.0);
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
