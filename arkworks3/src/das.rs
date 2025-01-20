use crate::kzg_proofs::LFFTSettings;
use crate::kzg_types::ArkFr as BlstFr;
use kzg::{DASExtension, Fr};
use std::cmp::Ordering;

impl FFTSettings {
    fn das_fft_extension_stride(&self, ab: &mut [BlstFr], stride: usize) {
        match ab.len().cmp(&2_usize) {
            Ordering::Less => {}
            Ordering::Greater => {
                let half = ab.len();
                let halfhalf = half / 2;

                for i in 0..halfhalf {
                    let tmp1 = ab[i].add(&ab[halfhalf + i]);
                    let tmp2 = ab[i].sub(&ab[halfhalf + i]);
                    ab[halfhalf + i] = tmp2.mul(&self.reverse_roots_of_unity[i * 2 * stride]);
                    ab[i] = tmp1;
                }

                #[cfg(feature = "parallel")]
                {
                    if ab.len() > 32 {
                        let (lo, hi) = ab.split_at_mut(halfhalf);
                        rayon::join(
                            || self.das_fft_extension_stride(hi, stride * 2),
                            || self.das_fft_extension_stride(lo, stride * 2),
                        );
                    } else {
                        self.das_fft_extension_stride(&mut ab[..halfhalf], stride * 2);
                        self.das_fft_extension_stride(&mut ab[halfhalf..], stride * 2);
                    }
                }
                #[cfg(not(feature = "parallel"))]
                {
                    self.das_fft_extension_stride(&mut ab[..halfhalf], stride * 2);
                    self.das_fft_extension_stride(&mut ab[halfhalf..], stride * 2);
                }
                for i in 0..halfhalf {
                    let x = ab[i];
                    let y = ab[halfhalf + i];
                    let y_times_root = y.mul(&self.roots_of_unity[(1 + 2 * i) * stride]);
                    ab[i] = x.add(&y_times_root);
                    ab[halfhalf + i] = x.sub(&y_times_root);
                }
            }
            Ordering::Equal => {
                let x = ab[0].add(&ab[1]);
                let y = ab[0].sub(&ab[1]);
                let tmp = y.mul(&self.roots_of_unity[stride]);

                evens[0] = x.add(&y_times_root);
                evens[1] = x.sub(&y_times_root);

                return;
            }
            Ordering::Greater => {}
        }

        let half: usize = evens.len() / 2;
        for i in 0..half {
            let tmp1 = evens[i].add(&evens[half + i]);
            let tmp2 = evens[i].sub(&evens[half + i]);
            evens[half + i] = tmp2.mul(&self.reverse_roots_of_unity[i * 2 * stride]);

            evens[i] = tmp1;
        }

        #[cfg(feature = "parallel")]
        {
            if evens.len() > 32 {
                let (lo, hi) = evens.split_at_mut(half);
                rayon::join(
                    || self.das_fft_extension_stride(hi, stride * 2),
                    || self.das_fft_extension_stride(lo, stride * 2),
                );
            } else {
                // Recurse
                self.das_fft_extension_stride(&mut evens[..half], stride * 2);
                self.das_fft_extension_stride(&mut evens[half..], stride * 2);
            }
        }

        #[cfg(not(feature = "parallel"))]
        {
            // Recurse
            self.das_fft_extension_stride(&mut evens[..half], stride * 2);
            self.das_fft_extension_stride(&mut evens[half..], stride * 2);
        }

        for i in 0..half {
            let x = evens[i];
            let y = evens[half + i];
            let y_times_root: BlstFr = y.mul(&self.roots_of_unity[(1 + 2 * i) * stride]);

            evens[i] = x.add(&y_times_root);
            evens[half + i] = x.sub(&y_times_root);
        }
    }
}

impl DASExtension<BlstFr> for FFTSettings {
    fn das_fft_extension(&self, vals: &[BlstFr]) -> Result<Vec<BlstFr>, String> {
        if vals.is_empty() {
            return Err(String::from("vals can not be empty"));
        }
        if !vals.len().is_power_of_two() {
            return Err(String::from("vals lenght has to be power of 2"));
        }
        if vals.len() * 2 > self.max_width {
            return Err(String::from(
                "Supplied list is longer than the available max width",
            ));
        }

        // In case more roots are provided with fft_settings, use a larger stride
        let stride = self.max_width / (evens.len() * 2);
        let mut odds = evens.to_vec();
        self.das_fft_extension_stride(&mut odds, stride);

        // TODO: explain why each odd member is multiplied by euclidean inverse of length
        let mut inv_len = BlstFr::from_u64(odds.len() as u64);
        inv_len = inv_len.eucl_inverse();
        let odds = odds.iter().map(|f| f.mul(&inv_len)).collect();

        Ok(odds)
    }
}
