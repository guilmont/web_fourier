#![allow(dead_code)]

pub use num_complex::Complex32;

pub struct Fourier {
    original: Vec<Complex32>,
    transform: Vec<Complex32>, // DFT coefficients for all frequencies (0..N-1)
}

impl Fourier {
    /// Creates a new `Fourier` instance with real-valued data.
    /// The data is automatically centered (mean removed) before computing the DFT.
    ///
    /// # Arguments
    /// * `data` - A vector of real input data.
    ///
    /// # Returns
    /// A `Result` containing the `Fourier` instance or an error message.
    pub fn from_real(data: Vec<f32>) -> Result<Self, String> {
        Self::from_complex(data.iter().map(|&x| Complex32::new(x, 0.0)).collect())
    }

    /// Internal constructor from complex data
    pub fn from_complex(data: Vec<Complex32>) -> Result<Self, String> {
        // Validate input
        if data.is_empty() { return Err("Input vector is empty".to_string()); }

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

    /// Reconstruct a filtered signal using only frequencies in the given range [k_min, k_max] (absolute values).
    pub fn filtered_range(&self, k_min: usize, k_max: usize) -> Result<Vec<Complex32>, String> {
        let max_k = self.transform.len() / 2; // Only consider positive frequencies
        if k_min > k_max || k_max > max_k {
            return Err(format!("Frequency range [{}, {}] out of bounds (max {})", k_min, k_max, max_k));
        }
        Ok(idft(&self.transform, k_min, k_max))
    }

    /// Return the power spectrum (magnitude squared) for all N frequencies.
    pub fn power_spectrum(&self) -> Vec<f32> {
        self.transform.iter().map(|c| c.norm_sqr()).collect()
    }

    /// Get the signal component for a specific frequency and time step.
    /// Returns the complex component (real + imaginary).
    ///
    /// # Arguments
    /// * `frequency` - The frequency index.
    /// * `time_step` - The time step index.
    ///
    /// # Returns
    /// A `Result` containing the complex signal strength or an error message.
    pub fn get_component(&self, frequency: usize, time_step: usize) -> Complex32 {
        let total_points = self.transform.len();

        let angle = 2.0 * std::f32::consts::PI * (time_step as f32) * (frequency as f32) / (total_points as f32);
        let exp_term = Complex32::new(0.0, angle).exp();

        self.transform[frequency] * exp_term
    }

    pub fn size(&self) -> usize { self.original.len() }
    pub fn original(&self) -> &[Complex32] { &self.original }
    pub fn max_frequency(&self) -> usize { self.transform.len() / 2 }
}

/// Fallback DFT for non-power-of-2 lengths
fn dft(data: &[Complex32]) -> Vec<Complex32> {
    let total_points = data.len();
    let param = 1.0 / total_points as f32;
    // Use a complex exponential for the DFT
    // omega = 2 * pi / N, where N is the total number of points

    let omega = Complex32::new(0.0, -2.0 * std::f32::consts::PI * param);

    let mut result = Vec::<Complex32>::with_capacity(total_points);
    for k in 0..total_points {
        let mut res = Complex32::new(0.0, 0.0);
        for (i, &val) in data.iter().enumerate() {
            let angle = omega * (k as f32) * (i as f32);
            res += val * angle.exp();
        }
        result.push(res * param); // No scaling in forward transform
    }
    result
}


/// Inverse DFT for signal reconstruction
fn idft(transform: &[Complex32], k_min: usize, k_max: usize) -> Vec<Complex32> {
    let total_points = transform.len();
    let omega = Complex32::new(0.0, 2.0 * std::f32::consts::PI / total_points as f32);

    let mut result = Vec::<Complex32>::with_capacity(total_points);
    for i in 0..total_points {
        let mut res = Complex32::new(0.0, 0.0);
        for k in k_min..=k_max {
            let angle = omega * (k as f32) * (i as f32);
            let exp_term = angle.exp();
            res += transform[k] * exp_term;

            // Include the corresponding negative frequency
            // For negative frequency: exp(-jθ) = 1 / exp(jθ)
            if k > 0 {
                res += transform[total_points - k] * exp_term.inv();
            }
        }
        result.push(res);
    }
    result
}
