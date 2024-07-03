use image::io::Reader as ImageReader;
use std::path::{Path, PathBuf};

use crate::{
    float::*,
    triple::{Colour, Point3},
};

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

const COLOUR_SCALE: FltPrim = 1.0 / 255.0;

impl Texture for Image {
    fn value(&self, u: Flt, v: Flt, _p: &Point3) -> Colour {
        // Clamp input texture coordinates to [0,1] x [1,0]
        let uc = clamp(u, flt(0.0), flt(1.0));
        let vc = clamp(flt(1.0) - v, flt(0.0), flt(1.0));

        let x = (FltPrim::from(uc) * (self.width - 1) as FltPrim) as usize;
        let y = (FltPrim::from(vc) * (self.height - 1) as FltPrim) as usize;

        let idx = (y * (self.width as usize * 3)) + (x * 3);

        let (r, g, b) = (self.map[idx], self.map[idx + 1], self.map[idx + 2]);

        Colour::new(
            r as FltPrim * COLOUR_SCALE,
            g as FltPrim * COLOUR_SCALE,
            b as FltPrim * COLOUR_SCALE,
        )
    }
}
