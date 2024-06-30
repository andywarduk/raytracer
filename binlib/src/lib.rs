use image::render_to_image;
use std::{error::Error, path::PathBuf};
use window::render_to_window;

use clap::Parser;

mod image;
mod parms;
mod window;

pub use image::save_image;
pub use parms::MainParms;

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

pub fn bin_main(mut parms: MainParms) -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args = Args::parse();

    // Set image dimensions if overridden
    match (args.width, args.height) {
        (Some(w), None) => parms.cam.set_width(w as u64),
        (None, Some(h)) => parms.cam.set_height(h as u64),
        (Some(w), Some(h)) => parms.cam.set_dimensions(w as u64, h as u64),
        _ => (),
    }

    // Set gamma correction
    parms.set_gamma(args.gamma);

    // Output to image?
    match args.output {
        Some(output) => {
            // Output to image
            render_to_image(parms, &output)?;
        }
        None => {
            // Output to window
            render_to_window(parms)?;
        }
    }

    Ok(())
}
