mod canvas;
mod math;
mod console;
mod plotter;
mod browser;

#[no_mangle]
pub fn say_hi() { console::log("Hello there"); }

#[no_mangle]
pub fn foo(ptr: *const u8, len: usize) {
    let utf8 = unsafe { core::slice::from_raw_parts(ptr, len) };
    let msg = str::from_utf8(utf8).unwrap();
    let v = format!("Question: {} :: Answer: {}", msg, "your face");
    console::log(v.as_str());
}

#[no_mangle]
pub fn draw_random_pattern() {
    let width = canvas::width();
    let height = canvas::height();
    canvas::clear();

    for _i in 0..50 {
        let x = math::random() * width;
        let y = math::random() * height;
        let radius = math::random() * 20.0 + 5.0;
        let r = (math::random() * 255.0) as u8;
        let g = (math::random() * 255.0) as u8;
        let b = (math::random() * 255.0) as u8;

        canvas::draw_circle(x, y, radius, (r, g, b));
    }
}

#[no_mangle]
pub fn plot_step(cutoff: usize) {
    const STEP_START: usize = 150;
    const STEP_END: usize = 350;

    let mut t = vec![0.0; 500];
    let mut x = vec![0.0; 500];
    for i in 0..t.len() {
        t[i] = i as f32 / 100.0;
        if i > STEP_START && i < STEP_END { x[i] = 1.0; } // Step function: 1 for middle part
    }

    let y = match math::low_pass(x.as_slice(), cutoff) {
        Ok(val) => val,
        Err(msg) => { browser::alert(&format!("Error in low-pass filter: {}", msg)); return; }
    };
    let mut plt = plotter::Plotter::new();

    if let Err(msg) = plt.plot_line(&t, &x, canvas::TAB_BLUE, 2.0) {
        console::error(&format!("Error plotting step function: {}", msg));
        return;
    }
    if let Err(msg) = plt.plot_line(&t, &y, canvas::TAB_ORANGE, 2.0) {
        console::error(&format!("Error plotting low-pass filter: {}", msg));
        return;
    }
    plt.show();
}

#[no_mangle]
pub fn plot_multiple_functions() {
    let mut plt = plotter::Plotter::new();
    plt.set_x_range(-0.5, 5.0);
    plt.set_y_range(-2.0, 2.0);
    plt.set_x_ticks(11);

    // Create some sample functions
    let mut x_data = vec![0.0; 101];
    let mut sin_data = vec![0.0; 101];
    let mut cos_data = vec![0.0; 101];
    let mut exp_data = vec![0.0; 101];

    for i in 0..101 {
        x_data[i] = i as f32 * 0.05;
        sin_data[i] = (x_data[i] * 2.0).sin();
        cos_data[i] = (x_data[i] * 2.0).cos();
        exp_data[i] = (-x_data[i]).exp();
    }

    if let Err(msg) = plt.plot_line(&x_data, &sin_data, canvas::TAB_BLUE, 2.0) {
        console::error(&format!("Error plotting sine function: {}", msg));
        return;
    }
    if let Err(msg) = plt.plot_line(&x_data, &cos_data, canvas::TAB_ORANGE, 2.0) {
        console::error(&format!("Error plotting cosine function: {}", msg));
        return;
    }
    if let Err(msg) = plt.plot_line(&x_data, &exp_data, canvas::TAB_GREEN, 2.0) {
        console::error(&format!("Error plotting exponential function: {}", msg));
        return;
    }

    plt.show();
}
