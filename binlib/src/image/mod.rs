use std::{error::Error, path::Path};

use raytracer_lib::{gamma::Gamma, triple::Colour};

use crate::MainParms;

pub(super) fn render_to_image(state: MainParms, output: &Path) -> Result<(), Box<dyn Error>> {
    // Output camera parameters
    state.dump_camera_parameters(true);

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
