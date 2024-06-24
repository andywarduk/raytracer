use minifb::{Key, Window, WindowOptions};
use std::{
    error::Error,
    path::{Path, PathBuf},
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
            render_to_window(cam, state)?;
        }
    }

    Ok(())
}

fn render_to_image(cam: Camera, state: impl Renderer, file: &Path) -> Result<(), Box<dyn Error>> {
    let image = state.render(
        &cam,
        Some(|l, h| {
            // Print progress
            eprint!("\r{} / {}  ", l, h);
        }),
    );

    eprintln!("\nDone!");

    save_image(image, &file)?;

    Ok(())
}

/// Renders the scene to a PNG
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

fn render_to_window(mut cam: Camera, state: impl Renderer) -> Result<(), Box<dyn Error>> {
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

        // Get next frame
        let next_frame = state.render(&cam, None);

        // Merge with current frame
        for (fl, nl) in frame.iter_mut().zip(next_frame.iter()) {
            for (fc, nc) in fl.iter_mut().zip(nl.iter()) {
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
