#![allow(dead_code)]
use num_complex::Complex32;

mod js {
    #[link(wasm_import_module = "Math")]
    extern "C" {
        pub fn random() -> f32;
    }
}

pub fn random() -> f32 { unsafe { js::random() } }

pub fn low_pass(inp_vec: &[f32], cutoff: usize) -> Result<Vec<f32>, String> {
    let total_size = inp_vec.len();
    if total_size == 0 { return Err("Input vector is empty".into()); }

    let max_cutoff = total_size - 50; // Avoid cutoff too close to the end
    if cutoff == 0 || cutoff > max_cutoff {
        return Err(format!("Cutoff frequency must be between 1 and {}", max_cutoff));
    }
    // Clamp cutoff to avoid out-of-bounds access
    let cutoff = cutoff.min(total_size - 1);

    let coeff = 2.0 * std::f32::consts::PI / (total_size as f32);

    // Compute DFT coefficients for frequencies 1 to cutoff
    let mut trans = vec![Complex32::new(0.0, 0.0); cutoff + 1];
    for k in 1..=cutoff {
        let mut sum = Complex32::new(0.0, 0.0);
        for (i, &val) in inp_vec.iter().enumerate() {
            let angle = -coeff * (k as f32) * (i as f32);
            let exp_term = Complex32::new(0.0, angle).exp();
            sum += exp_term * val;
        }
        trans[k] = sum / (total_size as f32);
    }

    // Compute mean, that is, the term for frequency zero
    let mean = inp_vec.iter().sum::<f32>() / (total_size as f32);

    // Reconstruct signal using only low frequencies
    let mut result = vec![0.0; total_size];
    for i in 0..total_size {
        result[i] = mean;
        for k in 1..=cutoff {
            let angle = coeff * (i as f32) * (k as f32);
            let exp_term = Complex32::new(0.0, angle).exp();
            // 2 * Re(trans[k] * exp(i*angle))
            let product = trans[k] * exp_term;
            result[i] += 2.0 * product.re;
        }
    }

    Ok(result)
}
