mod canvas;
mod math;
mod console;
mod plotter;
mod browser;
mod animation;

fn get_example_data(kind: u32) -> (Vec<f32>, Vec<f32>) {
    const TOTAL_NUM_POINTS: usize = 500;
    let mut t = vec![0.0; TOTAL_NUM_POINTS];
    let mut x = vec![0.0; TOTAL_NUM_POINTS];
    match kind {
        0 /* STEP */ => {
            for i in 0..TOTAL_NUM_POINTS {
                t[i] = i as f32 / 100.0;
                if i > 150 && i < 350 { x[i] = 1.0; }
            }
        },
        1 /* SINE */ => {
            for i in 0..TOTAL_NUM_POINTS {
                t[i] = i as f32 / 100.0;
                x[i] = (2.0 * std::f32::consts::PI * t[i]).sin();
            }
        },
        2 /* SQUARE */ => {
            for i in 0..t.len() {
                t[i] = i as f32 / 100.0;
                x[i] = if (2.0 * std::f32::consts::PI * t[i]).sin() >= 0.0 { 1.0 } else { -1.0 };
            }
        },
        _ /* TRIANGLE */ => {
            for i in 0..t.len() {
                t[i] = i as f32 / 100.0;
                x[i] = 2.0 * (2.0 * (t[i] - (t[i] + 0.25).floor() + 0.25)).abs() - 1.0;
            }
        },
    }
    (t,x)
}


struct ExampleCache {
    kind: u32,
    t: Vec<f32>,
    fourier: math::Fourier,
}
static mut EXAMPLE_CACHE: Option<ExampleCache> = None;

fn clamp_frequency_range(k_min: usize, k_max: usize, max_k: usize) -> (usize, usize) {
    let k_min = if k_min > max_k { max_k } else { k_min };
    let k_max = if k_max > max_k { max_k } else { k_max };
    (k_min, k_max)
}

fn plot_cached_example(canvas_id: u32, k_min: usize, k_max: usize, cache: &ExampleCache) {
    let mut plt = plotter::Plotter::new(canvas_id);
    if let Err(msg) = plt.plot_line(&cache.t, cache.fourier.original(), canvas::TAB_BLUE, 2.0) {
        console::error(&format!("Error plotting function: {}", msg));
        return;
    }
    let max_k = cache.fourier.max_frequency();
    let (k_min, k_max) = clamp_frequency_range(k_min, k_max, max_k);
    let filtered = match cache.fourier.filtered_range(k_min, k_max) {
        Ok(vec) => vec,
        Err(msg) => { console::error(&format!("Error filtering: {}", msg)); return; }
    };
    if let Err(msg) = plt.plot_line(&cache.t, &filtered, canvas::TAB_ORANGE, 2.0) {
        console::error(&format!("Error plotting filtered: {}", msg));
        return;
    }
    plt.show();
}

#[no_mangle]
pub unsafe fn plot_example(canvas_id: u32, k_min: usize, k_max: usize, kind: u32) {
    if let Some(ref cache) = EXAMPLE_CACHE {
        if cache.kind == kind {
            plot_cached_example(canvas_id, k_min, k_max, &cache);
            return;
        }
    }

    // If no cache or different kind, generate new cache
    let (t, x) = get_example_data(kind);
    let fourier = match math::Fourier::new(x) {
        Ok(fourier) => fourier,
        Err(msg) => {
            console::error(&format!("Failed to create Fourier instance: {}", msg));
            return;
        }
    };

    let cache = ExampleCache { kind, t, fourier };
    plot_cached_example(canvas_id, k_min, k_max, &cache);
    EXAMPLE_CACHE = Some(cache);
}


///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////

// Static animation instance for WASM interface
/// Start an animation loop for a 2D Fourier series visualization
static mut ANIMATION: Option<animation::Fourier> = None;

fn gen_cyclic_function() -> (Vec<f32>, Vec<f32>) {
    const BIG_R: f32 = 5.0;
    const SMALL_R: f32 = 1.0;
    const D: f32 = 2.0;

    let mut x = vec![0.0; 400];
    let mut y = vec![0.0; 400];
    for i in 0..400 {
        let angle =  (i as f32) * 2.0 * std::f32::consts::PI / 399.0;
        x[i] = (BIG_R + SMALL_R) * angle.cos() + D * ((BIG_R + SMALL_R) / SMALL_R * angle).cos();
        y[i] = (BIG_R + SMALL_R) * angle.sin() + D * ((BIG_R + SMALL_R) / SMALL_R * angle).sin();
    }
    (x, y)
}


fn init_animation_on_canvas(k_min: usize, k_max: usize, canvas_id: u32) {
    let (x, y) = crate::gen_cyclic_function();
    match animation::Fourier::new(x, y, k_min, k_max, canvas_id) {
        Ok(mut var) => {
            var.start();
            unsafe { ANIMATION = Some(var); }
        },
        Err(msg) => {
            console::error(&format!("Failed to create Fourier animation: {}", msg));
            unsafe { ANIMATION = None; }
        }
    }
}


#[no_mangle]
pub fn step_animation() {
    unsafe { if let Some(ref mut var) = ANIMATION { var.step(); } }
}

#[no_mangle]
pub fn play_pause_animation(canvas_id: u32, k_min: usize, k_max: usize) {
    unsafe {
        if let Some(ref mut var) = ANIMATION {
            if var.is_stopped() {
                init_animation_on_canvas(k_min, k_max, canvas_id);
            } else if var.is_paused() {
                var.play();
            } else if var.speed() > 1.0 || var.speed() < 1.0 {
                var.set_speed(1.0);
            } else {
                var.pause();
            }
        } else {
            init_animation_on_canvas(k_min, k_max, canvas_id);
        }
    }
}

#[no_mangle]
pub fn stop_animation() {
    unsafe { if let Some(ref mut var) = ANIMATION { var.stop(); } }
}

#[no_mangle]
pub fn increase_animation_speed() {
    unsafe { if let Some(ref mut var) = ANIMATION { var.set_speed(var.speed() + 0.5); } }
}

#[no_mangle]
pub fn decrease_animation_speed() {
    unsafe { if let Some(ref mut var) = ANIMATION { var.set_speed(var.speed() - 0.5); } }
}