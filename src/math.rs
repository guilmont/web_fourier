#![allow(dead_code)]
use num_complex::Complex32;

pub struct Fourier {
    original: Vec<f32>,
    transform: Vec<Complex32>, // DFT coefficients for all frequencies (0..=N/2)
    coeff: f32,
}

impl Fourier {
    /// Creates a new `Fourier` instance with the given data.
    /// The data is automatically centered (mean removed) before computing the DFT.
    ///
    /// # Arguments
    /// * `original` - A vector of input data.
    ///
    /// # Returns
    /// A `Result` containing the `Fourier` instance or an error message.
    pub fn new(original: Vec<f32>) -> Result<Self, String> {
        let total_size = original.len();
        if total_size == 0 {
            return Err("Input vector is empty".to_string());
        }

        // Calculate mean and center the data
        let mean: f32 = original.iter().sum::<f32>() / (total_size as f32);
        let centered_data: Vec<f32> = original.iter().map(|&x| x - mean).collect();

        let coeff = 2.0 * std::f32::consts::PI / (total_size as f32);
        // Compute DFT coefficients for frequencies 0 to N/2
        let max_k = total_size / 2;
        let mut transform = vec![Complex32::new(0.0, 0.0); max_k + 1];
        for k in 0..=max_k {
            let mut sum = Complex32::new(0.0, 0.0);
            for (i, &val) in centered_data.iter().enumerate() {
                if !val.is_finite() {
                    return Err("Input vector contains invalid values (NaN or Inf)".to_string());
                }
                let angle = -coeff * (k as f32) * (i as f32);
                let exp_term = Complex32::new(0.0, angle).exp();
                sum += exp_term * val;
            }
            transform[k] = sum / (total_size as f32);
        }
        Ok(Fourier { original: centered_data, transform, coeff })
    }

    /// Reconstruct a filtered signal using only frequencies in the given range [k_min, k_max] (inclusive).
    pub fn filtered_range(&self, k_min: usize, k_max: usize) -> Result<Vec<f32>, String> {
        let n = self.original.len();
        let max_k = self.transform.len() - 1;
        if k_min > k_max || k_max > max_k {
            return Err(format!("Frequency range [{}, {}] out of bounds (max {})", k_min, k_max, max_k));
        }
        let mut filtered = vec![0.0; n];
        for i in 0..n {
            for k in k_min..=k_max {
                let angle = self.coeff * (i as f32) * (k as f32);
                let exp_term = Complex32::new(0.0, angle).exp();
                let product = self.transform[k] * exp_term;
                filtered[i] += if k == 0 { product.re } else { 2.0 * product.re };
            }
        }
        Ok(filtered)
    }

    /// Return the power spectrum (magnitude squared) for all N/2+1 frequencies.
    pub fn power_spectrum(&self) -> Vec<f32> {
        self.transform.iter().map(|c| c.norm_sqr()).collect()
    }

    /// Get the signal component for a specific frequency and time step.
    ///
    /// # Arguments
    /// * `frequency` - The frequency index.
    /// * `time_step` - The time step index.
    ///
    /// # Returns
    /// A `Result` containing the signal strength or an error message.
    pub fn get_component(&self, frequency: usize, time_step: usize) -> Result<f32, String> {
        let max_k = self.transform.len() - 1;
        if frequency > max_k {
            return Err(format!("Frequency {} out of bounds (max {})", frequency, max_k));
        }
        let angle = self.coeff * (time_step as f32) * (frequency as f32);
        let exp_term = Complex32::new(0.0, angle).exp();
        let component = self.transform[frequency] * exp_term;
        Ok(if frequency == 0 { component.re } else { 2.0 * component.re })
    }

    /// Getters
    pub fn original(&self) -> &[f32] { &self.original }
    pub fn size(&self) -> usize { self.original.len() }
    pub fn max_frequency(&self) -> usize { self.transform.len() - 1 }
}