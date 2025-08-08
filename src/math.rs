#![allow(dead_code)]

/// Discrete Fourier Transform (DFT) and related utilities for complex and real data.
///
/// Note: For a DFT of length N, the frequency index N-k is equivalent to -k,
/// so negative frequencies are represented by the upper half of the spectrum.
/// For real input, DFT coefficients at k and N-k are complex conjugates (Hermitian symmetry).
pub use num_complex::Complex32;

pub struct Fourier {
    original: Vec<Complex32>,
    // DFT coefficients for all frequencies (0..N-1).
    transform: Vec<Complex32>,
}

impl Fourier {
    /// Constructs a Fourier object from a vector of real values.
    /// Returns an error if the input is empty or contains invalid values.
    pub fn from_real(data: Vec<f32>) -> Result<Self, String> {
        Self::from_complex(data.iter().map(|&x| Complex32::new(x, 0.0)).collect())
    }

    /// Constructs a Fourier object from a vector of complex values.
    /// Returns an error if the input is empty or contains invalid values.
    pub fn from_complex(data: Vec<Complex32>) -> Result<Self, String> {
        // Validate input
        if data.is_empty() { return Err("Input vector is empty".to_string()); }
        // Check for NaN or Inf values
        for val in &data {
            if !val.re.is_finite() || !val.im.is_finite() {
                return Err("Input vector contains invalid values (NaN or Inf)".to_string());
            }
        }

        Ok(Fourier {
            transform: dft(&data),
            original: data,
        })
    }

    /// Reconstructs the signal using only the frequency range [k_min, k_max].
    ///
    /// k_min and k_max are absolute frequency indices (0-based).
    /// Due to the DFT frequency symmetry (N-k ~ -k), this includes both positive and negative frequencies
    /// by summing the corresponding coefficients. For real input, coefficients at k and N-k are complex conjugates.
    /// Returns an error if the range is invalid.
    pub fn filtered_range(&self, k_min: usize, k_max: usize) -> Result<Vec<Complex32>, String> {
        let max_k = self.max_frequency();
        if k_min > k_max || k_max > max_k {
            return Err(format!("Frequency range [{}, {}] out of bounds (max {})", k_min, k_max, max_k));
        }
        Ok(idft(&self.transform, k_min, k_max))
    }

    /// Returns the power spectrum (magnitude squared) and phase (angle) of the DFT coefficients as two vectors.
    /// If `shifted` is true, the output is fftshifted (zero frequency centered).
    pub fn power_spectrum(&self, shifted: bool) -> (Vec<f32>, Vec<f32>) {
        let n = self.transform.len();
        let mut freq: Vec<f32> = (0..n).map(|k| k as f32).collect();
        let mut powers: Vec<f32> = self.transform.iter().map(|c| c.norm_sqr()).collect();
        if shifted {
            let max_k = self.max_frequency()+1;
            freq.rotate_left(max_k);
            powers.rotate_left(max_k);
            for i in 0..max_k {
                // Adjust frequencies to be centered around zero
                freq[i] -= n as f32;
            }
        }
        (freq, powers)
    }

    /// Returns the value of a single frequency component at a given time step.
    pub fn get_component(&self, frequency: usize, time_step: usize) -> Complex32 {
        let total_points = self.transform.len();

        let angle = 2.0 * std::f32::consts::PI * (time_step as f32) * (frequency as f32) / (total_points as f32);
        let exp_term = Complex32::new(0.0, angle).exp();

        self.transform[frequency] * exp_term / (total_points as f32).sqrt()
    }

    /// Returns the number of points in the original signal.
    pub fn size(&self) -> usize { self.original.len() }
    /// Returns a reference to the original signal data.
    pub fn original(&self) -> &[Complex32] { &self.original }

    /// Returns the maximum frequency index (N/2 - 1 for N points).
    pub fn max_frequency(&self) -> usize { self.transform.len() / 2  - 1 }
}

/// Computes the Discrete Fourier Transform (DFT) of the input data.
fn dft(data: &[Complex32]) -> Vec<Complex32> {
    let total_points = data.len();
    let norm = 1.0 / (total_points as f32).sqrt();
    let omega = Complex32::new(0.0, -2.0 * std::f32::consts::PI / total_points as f32);

    let mut result = Vec::<Complex32>::with_capacity(total_points);
    for k in 0..total_points {
        let mut res = Complex32::new(0.0, 0.0);
        for (i, &val) in data.iter().enumerate() {
            let angle = omega * (k as f32) * (i as f32);
            res += val * angle.exp();
        }
        result.push(res * norm); // Unitary scaling
    }
    result
}

/// Computes the Inverse Discrete Fourier Transform (IDFT) for a frequency range [k_min, k_max].
///
/// k_min and k_max are absolute frequency indices, so both positive and negative frequencies
/// are included by summing the coefficients at k and N-k.
fn idft(transform: &[Complex32], k_min: usize, k_max: usize) -> Vec<Complex32> {
    let total_points = transform.len();
    let norm = 1.0 / (total_points as f32).sqrt();
    let omega = Complex32::new(0.0, 2.0 * std::f32::consts::PI / total_points as f32);

    let mut result = Vec::<Complex32>::with_capacity(total_points);
    for i in 0..total_points {
        let mut res = Complex32::new(0.0, 0.0);
        let partial = omega * (i as f32);
        for k in k_min..=k_max {
            // Positive frequency component
            res += transform[k] * (partial * (k as f32)).exp();
            // Skip k=0 to avoid double counting
            if k == 0 { continue; }
            // The transform is symetric on 180 degrees, so we must include  negative frequency
            res += transform[total_points - k] * (partial * (total_points - k) as f32).exp();
        }
        result.push(res * norm); // Unitary scaling
    }
    result
}
