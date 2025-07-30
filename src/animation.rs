use crate::math;
use crate::plotter;
use crate::canvas;

mod js {
    #[link(wasm_import_module = "Animation")]
    extern "C" {
        pub fn start_animation_loop(canvas_id: u32);
        pub fn stop_animation_loop(canvas_id: u32);
    }
}

// Define constants for magic numbers
const X_RANGE: (f32, f32) = (-12.0, 12.0);
const Y_RANGE: (f32, f32) = (-12.0, 12.0);
const LINE_WIDTH_ORIGINAL: f32 = 1.0;
const LINE_WIDTH_RECONSTRUCTED: f32 = 2.0;
const ARROW_WIDTH: f32 = 2.0;

pub struct Fourier {
    // Fourier data
    fourier_x: math::Fourier,
    fourier_y: math::Fourier,

    // Canvas
    canvas: canvas::Canvas,

    // Animation control
    current_point: f64,
    point_speed: f64,

    is_paused: bool,
    is_stopped: bool,
}

/// The `Fourier` struct represents a Fourier series and its animation state.
/// It includes methods for controlling the animation and plotting the Fourier components.
impl Fourier {
    /// Creates a new `Fourier` instance with the given data and cutoff frequency.
    ///
    /// # Arguments
    /// * `x_data` - A vector of x-coordinates.
    /// * `y_data` - A vector of y-coordinates.
    /// * `cutoff` - The maximum frequency to include in the Fourier series.
    /// * `canvas_id` - The ID of the canvas to draw on.
    ///
    /// # Returns
    /// A `Result` containing the `Fourier` instance or an error message.
    pub fn new(x_data: Vec<f32>, y_data: Vec<f32>, cutoff: usize, canvas_id: u32) -> Result<Self, String> {
        if x_data.len() != y_data.len() {
            return Err("X and Y data must have same length".into());
        }

        Ok(Fourier {
            fourier_x: math::Fourier::new(x_data, cutoff)?,
            fourier_y: math::Fourier::new(y_data, cutoff)?,
            canvas: canvas::Canvas::new(canvas_id),
            current_point: 0.0,
            point_speed: 1.0,
            is_paused: true,
            is_stopped: true,
        })
    }

    /// Start the self-contained animation loop
    pub fn start(&mut self) {
        unsafe { js::start_animation_loop(self.canvas.id()); }
        self.is_paused = false;
        self.is_stopped = false;
    }

    /// Stop the animation loop
    pub fn stop(&mut self) {
        unsafe { js::stop_animation_loop(self.canvas.id()); }
        self.is_stopped = true;
        self.current_point = 0.0;
    }

    /// Advances the animation by one step, updating the plot.
    pub fn step(&mut self) {
        // If not running, do nothing
        if self.is_paused { return; }

        self.current_point += self.point_speed;
        if self.current_point < 0.0 {
            self.current_point = self.fourier_x.size() as f64;
        }

        let current_point = (self.fourier_x.size() + self.current_point as usize) % self.fourier_x.size();

        let mut plt = Self::setup_plotter_from_canvas(&self.canvas);
        self.plot_original_curve(&mut plt);
        self.plot_reconstructed_curve(&mut plt, current_point);
        self.plot_fourier_components(&mut plt, current_point);

        plt.show();
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

    /// Configures and returns a new plotter instance with predefined ranges.
    fn setup_plotter_from_canvas(canvas: &canvas::Canvas) -> plotter::Plotter {
        let mut plt = plotter::Plotter::new(canvas.id());
        plt.set_x_range(X_RANGE.0, X_RANGE.1);
        plt.set_y_range(Y_RANGE.0, Y_RANGE.1);
        plt
    }

    /// Plots the original curve on the given plotter.
    fn plot_original_curve(&self, plt: &mut plotter::Plotter) {
        let _ = plt.plot_line(
            self.fourier_x.original(),
            self.fourier_y.original(),
            canvas::TAB_BLUE,
            LINE_WIDTH_ORIGINAL,
        );
    }

    /// Plots the reconstructed curve up to the current frequency on the given plotter.
    fn plot_reconstructed_curve(&self, plt: &mut plotter::Plotter, current_point: usize) {
        let mut recon_x = self.fourier_x.filtered().to_vec();
        let mut recon_y = self.fourier_y.filtered().to_vec();
        recon_x.truncate(current_point + 1);
        recon_y.truncate(current_point + 1);
        let _ = plt.plot_line(&recon_x, &recon_y, canvas::TAB_ORANGE, LINE_WIDTH_RECONSTRUCTED);
    }

    /// Plots the Fourier components as vectors on the given plotter.
    fn plot_fourier_components(&self, plt: &mut plotter::Plotter, current_point: usize) {
        let mut current_x = 0.0;
        let mut current_y = 0.0;
        for k in 1..=self.fourier_x.cutoff() {
            if let (Ok(next_x), Ok(next_y)) = (
                self.fourier_x.get_component(k, current_point),
                self.fourier_y.get_component(k, current_point),
            ) {
                let next_x = current_x + next_x;
                let next_y = current_y + next_y;
                let _ = plt.plot_arrow(&[current_x, next_x], &[current_y, next_y], canvas::TAB_GREEN, ARROW_WIDTH);
                current_x = next_x;
                current_y = next_y;
            }
        }
    }
}