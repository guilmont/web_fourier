#![allow(dead_code)]
use num_complex::Complex32;

pub struct Fourier {
    original: Vec<f32>,
    filtered: Vec<f32>,
    transform: Vec<Complex32>,
    cutoff: usize,
    coeff: f32,
}

impl Fourier {
    /// Creates a new `Fourier` instance with the given data and cutoff frequency.
    ///
    /// # Arguments
    /// * `original` - A vector of input data.
    /// * `cutoff` - The maximum frequency to include in the Fourier series.
    ///
    /// # Returns
    /// A `Result` containing the `Fourier` instance or an error message.
    pub fn new(original: Vec<f32>, cutoff: usize) -> Result<Self, String> {
        let total_size = original.len();
        if total_size == 0 {
            return Err("Input vector is empty".to_string());
        }

        let max_cutoff = total_size - 50;
        if cutoff == 0 || cutoff > max_cutoff {
            return Err(format!("Cutoff frequency must be between 1 and {}", max_cutoff));
        }

        let coeff = 2.0 * std::f32::consts::PI / (total_size as f32);

        // Compute DFT coefficients for frequencies 1 to cutoff
        let mut transform = vec![Complex32::new(0.0, 0.0); cutoff + 1];
        for k in 1..=cutoff {
            let mut sum = Complex32::new(0.0, 0.0);
            for (i, &val) in original.iter().enumerate() {
                if !val.is_finite() {
                    return Err("Input vector contains invalid values (NaN or Inf)".to_string());
                }
                let angle = -coeff * (k as f32) * (i as f32);
                let exp_term = Complex32::new(0.0, angle).exp();
                sum += exp_term * val;
            }
            transform[k] = sum / (total_size as f32);
        }

        // Reconstruct signal using only low frequencies
        let mut filtered = vec![0.0; total_size];
        let mean = original.iter().sum::<f32>() / (total_size as f32);
        for i in 0..total_size {
            filtered[i] = mean;
            for k in 1..=cutoff {
                let angle = coeff * (i as f32) * (k as f32);
                let exp_term = Complex32::new(0.0, angle).exp();
                let product = transform[k] * exp_term;
                filtered[i] += 2.0 * product.re;
            }
        }
        Ok(Fourier { original, filtered, transform, cutoff, coeff })
    }

    /// Get low pass signal strength for specific frequency and time step.
    ///
    /// # Arguments
    /// * `frequency` - The frequency index.
    /// * `time_step` - The time step index.
    ///
    /// # Returns
    /// A `Result` containing the signal strength or an error message.
    pub fn get_component(&self, frequency: usize, time_step: usize) -> Result<f32, String> {
        if frequency > self.cutoff {
            return Err(format!("Frequency {} out of bounds (max {})", frequency, self.cutoff));
        }

        let angle = self.coeff * (time_step as f32) * (frequency as f32);
        let exp_term = Complex32::new(0.0, angle).exp();
        let component = self.transform[frequency] * exp_term;
        Ok(2.0 * component.re)
    }

    /// Getters
    pub fn original(&self) -> &[f32] { &self.original }
    pub fn filtered(&self) -> &[f32] { &self.filtered }
    pub fn cutoff(&self) -> usize { self.cutoff }
    pub fn size(&self) -> usize { self.original.len() }
}