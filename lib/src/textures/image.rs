use image::io::Reader as ImageReader;
use std::path::{Path, PathBuf};

use crate::{colour::Colour, vec3::Point3};

use super::texture::Texture;

#[derive(Debug)]
pub struct Image {
    width: u32,
    height: u32,
    map: Vec<u8>,
}

impl Image {
    pub fn new_from_file(file: &Path) -> Self {
        let file_path = if file.exists() {
            PathBuf::from(file)
        } else {
            Self::find_file(file).expect("Unable to find file")
        };

        eprintln!("Loading image {}", file_path.display());

        let img = ImageReader::open(file_path).expect("Unable to open image");
        let dynimg = img.decode().expect("Unable to decode image");
        let width = dynimg.width();
        let height = dynimg.height();
        let map = dynimg.into_rgb8().into_vec();

        Self { width, height, map }
    }

    fn find_file(file: &Path) -> Option<PathBuf> {
        for parent in 0..8 {
            let mut buf = PathBuf::new();

            for _ in 0..parent {
                buf.push("..")
            }

            buf.push("images");
            buf.push(file);

            if buf.is_file() {
                return Some(buf);
            }
        }

        None
    }
}

const COLOUR_SCALE: f64 = 1.0 / 255.0;

impl Texture for Image {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Colour {
        // Clamp input texture coordinates to [0,1] x [1,0]
        let uc = u.clamp(0.0, 1.0);
        let vc = (1.0 - v).clamp(0.0, 1.0);

        let x = (uc * (self.width - 1) as f64) as usize;
        let y = (vc * (self.height - 1) as f64) as usize;

        let idx = (y * (self.width as usize * 3)) + (x * 3);

        let (r, g, b) = (self.map[idx], self.map[idx + 1], self.map[idx + 2]);

        Colour::new(
            r as f64 * COLOUR_SCALE,
            g as f64 * COLOUR_SCALE,
            b as f64 * COLOUR_SCALE,
        )
    }
}
