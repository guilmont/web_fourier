mod canvas;
mod math;
mod console;
mod plotter;
mod browser;
mod animation;

#[no_mangle]
pub fn plot_step(cutoff: usize) {
    const STEP_START: usize = 150;
    const STEP_END: usize = 350;

    let mut t = vec![0.0; 500];
    let mut x = vec![0.0; 500];
    for i in 0..t.len() {
        t[i] = i as f32 / 100.0;
        if i > STEP_START && i < STEP_END { x[i] = 1.0; }
    }

    let fourier = match math::Fourier::new(x, cutoff) {
        Ok(val) => val,
        Err(msg) => { browser::alert(&format!("Error in low-pass filter: {}", msg)); return; }
    };
    let mut plt = plotter::Plotter::new();

    if let Err(msg) = plt.plot_line(&t, fourier.original(), canvas::TAB_BLUE, 2.0) {
        console::error(&format!("Error plotting step function: {}", msg));
        return;
    }
    if let Err(msg) = plt.plot_line(&t, fourier.filtered(), canvas::TAB_ORANGE, 2.0) {
        console::error(&format!("Error plotting low-pass filter: {}", msg));
        return;
    }
    plt.show();
}

///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////

// Static animation instance for WASM interface
/// Start an animation loop for a 2D Fourier series visualization
pub static mut ANIMATION: Option<animation::Fourier> = None;

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


fn init_animation(cutoff: usize) {
    let (x, y) = crate::gen_cyclic_function();
    match animation::Fourier::new(x, y, cutoff) {
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
pub fn play_pause_animation(cutoff: usize) {
    unsafe {
        if let Some(ref mut var) = ANIMATION {
            if var.is_stopped() {
                init_animation(cutoff);
            } else if var.is_paused() {
                var.play();
            } else if var.speed() > 1.0 || var.speed() < 1.0 {
                var.set_speed(1.0);
            } else {
                var.pause();
            }
        } else {
            init_animation(cutoff);
        }
    }
}

#[no_mangle]
pub fn stop_animation() {
    unsafe { if let Some(ref mut var) = ANIMATION { var.stop(); } }
}

#[no_mangle]
pub fn increase_animation_speed() {
    unsafe { if let Some(ref mut var) = ANIMATION {

        var.set_speed(var.speed() + 0.5);
    } }
}

#[no_mangle]
pub fn decrease_animation_speed() {
    unsafe { if let Some(ref mut var) = ANIMATION { var.set_speed(var.speed() - 0.5); } }
}