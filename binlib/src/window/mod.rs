use keys::{print_help, process_keys, setup_keys};
use minifb::{Key, ScaleMode, Window, WindowOptions};
use raytracer_lib::{float::*, hits::hittable::Hittable, triple::Colour};
use std::{error::Error, time::Instant};

use crate::MainParms;

mod adjust;
mod keys;

struct WinState {
    move_delta_big: Flt,
    move_delta_small: Flt,
}

#[derive(Default)]
struct RenderState {
    frame_no: u64,
    started: Option<Instant>,
    fps: Flt,
}

impl RenderState {
    fn reset(&mut self) {
        self.frame_no = 0;
        self.started = Some(Instant::now());
    }

    fn frame_finished(&mut self) {
        self.frame_no += 1;

        if let Some(started) = self.started {
            let dur = flt(started.elapsed().as_secs_f64() as FltPrim);

            self.fps = if dur > 0.0 {
                flt(self.frame_no as FltPrim) / dur
            } else {
                flt(0.0)
            };
        }
    }

    fn stop(&mut self) {
        self.started = None;
    }
}

pub(super) fn render_to_window(mut state: MainParms) -> Result<(), Box<dyn Error>> {
    // Print float type
    println!("Float type: {FLOAT_DESC} ({} bytes)", size_of::<FltPrim>());

    // Set up window state
    let mut winstate = WinState {
        move_delta_big: flt(100.0),
        move_delta_small: flt(10.0),
    };

    // Main bounding box
    let mut main_bbox = None;

    // Any objects in the world?
    if state.world.length() > 0 {
        // Get bounding box for the world
        let bbox = state.world.bounding_box();

        println!("World bounding box:");
        println!("  {bbox}");

        main_bbox = Some(bbox.clone());
    }

    // Main bounding box supplied?
    if let Some(bbox) = &state.main_bbox {
        println!("Main feature bounding box:");
        println!("  {bbox}");

        main_bbox = Some(bbox.clone());
    }

    if let Some(bbox) = main_bbox {
        // Find longest axis in the bounding box
        let axis = bbox.longest_axis();

        // Set up movement parameters - 1/50th of longest axis for small, 1/5 for big
        winstate.move_delta_small =
            ((bbox.ranges[axis].end - bbox.ranges[axis].start) / 50.0).max(flt(0.1));
        winstate.move_delta_big = winstate.move_delta_small * 10.0;
    }

    // Output camera parameters
    state.dump_camera_parameters(true);

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

    // Limit to max ~120 fps update rate
    window.set_target_fps(600);

    // Set up key buffer
    let keys = setup_keys(&mut window);

    // State
    let mut render_state = RenderState::default();
    render_state.reset();

    // Render the first frame
    let mut frame = state.cam.render(&state.world, &*state.ambience, None);
    render_state.frame_finished();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if render_state.started.is_some() {
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
            window.set_title(&format!(
                "Pass {} ({}%), {:.2} fps - ESC to exit",
                render_state.frame_no,
                (render_state.frame_no * 100) / max_frame,
                render_state.fps
            ));
            window.update_with_buffer(&output_buffer, w as usize, h as usize)?;
        } else {
            // No buffer update
            window.update();
        }

        // Process key presses
        if process_keys(&mut state, &winstate, &keys) {
            // Print new camera parameters
            state.dump_camera_parameters(false);

            // Clear frame buffer
            for fl in frame.iter_mut() {
                for fc in fl.iter_mut() {
                    *fc = Colour::default();
                }
            }

            // Reset frame count and iterating flag
            render_state.reset();
        }

        if render_state.started.is_some() {
            // Get the next frame
            let next_frame = state.cam.render(&state.world, &*state.ambience, None);

            // Merge with the current frame
            for (fl, nl) in frame.iter_mut().zip(next_frame.into_iter()) {
                for (fc, nc) in fl.iter_mut().zip(nl.into_iter()) {
                    // Rolling average
                    let cnt: Flt = flt(render_state.frame_no as FltPrim);
                    *fc *= cnt;
                    *fc += nc;
                    *fc /= cnt + flt(1.0);
                }
            }

            // Increment frame number
            render_state.frame_finished();

            // Reached the end?
            if render_state.frame_no >= max_frame {
                render_state.stop();
                window.set_title(&format!(
                    "Finished ({:.2} fps) - ESC to exit",
                    render_state.fps
                ));
            }
        }
    }

    Ok(())
}
