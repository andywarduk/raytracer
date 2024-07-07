//! Image map texture

use image::io::Reader as ImageReader;
use rand::{rngs::ThreadRng, Rng};
use std::path::{Path, PathBuf};

use crate::{
    float::*,
    triple::{Colour, Point3},
};

use super::texture::Texture;

/// Image map details
#[derive(Debug)]
pub struct Image {
    width: u32,
    height: u32,
    map: Vec<u8>,
    transparency: bool,
}

impl Image {
    /// Create a new image map from an image file
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
        let map = dynimg.into_rgba8().into_vec();

        let transparency = map.iter().skip(3).step_by(4).any(|&x| x != 255);

        Self {
            width,
            height,
            map,
            transparency,
        }
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

    fn map_coords(&self, u: Flt, v: Flt) -> usize {
        // Clamp input texture coordinates to [0,1] x [1,0]
        let uc = clamp(u, flt(0.0), flt(1.0));
        let vc = clamp(flt(1.0) - v, flt(0.0), flt(1.0));

        let x = (FltPrim::from(uc) * (self.width - 1) as FltPrim) as usize;
        let y = (FltPrim::from(vc) * (self.height - 1) as FltPrim) as usize;

        (y * (self.width as usize * 4)) + (x * 4)
    }

    fn get_rgb(&self, u: Flt, v: Flt) -> (u8, u8, u8) {
        let idx = self.map_coords(u, v);

        (self.map[idx], self.map[idx + 1], self.map[idx + 2])
    }

    fn get_alpha(&self, u: Flt, v: Flt) -> u8 {
        let idx = self.map_coords(u, v);

        self.map[idx + 3]
    }
}

const U8_SCALE: FltPrim = 1.0 / 255.0;

impl Texture for Image {
    fn hit(&self, rng: &mut ThreadRng, u: Flt, v: Flt, _p: &Point3) -> bool {
        if self.transparency {
            let alpha = self.get_alpha(u, v);

            if alpha == 0xff {
                true
            } else if alpha == 0 {
                false
            } else {
                if rng.gen_range(0..255) > alpha {
                    false
                } else {
                    true
                }
            }
        } else {
            true
        }
    }

    fn value(&self, u: Flt, v: Flt, _p: &Point3) -> Colour {
        let (r, g, b) = self.get_rgb(u, v);

        Colour::new(
            r as FltPrim * U8_SCALE,
            g as FltPrim * U8_SCALE,
            b as FltPrim * U8_SCALE,
        )
    }
}
