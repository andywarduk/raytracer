use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::{
    float::*,
    triple::{Point3, Vec3},
};

const POINT_COUNT: usize = 256;

#[derive(Debug)]
pub struct PerlinNoise {
    randvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl PerlinNoise {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn noise(&self, p: &Point3) -> Flt {
        let u = p.x() - (p.x().floor());
        let v = p.y() - (p.y().floor());
        let w = p.z() - (p.z().floor());

        let u = u * u * (flt(3.0) - flt(2.0) * u);
        let v = v * v * (flt(3.0) - flt(2.0) * v);
        let w = w * w * (flt(3.0) - flt(2.0) * w);

        let i = FltPrim::from(p.x().floor()) as i64;
        let j = FltPrim::from(p.y().floor()) as i64;
        let k = FltPrim::from(p.z().floor()) as i64;

        let mut c = vec![vec![vec![Vec3::default(); 2]; 2]; 2];

        for (di, ci) in c.iter_mut().enumerate() {
            for (dj, cj) in ci.iter_mut().enumerate() {
                for (dk, ck) in cj.iter_mut().enumerate() {
                    *ck = self.randvec[self.perm_x[((i + di as i64) & 255) as usize]
                        ^ self.perm_y[((j + dj as i64) & 255) as usize]
                        ^ self.perm_z[((k + dk as i64) & 255) as usize]]
                        .clone();
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }

    pub fn turbulence(&self, p: &Point3, depth: usize) -> Flt {
        let (accum, _, _) =
            (0..depth).fold((flt(0.0), p.clone(), flt(1.0)), |(accum, p, weight), _| {
                (
                    accum + (weight * self.noise(&p)),
                    p * flt(2.0),
                    weight * flt(0.5),
                )
            });

        accum.abs()
    }

    fn generate_perm(rng: &mut ThreadRng) -> Vec<usize> {
        let mut vec: Vec<usize> = (0..POINT_COUNT).collect();

        Self::permute(rng, &mut vec);

        vec
    }

    fn permute(rng: &mut ThreadRng, vec: &mut [usize]) {
        (0..POINT_COUNT).rev().for_each(|i| {
            let target = rng.gen_range(0..POINT_COUNT);
            vec.swap(i, target);
        })
    }

    fn perlin_interp(c: Vec<Vec<Vec<Vec3>>>, u: Flt, v: Flt, w: Flt) -> Flt {
        // Hermitian smoothing
        let uu = u * u * (flt(3.0) - flt(2.0) * u);
        let vv = v * v * (flt(3.0) - flt(2.0) * v);
        let ww = w * w * (flt(3.0) - flt(2.0) * w);

        let mut accum = flt(0.0);

        for (i, ci) in c.iter().enumerate() {
            let fi = flt(i as FltPrim);

            for (j, cj) in ci.iter().enumerate() {
                let fj = flt(j as FltPrim);

                for (k, ck) in cj.iter().enumerate() {
                    let fk = flt(k as FltPrim);

                    let weight_v = Vec3::new_flt(u - fi, v - fj, w - fk);

                    accum += (fi * uu + (flt(1.0) - fi) * (flt(1.0) - uu))
                        * (fj * vv + (flt(1.0) - fj) * (flt(1.0) - vv))
                        * (fk * ww + (flt(1.0) - fk) * (flt(1.0) - ww))
                        * ck.dot(&weight_v);
                }
            }
        }

        accum
    }
}

impl Default for PerlinNoise {
    fn default() -> Self {
        let mut rng = thread_rng();

        let randvec = (0..POINT_COUNT)
            .map(|_| Vec3::new_random_clamped(&mut rng, -1.0, 1.0).unit_vector())
            .collect();

        let perm_x = Self::generate_perm(&mut rng);
        let perm_y = Self::generate_perm(&mut rng);
        let perm_z = Self::generate_perm(&mut rng);

        Self {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }
}
