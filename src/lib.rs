mod canvas;
mod math;
mod console;
mod plotter;
mod browser;

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

fn gen_cyclic_function() -> (Vec<f32>, Vec<f32>) {
    const BIG_R: f32 = 5.0;
    const SMALL_R: f32 = 3.0;
    const D: f32 = 2.0;

    let mut x = vec![0.0; 400];
    let mut y = vec![0.0; 400];
    for i in 0..400 {
        let angle =  (i as f32) * 6.0 * std::f32::consts::PI / 399.0; // Full cycle
        x[i] = (BIG_R + SMALL_R) * angle.cos() + D * ((BIG_R + SMALL_R) / SMALL_R * angle).cos();
        y[i] = (BIG_R + SMALL_R) * angle.sin() + D * ((BIG_R + SMALL_R) / SMALL_R * angle).sin();
    }
    (x, y)
}

#[no_mangle]
pub fn animate_fourier(cutoff: usize, step: usize) {
    // Create step function data
    let (x, y) = gen_cyclic_function();

    let mx = match math::low_pass_matrix(x.as_slice(), cutoff) {
        Ok(val) => val,
        Err(msg) => { browser::alert(&format!("Error in low-pass matrix: {}", msg)); return; }
    };
    let my = match math::low_pass_matrix(y.as_slice(), cutoff) {
        Ok(val) => val,
        Err(msg) => { browser::alert(&format!("Error in low-pass matrix: {}", msg)); return; }
    };

    let rows = mx.len();
    let cols = mx[0].len();

    let mut arx = vec![0.0; rows];
    let mut ary = vec![0.0; rows];
    for i in 0..rows {
        for j in 0..cols {
            arx[i] += mx[i][j];
            ary[i] += my[i][j];
        }
    }

    let mut plt = plotter::Plotter::new();
    plt.set_x_range(-12.0, 12.0);
    plt.set_y_range(-12.0, 12.0);
    let _ = plt.plot_line(x.as_slice(), y.as_slice(), canvas::TAB_BLUE, 2.0);

    let mut start_x = 0.0;
    let mut start_y = 0.0;
    let mut end_x = 0.0;
    let mut end_y = 0.0;
    for j in 0..cols {
        end_x += mx[step][j];
        end_y += my[step][j];
        let _ = plt.plot_arrow(&[start_x, end_x], &[start_y, end_y], canvas::TAB_GREEN, 2.0);
        start_x = end_x;
        start_y = end_y;

    }
    let _ = plt.plot_line(arx.as_slice(), ary.as_slice(), canvas::TAB_ORANGE, 2.0);

    plt.show();
}