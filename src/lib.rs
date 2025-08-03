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
    // // Animation instance for the Fourier series visualization
    // static ANIMATION: RefCell<animation::Fourier> = RefCell::new(animation::Fourier::default());
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
    plt.set_x_range(-5.0, 50.0);
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

// fn gen_cyclic_function() -> (Vec<f32>, Vec<f32>) {
//     const BIG_R: f32 = 5.0;
//     const SMALL_R: f32 = 1.0;
//     const D: f32 = 2.0;

//     let mut x = vec![0.0; 400];
//     let mut y = vec![0.0; 400];
//     for i in 0..400 {
//         let angle =  (i as f32) * 2.0 * std::f32::consts::PI / 399.0;
//         x[i] = (BIG_R + SMALL_R) * angle.cos() + D * ((BIG_R + SMALL_R) / SMALL_R * angle).cos();
//         y[i] = (BIG_R + SMALL_R) * angle.sin() + D * ((BIG_R + SMALL_R) / SMALL_R * angle).sin();
//     }
//     (x, y)
// }


// fn init_animation_on_canvas(k_min: usize, k_max: usize) {
//     let (x, y) = crate::gen_cyclic_function();
//     match animation::Fourier::new(x, y, k_min, k_max) {
//         Ok(mut var) => {
//             var.start();
//             ANIMATION.with(|cell| { *cell.borrow_mut() = Some(var); });
//         },
//         Err(msg) => {
//             console::error(&format!("Failed to create Fourier animation: {}", msg));
//             ANIMATION.with(|cell| { *cell.borrow_mut() = None; });
//         }
//     }
// }


// #[no_mangle]
// pub fn step_animation() {
//     ANIMATION.with(|cell| {
//         if let Some(ref mut var) = *cell.borrow_mut() { var.step(); }
//     });
// }

// #[no_mangle]
// pub fn play_pause_animation(k_min: usize, k_max: usize) {
//     ANIMATION.with(|cell| {
//         let mut borrow = cell.borrow_mut();
//         if let Some(ref mut var) = *borrow {
//             if var.is_stopped() {
//                 drop(borrow);
//                 init_animation_on_canvas(k_min, k_max);
//             } else if var.is_paused() {
//                 var.play();
//             } else if var.speed() > 1.0 || var.speed() < 1.0 {
//                 var.set_speed(1.0);
//             } else {
//                 var.pause();
//             }
//         } else {
//             drop(borrow);
//             init_animation_on_canvas(k_min, k_max);
//         }
//     });
// }

// #[no_mangle]
// pub fn stop_animation() {
//     ANIMATION.with(|cell| {
//         if let Some(ref mut var) = *cell.borrow_mut() {
//             var.stop();
//         }
//     });
// }

// #[no_mangle]
// pub fn increase_animation_speed() {
//     ANIMATION.with(|cell| {
//         if let Some(ref mut var) = *cell.borrow_mut() {
//             var.set_speed(var.speed() + 0.5);
//         }
//     });
// }

// #[no_mangle]
// pub fn decrease_animation_speed() {
//     ANIMATION.with(|cell| {
//         if let Some(ref mut var) = *cell.borrow_mut() {
//             var.set_speed(var.speed() - 0.5);
//         }
//     });
// }