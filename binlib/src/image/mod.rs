use std::io::{stdout, Write};
use std::time::Instant;
use std::{error::Error, path::Path};

use atty::Stream;
use raytracer_lib::{gamma::Gamma, triple::Colour};
use simple_process_stats::ProcessStats;

use crate::MainParms;

pub(super) fn render_to_image(state: MainParms, output: &Path) -> Result<(), Box<dyn Error>> {
    // Output camera parameters
    state.dump_camera_parameters(true);

    // Start time
    let start = Instant::now();

    // Render the image
    let image = state.cam.render(
        &state.world,
        &*state.ambience,
        Some(|l, h| {
            // Print progress
            let tty = atty::is(Stream::Stdout);
            let msg = format!("{} / {} ({}%)", l, h, (l * 100) / h);

            let mut lock = stdout().lock();
            lock.write_all(msg.as_bytes()).unwrap();
            if tty {
                lock.write_all("\r".as_bytes()).unwrap();
            } else {
                lock.write_all("\n".as_bytes()).unwrap();
            }
            lock.flush().unwrap();
            drop(lock);
        }),
    );

    println!("Render completed in {:?}", start.elapsed());

    // Save the image
    save_image(image, output, &state.gamma)?;

    println!("Written to {}", output.display());

    // Print CPU statistics
    match ProcessStats::get() {
        Ok(stats) => {
            println!(
                "CPU time: {:?} user, {:?} kernel",
                stats.cpu_time_user, stats.cpu_time_kernel
            );
        }
        Err(e) => println!("Failed to get process stats ({e})"),
    }

    Ok(())
}

/// Saves an image vector to a file
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
