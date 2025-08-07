use crate::math;
use web_canvas::canvas;
use web_canvas::console;

use std::cell::RefCell;
use num_complex::Complex32;

thread_local! {
    // Animation instance for the Fourier series visualization
    static ANIMATION: RefCell<Option<Fourier>> = RefCell::new(None);
}


// Define constants for magic numbers
const LINE_WIDTH_ORIGINAL: f32 = 1.0;
const LINE_WIDTH_RECONSTRUCTED: f32 = 2.0;
const ARROW_WIDTH: f32 = 2.0;
const DEFAULT_SPEED: f64 = 50.0; // points per second at 60 FPS

#[derive(Clone)]
struct Viewport {
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    preserve_aspect_ratio: bool,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            x_min: -10.0,
            x_max: 10.0,
            y_min: -10.0,
            y_max: 10.0,
            preserve_aspect_ratio: true,
        }
    }
}

pub struct Fourier {
    // Fourier data
    fourier: math::Fourier, // Single Fourier transform where real=x, imag=y
    k_min: usize,
    k_max: usize,

    // Canvas
    canvas: canvas::Canvas,

    // Animation control
    current_point: f64,
    point_speed: f64,

    is_paused: bool,
    is_stopped: bool,

    // Viewport for plotting
    viewport: Viewport,
}

/// The `Fourier` struct represents a Fourier series and its animation state.
/// It includes methods for controlling the animation and plotting the Fourier components.
impl Fourier {
    /// Creates a new `Fourier` instance with the given data and cutoff frequency.
    /// Note: This creates new Fourier transforms internally. For better performance
    /// when you already have Fourier transforms, use `from_fourier` instead.
    ///
    /// # Arguments
    /// * `x_data` - A vector of x-coordinates.
    /// * `y_data` - A vector of y-coordinates.
    /// * `k_min` - The minimum frequency to include in the Fourier series.
    /// * `k_max` - The maximum frequency to include in the Fourier series.
    ///
    /// # Returns
    /// A `Result` containing the `Fourier` instance or an error message.
    #[allow(dead_code)] // Keep for API completeness
    pub fn new(x_data: Vec<f32>, y_data: Vec<f32>, k_min: usize, k_max: usize) -> Result<Self, String> {
        if x_data.len() != y_data.len() {
            return Err("X and Y data must have same length".into());
        }

        // Convert x,y to complex numbers where x=real, y=imaginary
        let complex_data: Vec<Complex32> = x_data.iter().zip(y_data.iter())
            .map(|(&x, &y)| Complex32::new(x, y))
            .collect();

        // Create Fourier transforms from complex data
       let fourier =  math::Fourier::from_complex(complex_data)?;
       Self::from_fourier(fourier, k_min, k_max)
    }

    /// Creates a new `Fourier` instance from pre-computed Fourier transforms.
    /// This is more efficient when you already have the Fourier transforms.
    ///
    /// # Arguments
    /// * `fourier` - Pre-computed Fourier transform for complex data (x=real, y=imaginary).
    /// * `k_min` - The minimum frequency to include in the Fourier series.
    /// * `k_max` - The maximum frequency to include in the Fourier series.
    ///
    /// # Returns
    /// A `Result` containing the `Fourier` instance or an error message.
    pub fn from_fourier(fourier: math::Fourier, k_min: usize, k_max: usize) -> Result<Self, String> {
        let fourier_struct = Fourier {
            fourier,
            k_min,
            k_max,
            canvas: canvas::Canvas::from_element("animation-canvas"),
            current_point: 0.0,
            point_speed: DEFAULT_SPEED,
            is_paused: true,
            is_stopped: true,
            viewport: Viewport::default(),
        };

        // Register the event handler - we'll handle animation frame calls
        fourier_struct.canvas.register_handler(AnimationEventHandler);
        Ok(fourier_struct)
    }

    /// Start the self-contained animation loop
    pub fn start(&mut self) {
        self.calculate_viewport();
        self.is_paused = false;
        self.is_stopped = false;

        self.canvas.start_animation_loop();
    }

    /// Stop the animation loop
    pub fn stop(&mut self) {
        self.is_stopped = true;
        self.current_point = 0.0;
        self.canvas.stop_animation_loop();
    }

    /// Advances the animation by one step, updating the plot.
    pub fn step(&mut self, elapsed: f64) {
        // If not running, do nothing
        if self.is_paused { return; }

        self.current_point += self.point_speed * elapsed;
        if self.current_point < 0.0 {
            self.current_point = self.fourier.size() as f64;
        }

        let current_point = (self.fourier.size() + self.current_point as usize) % self.fourier.size();

        self.plot_all(current_point);
    }

    // Control methods
    /// Resumes the animation.
    pub fn play(&mut self) { self.is_paused = false; }
    /// Pauses the animation.
    pub fn pause(&mut self) { self.is_paused = true; }
    /// Checks if the animation is paused.
    pub fn is_paused(&self) -> bool { self.is_paused }
    /// Checks if the animation is stopped.
    pub fn is_stopped(&self) -> bool { self.is_stopped }

    /// Sets the speed of the animation.
    ///
    /// # Arguments
    /// * `speed` - The new speed value.
    pub fn set_speed(&mut self, speed: f64) { self.point_speed = speed; }
    /// Gets the current speed of the animation.
    pub fn speed(&self) -> f64 { self.point_speed }

    /////////////////////////////////////////////////////////////////////////////////////
    /// Private methods for plotting
    /////////////////////////////////////////////////////////////////////////////////////

    /// Calculate viewport based on data bounds and center of mass
    fn calculate_viewport(&mut self) {
        // Calculate center of mass from the original data
        // Since data is already centered in math::Fourier, we just need to find bounds
        let complex_data = self.fourier.original();

        // Find bounds of all data (already centered)
        let mut x_min = f32::INFINITY;
        let mut x_max = f32::NEG_INFINITY;
        let mut y_min = f32::INFINITY;
        let mut y_max = f32::NEG_INFINITY;

        // Check original data (x=real, y=imaginary)
        for complex_val in complex_data {
            if complex_val.re < x_min { x_min = complex_val.re; }
            if complex_val.re > x_max { x_max = complex_val.re; }
            if complex_val.im < y_min { y_min = complex_val.im; }
            if complex_val.im > y_max { y_max = complex_val.im; }
        }

        // Add some padding
        let x_range = x_max - x_min;
        let y_range = y_max - y_min;
        let padding = 0.15; // 15% padding
        x_min -= padding * x_range;
        x_max += padding * x_range;
        y_min -= padding * y_range;
        y_max += padding * y_range;

        // Preserve aspect ratio if needed
        if self.viewport.preserve_aspect_ratio {
            let aspect_ratio = self.canvas.width() / self.canvas.height();
            let data_aspect = (x_max - x_min) / (y_max - y_min);

            if data_aspect > aspect_ratio {
                // Data is wider, expand y range
                let new_y_range = (x_max - x_min) / aspect_ratio;
                let y_center = (y_max + y_min) / 2.0;
                y_min = y_center - new_y_range / 2.0;
                y_max = y_center + new_y_range / 2.0;
            } else {
                // Data is taller, expand x range
                let new_x_range = (y_max - y_min) * aspect_ratio;
                let x_center = (x_max + x_min) / 2.0;
                x_min = x_center - new_x_range / 2.0;
                x_max = x_center + new_x_range / 2.0;
            }
        }

        self.viewport.x_min = x_min;
        self.viewport.x_max = x_max;
        self.viewport.y_min = y_min;
        self.viewport.y_max = y_max;
    }

    /// Convert viewport coordinates to canvas pixel coordinates
    fn viewport_to_canvas(&self, x: f32, y: f32) -> (f32, f32) {
        let x_pos = (x - self.viewport.x_min) / (self.viewport.x_max - self.viewport.x_min) * self.canvas.width();
        let y_pos = self.canvas.height() - (y - self.viewport.y_min) / (self.viewport.y_max - self.viewport.y_min) * self.canvas.height();
        (x_pos, y_pos)
    }

    /// Plot all components of the animation
    fn plot_all(&self, current_point: usize) {
        if !self.check_frequency_range() { return; }

        self.canvas.clear();
        self.plot_dimensional_indicators();
        self.plot_original_curve();
        self.plot_reconstructed_curve(current_point);
        self.plot_fourier_components(current_point);
    }

    /// Draw dimensional indicators like axes and scale markers
    fn plot_dimensional_indicators(&self) {
        // Draw scale markers and labels
        const FONT: &str = "10px monospace";
        let canvas_width = self.canvas.width();
        let canvas_height = self.canvas.height();

        // Draw coordinate axes through the origin (0,0)
        let (origin_x, origin_y) = self.viewport_to_canvas(0.0, 0.0);

        // X-axis (horizontal line through origin)
        if origin_y >= 0.0 && origin_y <= canvas_height {
            self.canvas.draw_line(0.0, origin_y, canvas_width, origin_y, 1.0, canvas::LIGHT_GRAY);
        }

        // Y-axis (vertical line through origin)
        if origin_x >= 0.0 && origin_x <= canvas_width {
            self.canvas.draw_line(origin_x, 0.0, origin_x, canvas_height, 1.0, canvas::LIGHT_GRAY);
        }


        // Calculate nice scale intervals
        let x_range = self.viewport.x_max - self.viewport.x_min;
        let y_range = self.viewport.y_max - self.viewport.y_min;

        let x_interval = self.nice_interval(x_range / 8.0); // About 8 ticks across
        let y_interval = self.nice_interval(y_range / 6.0); // About 6 ticks vertically

        // Draw X-axis scale markers
        let x_start = (self.viewport.x_min / x_interval).floor() * x_interval;
        let mut x_val = x_start;
        while x_val <= self.viewport.x_max {
            if (x_val - 0.0).abs() > x_interval * 0.1 { // Skip origin to avoid clutter
                let (tick_x, tick_y) = self.viewport_to_canvas(x_val, 0.0);
                if tick_x >= 0.0 && tick_x <= canvas_width {
                    // Draw tick mark
                    self.canvas.draw_line(tick_x, tick_y - 3.0, tick_x, tick_y + 3.0, 1.0, canvas::DARK_GRAY);
                    // Draw label
                    let label = if x_val.abs() < 0.01 { "0".to_string() } else { format!("{:.1}", x_val) };
                    let text_width = self.canvas.measure_text_width(&label, FONT);
                    self.canvas.draw_text(&label, tick_x - text_width / 2.0, tick_y + 15.0, FONT, canvas::DARK_GRAY);
                }
            }
            x_val += x_interval;
        }

        // Draw Y-axis scale markers
        let y_start = (self.viewport.y_min / y_interval).floor() * y_interval;
        let mut y_val = y_start;
        while y_val <= self.viewport.y_max {
            if (y_val - 0.0).abs() > y_interval * 0.1 { // Skip origin to avoid clutter
                let (tick_x, tick_y) = self.viewport_to_canvas(0.0, y_val);
                if tick_y >= 0.0 && tick_y <= canvas_height {
                    // Draw tick mark
                    self.canvas.draw_line(tick_x - 3.0, tick_y, tick_x + 3.0, tick_y, 1.0, canvas::DARK_GRAY);
                    // Draw label
                    let label = if y_val.abs() < 0.01 { "0".to_string() } else { format!("{:.1}", y_val) };
                    self.canvas.draw_text(&label, tick_x - 25.0, tick_y + 3.0, FONT, canvas::DARK_GRAY);
                }
            }
            y_val += y_interval;
        }
    }

    fn check_frequency_range(&self) -> bool {
        let max_freq = self.fourier.max_frequency();
        if self.k_min > self.k_max {
            console::error("k_min cannot be greater than k_max");
            return false;
        }
        if self.k_max > max_freq {
            console::error(&format!("k_max cannot exceed maximum frequency: {}", max_freq));
            return false;
        }
        true
    }

    /// Calculate a "nice" interval for scale markings
    fn nice_interval(&self, rough_interval: f32) -> f32 {
        let magnitude = 10_f32.powf(rough_interval.log10().floor());
        let normalized = rough_interval / magnitude;

        let nice_normalized = if normalized < 1.5 {
            1.0
        } else if normalized < 3.0 {
            2.0
        } else if normalized < 7.0 {
            5.0
        } else {
            10.0
        };

        nice_normalized * magnitude
    }

    /// Plots the original curve on the canvas.
    fn plot_original_curve(&self) {
        let complex_orig = self.fourier.original();

        if complex_orig.len() < 2 { return; }

        let mut x_pixels = Vec::with_capacity(complex_orig.len());
        let mut y_pixels = Vec::with_capacity(complex_orig.len());

        for complex_val in complex_orig {
            // Extract x (real) and y (imaginary) parts
            let (x_px, y_px) = self.viewport_to_canvas(complex_val.re, complex_val.im);
            x_pixels.push(x_px);
            y_pixels.push(y_px);
        }

        self.canvas.stroke_curve(&x_pixels, &y_pixels, LINE_WIDTH_ORIGINAL, canvas::TAB_BLUE);
    }

    /// Plots the reconstructed curve up to the current frequency on the canvas.
    fn plot_reconstructed_curve(&self, current_point: usize) {
        let recon_complex = self.fourier.filtered_range(self.k_min, self.k_max).unwrap_or_else(|_| vec![Complex32::new(0.0, 0.0); self.fourier.size()]);

        if current_point < 2 || recon_complex.len() < 2 { return; }

        let end_point = (current_point + 1).min(recon_complex.len());
        let mut x_pixels = Vec::with_capacity(end_point);
        let mut y_pixels = Vec::with_capacity(end_point);

        for i in 0..end_point {
            // Extract x (real) and y (imaginary) parts
            let (x_px, y_px) = self.viewport_to_canvas(recon_complex[i].re, recon_complex[i].im);
            x_pixels.push(x_px);
            y_pixels.push(y_px);
        }

        self.canvas.stroke_curve(&x_pixels, &y_pixels, LINE_WIDTH_RECONSTRUCTED, canvas::TAB_ORANGE);
    }

    /// Plots the Fourier components as vectors on the canvas.
    fn plot_fourier_components(&self, current_point: usize) {
        let mut current_complex = Complex32::new(0.0, 0.0);
        let total_points = self.fourier.size();

        // Draw positive frequencies (including DC)
        for k in self.k_min..=self.k_max {
            let component = self.fourier.get_component(k, current_point);
            let next_complex = current_complex + component;
            let (start_px, start_py) = self.viewport_to_canvas(current_complex.re, current_complex.im);
            let (end_px, end_py) = self.viewport_to_canvas(next_complex.re, next_complex.im);
            self.canvas.draw_arrow(start_px, start_py, end_px, end_py, ARROW_WIDTH, canvas::TAB_GREEN);
            current_complex = next_complex;

            // skip DC for negative frequencies
            if k == 0 { continue; }

            // For negative frequency, use conjugate and negative angle
            let component = self.fourier.get_component(total_points - k, current_point);
            let next_complex = current_complex + component;
            let (start_px, start_py) = self.viewport_to_canvas(current_complex.re, current_complex.im);
            let (end_px, end_py) = self.viewport_to_canvas(next_complex.re, next_complex.im);
            self.canvas.draw_arrow(start_px, start_py, end_px, end_py, ARROW_WIDTH, canvas::TAB_OLIVE);
            current_complex = next_complex;
        }


        // Draw origin marker
        let (origin_x, origin_y) = self.viewport_to_canvas(0.0, 0.0);
        self.canvas.fill_circle(origin_x, origin_y, 2.0, canvas::BLACK);
        // Draw current point indicator
        let (tip_px, tip_py) = self.viewport_to_canvas(current_complex.re, current_complex.im);
        self.canvas.fill_circle(tip_px, tip_py, 3.0, canvas::TAB_RED);
    }

}

struct AnimationEventHandler;

impl canvas::EventHandler for AnimationEventHandler {
    fn on_animation_frame(&mut self, _canvas: &canvas::Canvas, elapsed: f32) {
        ANIMATION.with(|cell| {
            if let Some(ref mut animation) = *cell.borrow_mut() {
                animation.step(elapsed as f64);
            }
        });
    }
}

/// Set the animation instance
pub fn set_animation(animation: Fourier) {
    ANIMATION.with(|cell| {
        *cell.borrow_mut() = Some(animation);
    });
}

/// Clear the animation instance
pub fn clear_animation() {
    ANIMATION.with(|cell| {
        *cell.borrow_mut() = None;
    });
}

/// Play/pause animation with frequency range
pub fn play_pause_animation(k_min: usize, k_max: usize, example_code: usize, init_fn: impl FnOnce(usize, usize, usize)) {
    ANIMATION.with(|cell| {
        let mut borrow = cell.borrow_mut();
        if let Some(ref mut var) = *borrow {
            if var.is_stopped() {
                drop(borrow);
                init_fn(k_min, k_max, example_code);
            } else if var.is_paused() {
                var.play();
            } else if var.speed() > DEFAULT_SPEED || var.speed() < DEFAULT_SPEED {
                var.set_speed(DEFAULT_SPEED);
            } else {
                var.pause();
            }
        } else {
            drop(borrow);
            init_fn(k_min, k_max, example_code);
        }
    });
}

/// Stop the animation
pub fn stop_animation() {
    ANIMATION.with(|cell| {
        if let Some(ref mut var) = *cell.borrow_mut() {
            var.stop();
        }
    });
}

/// Increase animation speed
pub fn increase_animation_speed() {
    ANIMATION.with(|cell| {
        if let Some(ref mut var) = *cell.borrow_mut() {
            var.set_speed(3.0 * var.speed() / 2.0);
        }
    });
}

/// Decrease animation speed
pub fn decrease_animation_speed() {
    ANIMATION.with(|cell| {
        if let Some(ref mut var) = *cell.borrow_mut() {
            var.set_speed( 2.0 * var.speed() / 3.0);
        }
    });
}