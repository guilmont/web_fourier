use std::cell::RefCell;

use web_canvas::canvas;
use web_canvas::console;

mod math;
mod plotter;
mod animation;

struct ExampleCache {
    kind: u32,
    t: Vec<f32>,
    fourier: math::Fourier,
}

thread_local! {
    // Cache for example data, shared across the application
    static EXAMPLE_CACHE: RefCell<Option<ExampleCache>> = RefCell::new(None);
}

///////////////////////////////////////////////////////////////////////////////
/// Example data
///////////////////////////////////////////////////////////////////////////////

fn generate_cache(kind: u32) -> ExampleCache {
    const TOTAL_NUM_POINTS: usize = 500;
    let mut t = Vec::with_capacity(TOTAL_NUM_POINTS);
    let mut x = Vec::with_capacity(TOTAL_NUM_POINTS);

    let generator: fn(f32, usize) -> f32 = match kind {
        /* Step function */ 0 => |_, i|  { if i > 150 && i < 350  { 1.0 } else { 0.0 }                             },
        /* Sine */          1 => |ti, _| { (2.0 * std::f32::consts::PI * ti).sin()                                 },
        /* Square */        2 => |ti, _| { if (2.0 * std::f32::consts::PI * ti).sin() >= 0.0 { 1.0 } else { -1.0 } },
        /* Triangle */      _ => |ti, _| { 2.0 * (2.0 * (ti - (ti + 0.25).floor() + 0.25)).abs() - 1.0             },
    };

    for i in 0..TOTAL_NUM_POINTS {
        let ti = i as f32 / 100.0;
        t.push(ti);
        x.push(generator(ti, i));
    }

    let fourier = match math::Fourier::new(x) {
        Ok(fourier) => fourier,
        Err(msg) => {
            console::error(&format!("Failed to create Fourier instance: {}", msg));
            panic!("Fourier creation failed");
        }
    };
    ExampleCache { kind, t, fourier }
}

fn plot_cached_example(k_min: usize, k_max: usize, cache: &mut ExampleCache) {
    // Start with the example plotter
    let plt = plotter::Plotter::get_or_create("example-canvas");
    let filtered = match cache.fourier.filtered_range(k_min, k_max) {
        Ok(vec) => vec,
        Err(msg) => { console::error(&format!("Error filtering: {}", msg)); return; }
    };

    if let Err(msg) = plt.plot_line(&cache.t, cache.fourier.original(), canvas::TAB_BLUE, 2.0) {
        console::error(&format!("Error plotting function: {}", msg));
        return;
    }
    if let Err(msg) = plt.plot_line(&cache.t, &filtered, canvas::TAB_ORANGE, 2.0) {
        console::error(&format!("Error plotting filtered: {}", msg));
        return;
    }
    plt.show();
}

fn plot_cached_spectrum(cache: &mut ExampleCache) {
    // Now plot the Fourier spectrum
    let power = cache.fourier.power_spectrum();
    let n = power.len();
    let freq: Vec<f32> = (0..n).map(|k| k as f32).collect();

    let plt = plotter::Plotter::get_or_create("spectrum-canvas");
    plt.set_x_range(-5.0, freq.len() as f32 / 3.0);
    if let Err(msg) = plt.plot_histogram(&freq, &power, canvas::TAB_GREEN, 1.0) {
        console::error(&format!("Error plotting power spectrum: {}", msg));
        return;
    }
    plt.show();
}

#[no_mangle]
pub fn plot_example(k_min: usize, k_max: usize, kind: u32) {
    let mut cache_kind_match = false;
    EXAMPLE_CACHE.with(|cell| {
        // If the cache already exists and matches the kind, use it
        if let Some(ref mut cache) = *cell.borrow_mut() {
            if cache.kind == kind {
                plot_cached_example(k_min, k_max, cache);
                cache_kind_match = true;
            }
        }
    });

    if cache_kind_match { return; }

    let mut cache = generate_cache(kind);
    plot_cached_example(k_min, k_max, &mut cache);
    plot_cached_spectrum(&mut cache);
    EXAMPLE_CACHE.with(|cell| { *cell.borrow_mut() = Some(cache); });
}

///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////

fn gen_animation_function(example: usize) -> (Vec<f32>, Vec<f32>) {
    const TOTAL_NUM_POINTS: usize = 400;
    let mut x = vec![0.0; TOTAL_NUM_POINTS];
    let mut y = vec![0.0; TOTAL_NUM_POINTS];

    match example {
        0 => {
            // Epitrochoid (original example)
            const BIG_R: f32 = 5.0;
            const SMALL_R: f32 = 1.0;
            const D: f32 = 2.0;

            for i in 0..TOTAL_NUM_POINTS {
                let angle = (i as f32) * 2.0 * std::f32::consts::PI / (TOTAL_NUM_POINTS - 1) as f32;
                x[i] = (BIG_R + SMALL_R) * angle.cos() + D * ((BIG_R + SMALL_R) / SMALL_R * angle).cos();
                y[i] = (BIG_R + SMALL_R) * angle.sin() + D * ((BIG_R + SMALL_R) / SMALL_R * angle).sin();
            }
        },
        1 => {
            // Rose curve with multiple harmonics
            for i in 0..TOTAL_NUM_POINTS {
                let t = (i as f32) * std::f32::consts::PI / (TOTAL_NUM_POINTS - 1) as f32;
                let r = 3.0 * (3.0 * t).cos() + 1.0 * (9.0 * t).cos() + 0.5 * (15.0 * t).cos();
                x[i] = r * t.cos();
                y[i] = r * t.sin();
            }
        },
        2 => {
            // Lissajous curve with frequency ratio 3:5 + harmonics
            for i in 0..TOTAL_NUM_POINTS {
                let t = (i as f32) * 2.0 * std::f32::consts::PI / (TOTAL_NUM_POINTS - 1) as f32;
                let phase_shift = std::f32::consts::PI / 4.0;

                // Main frequencies
                x[i] = 4.0 * (3.0 * t).sin() + 1.5 * (6.0 * t).sin() + 0.8 * (9.0 * t).sin();
                y[i] = 3.0 * (5.0 * t + phase_shift).sin() + 1.2 * (10.0 * t).sin() + 0.6 * (15.0 * t).sin();
            }
        },
        3 => {
            // Spirograph-like pattern with multiple frequencies
            for i in 0..TOTAL_NUM_POINTS {
                let t = (i as f32) * 2.0 * std::f32::consts::PI / (TOTAL_NUM_POINTS - 1) as f32;

                // Complex combination of circular motions
                x[i] = 3.0 * t.cos() + 2.0 * (2.0 * t).cos() + 1.0 * (4.0 * t).cos() + 0.5 * (7.0 * t).cos();
                y[i] = 3.0 * t.sin() + 2.0 * (2.0 * t).sin() + 1.0 * (4.0 * t).sin() + 0.5 * (7.0 * t).sin();
            }
        },
        _ => {
           console::error(format!("Unknown example code: {}", example).as_str());
        }
    }

    (x, y)
}


fn init_animation_on_canvas(k_min: usize, k_max: usize, example_code: usize) {
    let (x, y) = gen_animation_function(example_code);

    // Create Fourier transforms once
    match math::Fourier::new(x) {
        Ok(fourier_x) => {
            match math::Fourier::new(y) {
                Ok(fourier_y) => {
                    // Plot the frequency spectrum histogram once during initialization
                    let power_x = fourier_x.power_spectrum();
                    let power_y = fourier_y.power_spectrum();

                    // Combine X and Y power spectra
                    let combined_power: Vec<f32> = power_x.iter()
                        .zip(power_y.iter())
                        .map(|(px, py)| px + py)
                        .collect();

                    let n = combined_power.len();
                    let freq: Vec<f32> = (0..n).map(|k| k as f32).collect();

                    // Plot the spectrum histogram
                    let spectrum_plt = plotter::Plotter::get_or_create("animation-spectrum-canvas");
                    spectrum_plt.set_x_range(-5.0, freq.len() as f32 / 3.0);
                    if let Err(msg) = spectrum_plt.plot_histogram(&freq, &combined_power, canvas::TAB_GREEN, 1.0) {
                        console::error(&format!("Error plotting animation spectrum: {}", msg));
                    } else {
                        spectrum_plt.show();
                    }

                    // Create and start the animation using the same Fourier transforms
                    match animation::Fourier::from_fourier(fourier_x, fourier_y, k_min, k_max) {
                        Ok(mut var) => {
                            var.start();
                            animation::set_animation(var);
                        },
                        Err(msg) => {
                            console::error(&format!("Failed to create Fourier animation: {}", msg));
                            animation::clear_animation();
                        }
                    }
                },
                Err(msg) => {
                    console::error(&format!("Failed to create Y Fourier for spectrum: {}", msg));
                    animation::clear_animation();
                }
            }
        },
        Err(msg) => {
            console::error(&format!("Failed to create X Fourier for spectrum: {}", msg));
            animation::clear_animation();
        }
    }
}


#[no_mangle]
pub fn play_pause_animation(k_min: usize, k_max: usize, example_code: usize) {
    animation::play_pause_animation(k_min, k_max, example_code, init_animation_on_canvas);
}

#[no_mangle]
pub fn stop_animation() {
    animation::stop_animation();
}

#[no_mangle]
pub fn increase_animation_speed() {
    animation::increase_animation_speed();
}

#[no_mangle]
pub fn decrease_animation_speed() {
    animation::decrease_animation_speed();
}