use crate::global_type_helper::{ColorClass, RgbPixel};

/// Ethereum blockies generator, which stores necessary seeds for creating blockies
pub struct KeyGenerator {
    /// Seeds for generating ethereum blockies  
    /// (Named as "randseed" in original implementation)
    key_seeds: [i32; KeyGenerator::KEY_SEEDS_LEN],
    /// Current index of key_seeds to update when next_key runs
    key_seed_curidx: usize,
}

impl KeyGenerator {
    const KEY_SEEDS_LEN: usize = 4;

    /// Initialize new ethereum blockies generator using a given seed byte sequences
    pub fn new(seed: &[u8]) -> Self {
        Self {
            key_seeds: (seed.chunks(KeyGenerator::KEY_SEEDS_LEN).fold(
                [0_i32; KeyGenerator::KEY_SEEDS_LEN],
                |mut key_seeds_acc, seed_chunks| {
                    key_seeds_acc.iter_mut().zip(seed_chunks.iter()).for_each(
                        |(key_seed_cur, seed_char_cur)| {
                            *key_seed_cur = Self::key_seed_init(*key_seed_cur, *seed_char_cur)
                        },
                    );

                    key_seeds_acc
                },
            )),
            key_seed_curidx: 0,
        }
    }

    /// Get next RGB pixel for palette using key_seeds
    pub fn next_rgb(&mut self) -> RgbPixel {
        fn hsl_to_rgb(hue: u32, saturation: f64, lightness: f64) -> RgbPixel {
            fn hue_to_rgb(p: f64, q: f64, t: u32) -> f64 {
                let t = match t {
                    0..=359 => t,
                    _ => t % 360,
                };

                return match t {
                    0..=60 => p + (q - p) * t as f64 / 60_f64,
                    0..=180 => q,
                    0..=240 => p + (q - p) * (4_f64 - t as f64 / 60_f64),
                    _ => p,
                };
            }

            let rgb_frac = match saturation == 0_f64 {
                true => (lightness, lightness, lightness),
                false => {
                    let q = match lightness < 0.5 {
                        true => lightness * (1_f64 + saturation),
                        false => lightness + saturation - lightness * saturation,
                    };
                    let p = 2_f64 * lightness - q;
                    (
                        hue_to_rgb(p, q, hue.overflowing_add(120).0),
                        hue_to_rgb(p, q, hue),
                        hue_to_rgb(p, q, hue.overflowing_add(240).0),
                    )
                }
            };

            (
                ((rgb_frac.0 * 255_f64) + 0.5_f64) as u8,
                ((rgb_frac.1 * 255_f64) + 0.5_f64) as u8,
                ((rgb_frac.2 * 255_f64) + 0.5_f64) as u8,
            )
        }

        hsl_to_rgb(
            (self.next_key() * 360_f64) as u32,
            self.next_key() * 0.6_f64 + 0.4_f64,
            (self.next_key() + self.next_key() + self.next_key() + self.next_key()) * 0.25_f64,
        )
    }

    /// Get next color class for current pixel using key_seeds
    pub fn next_colorclass(&mut self) -> ColorClass {
        ((self.next_key() * 2.3_f64) as u8).try_into().unwrap()
    }

    // Update single element in key_seeds for initialization
    fn key_seed_init(key_seed_cur: i32, seed_char_cur: u8) -> i32 {
        (key_seed_cur << 5)
            .overflowing_sub(key_seed_cur)
            .0
            .overflowing_add(seed_char_cur as i32)
            .0
    }

    // Get previous index of key_seeds
    fn idx_prev(idx: usize) -> usize {
        idx.overflowing_sub(1).0 % KeyGenerator::KEY_SEEDS_LEN
    }

    // Get next index of key_seeds
    fn idx_next(idx: usize) -> usize {
        idx.overflowing_add(1).0 % KeyGenerator::KEY_SEEDS_LEN
    }

    // Get next computed key using key_seeds, which is used for blockies generation
    // Returns f64 in range: [0, 1]
    fn next_key(&mut self) -> f64 {
        self.key_seeds
            .get(Self::idx_prev(self.key_seed_curidx))
            .zip(self.key_seeds.get(self.key_seed_curidx))
            // calc new cur val
            .map(|(key_seed_prev, key_seed_cur)| {
                let tmp = *key_seed_cur ^ (*key_seed_cur << 11);
                *key_seed_prev ^ (*key_seed_prev >> 19) ^ tmp ^ (tmp >> 8)
            })
            // update self members
            .and_then(|key_seed_new_cur| {
                self.key_seeds
                    .get_mut(self.key_seed_curidx)
                    .map(|key_seed_cur_mut| {
                        self.key_seed_curidx = Self::idx_next(self.key_seed_curidx);

                        *key_seed_cur_mut = key_seed_new_cur;
                        key_seed_new_cur
                    })
            })
            // map to return val: map key_seed_new_cur in [0, 1] range
            .map(|key_seed_new_cur| {
                key_seed_new_cur.unsigned_abs() as f64 / ((i32::MAX as u32 + 1) as f64)
            })
            .expect("next_key")
    }
}
