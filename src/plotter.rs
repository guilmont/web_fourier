#![allow(dead_code)]

use web_canvas::canvas;
use web_canvas::canvas::Canvas;


/// Mathematical canvas plotting engine with customizable viewport
pub struct Plotter {
    canvas: Canvas, // Own the `Canvas` object instead of borrowing it

    viewport: Viewport,
    x_ticks: u32,
    y_ticks: u32,

    font_family: String,
    font_size: f32,

    // Additional fields can be added for more features like grid lines, axes, etc.
    data: Vec<FunctionData>,
}

impl Plotter {
    /// Creates or retrieves a `Plotter` instance for the given canvas.
    pub fn get_or_create(canvas_name: &str) -> &mut Plotter {

        // Check if canvas already contains a plotter with the same ID
        PLOTTER_REGISTRY.with(move |reg| {
            let mut registry = reg.borrow_mut();
            let canvas = Canvas::from_element(canvas_name);
            let plotter = registry.entry(canvas.id()).or_insert_with(move || {
                let plotter = Plotter {
                    canvas: Canvas::from_element(canvas_name),
                    viewport: Viewport {
                        x_min: 0.0,
                        x_max: 1.0,
                        y_min: 0.0,
                        y_max: 1.0,
                        x_auto: true,
                        y_auto: true,
                        preserve_aspect_ratio: false,
                        on_update: None
                    },
                    x_ticks: 10,
                    y_ticks: 10,
                    font_family: "monospace".to_string(),
                    font_size: 12.0,
                    data: Vec::new(),
                };
                plotter.canvas.register_handler(PlotterEvents::new());
                plotter
            });

            // Clear previous data to avoid accumulation
            plotter.data.clear();
            plotter.viewport.x_auto = true;
            plotter.viewport.y_auto = true;

            // SAFETY: We assume that the `Plotter` is unique and not shared across threads.
            unsafe { &mut *(plotter as *mut Plotter) }
        })
    }

    /// Set the X-axis display range
    pub fn set_x_range(&mut self, x_min: f32, x_max: f32) {
        self.viewport.x_min = x_min;
        self.viewport.x_max = x_max;
        self.viewport.x_auto = false; // Disable auto-scaling
    }

    /// Set the Y-axis display range
    pub fn set_y_range(&mut self, y_min: f32, y_max: f32) {
        self.viewport.y_min = y_min;
        self.viewport.y_max = y_max;
        self.viewport.y_auto = false; // Disable auto-scaling
    }

    /// Set the number of ticks on the X-axis
    pub fn set_x_ticks(&mut self, ticks: u32) { self.x_ticks = ticks; }
    /// Set the number of ticks on the Y-axis
    pub fn set_y_ticks(&mut self, ticks: u32) { self.y_ticks = ticks; }
    /// Set the font size for text rendering
    pub fn set_font_size(&mut self, size: f32) { self.font_size = size; }
    /// Set preserve aspect ratio when drawing
    /// This is useful for ensuring that circles appear as circles, etc.
    pub fn preserve_aspect_ratio(&mut self, preserve: bool) { self.viewport.preserve_aspect_ratio = preserve; }

    /// Reset zoom to auto-range (fit all data)
    pub fn reset_zoom(&mut self) {
        self.viewport.x_auto = true;
        self.viewport.y_auto = true;
        self.show();
    }


    /// Zoom around a specific point with independent X and Y factors
    pub fn zoom_at_point(&mut self, x_factor: f32, y_factor: f32, center_x: f32, center_y: f32) {
        // Calculate the current viewport dimensions
        let current_width = self.viewport.x_max - self.viewport.x_min;
        let current_height = self.viewport.y_max - self.viewport.y_min;

        // Calculate the new viewport dimensions
        let new_width = current_width / x_factor;
        let new_height = current_height / y_factor;

        // Calculate how much the zoom point should move relative to viewport edges
        let x_ratio = (center_x - self.viewport.x_min) / current_width;
        let y_ratio = (center_y - self.viewport.y_min) / current_height;

        // Set new viewport boundaries to keep the zoom point fixed
        self.viewport.x_min = center_x - new_width * x_ratio;
        self.viewport.x_max = center_x + new_width * (1.0 - x_ratio);
        self.viewport.y_min = center_y - new_height * y_ratio;
        self.viewport.y_max = center_y + new_height * (1.0 - y_ratio);

        self.viewport.x_auto = false;
        self.viewport.y_auto = false;
        self.show();
    }

    /// Plot a single function as a line
    pub fn plot_line(&mut self, x_data: &[f32], y_data: &[f32], color: (u8, u8, u8), line_width: f32) -> Result<(), String> {
        if x_data.len() != y_data.len() {
            return Err("x_data and y_data must have the same length".to_string());
        }
        if x_data.len() < 2 {
            return Err("At least two data points are required to plot a line".to_string());
        }
        self.data.push(FunctionData { style: FunctionType::LINE, x_data: x_data.to_vec(), y_data: y_data.to_vec(), color, line_width, bar_width: 0.0 });
        Ok(())
    }

    pub fn plot_arrow(&mut self, x_data: &[f32], y_data: &[f32], color: (u8, u8, u8), line_width: f32) -> Result<(), String> {
        if x_data.len() == 2 && y_data.len() == 2 {
            self.data.push(FunctionData { style: FunctionType::ARROW, x_data: x_data.to_vec(), y_data: y_data.to_vec(), color, line_width, bar_width: 0.0 });
            Ok(())
        } else {
            Err("x_data and y_data must have exactly two points for arrows".to_string())
        }
    }

    /// Plot a histogram (bar plot) given x (bin centers) and y (heights)
    pub fn plot_histogram(&mut self, x_data: &[f32], y_data: &[f32], color: (u8, u8, u8), bar_width: f32) -> Result<(), String> {
        if x_data.len() != y_data.len() {
            return Err("x_data and y_data must have the same length".to_string());
        }
        if x_data.len() < 1 {
            return Err("At least one data point is required to plot a histogram".to_string());
        }
        self.data.push(FunctionData {
            style: FunctionType::HISTOGRAM,
            x_data: x_data.to_vec(),
            y_data: y_data.to_vec(),
            color,
            line_width: 1.0,
            bar_width
        });

        Ok(())
    }

    /// Plot multiple functions on the same canvas with different colors
    pub fn show(&mut self) {

        if self.viewport.x_auto {
            // Automatically calculate X range based on data
            let (mut x_min, mut x_max) = (f32::INFINITY, f32::NEG_INFINITY);
            for x in self.data.iter().flat_map(|f| f.x_data.iter()).copied() {
                if x < x_min { x_min = x; }
                if x > x_max { x_max = x; }
            }
            let range = x_max - x_min;
            self.set_x_range(x_min - 0.1 * range, x_max + 0.01 * range);
        }

        if self.viewport.y_auto {
            // Automatically calculate Y range based on data
            let (mut y_min, mut y_max) = (f32::INFINITY, f32::NEG_INFINITY);
            for y in self.data.iter().flat_map(|f| f.y_data.iter()).copied() {
                if y < y_min { y_min = y; }
                if y > y_max { y_max = y; }
            }
            let range = y_max - y_min;
            self.set_y_range(y_min - 0.1 * range, y_max + 0.1 * range);
        }

        // Adjust viewport for aspect ratio if needed
        if self.viewport.preserve_aspect_ratio {
            let x_range = self.viewport.x_max - self.viewport.x_min;
            let y_range = self.viewport.y_max - self.viewport.y_min;
            let aspect_ratio = self.canvas.width() / self.canvas.height();
            if x_range / y_range > aspect_ratio {
                // X range is too wide, adjust Y range
                let new_y_range = x_range / aspect_ratio;
                let y_center = (self.viewport.y_max + self.viewport.y_min) / 2.0;
                self.set_y_range(y_center - new_y_range / 2.0, y_center + new_y_range / 2.0);
            } else {
                // Y range is too wide, adjust X range
                let new_x_range = y_range * aspect_ratio;
                let x_center = (self.viewport.x_max + self.viewport.x_min) / 2.0;
                self.set_x_range(x_center - new_x_range / 2.0, x_center + new_x_range / 2.0);
            }
        }

        self.canvas.clear();
        self.draw_grid();
        self.draw_axes();

        for func in &self.data {
            match func.style {
                FunctionType::LINE => {
                    // Convert data to pixel coordinates
                    let capacity = func.x_data.len();
                    let mut x_pixels = Vec::<f32>::with_capacity(capacity);
                    let mut y_pixels = Vec::<f32>::with_capacity(capacity);
                    for k in 0..capacity {
                        let (x, y) = self.viewport_to_canvas(func.x_data[k], func.y_data[k]);
                        x_pixels.push(x);
                        y_pixels.push(y);
                    }
                    self.canvas.stroke_curve(&x_pixels, &y_pixels, func.line_width, func.color);
                },
                FunctionType::ARROW => {
                    let (start_x, start_y) = self.viewport_to_canvas(func.x_data[0], func.y_data[0]);
                    let (end_x, end_y) = self.viewport_to_canvas(func.x_data[1], func.y_data[1]);
                    self.canvas.draw_arrow(start_x, start_y, end_x, end_y, func.line_width, func.color);
                },
                FunctionType::HISTOGRAM => {
                    // For each bin, draw a vertical bar centered at x_data[i] with height y_data[i]
                    let x_data = &func.x_data;
                    let y_data = &func.y_data;
                    let color = &func.color;
                    let bar_width = func.bar_width;

                    for i in 0..x_data.len() {
                        // Calculate left and right edges of the bar, centered at x_data[i]
                        let half_width = bar_width / 2.0;
                        let left_edge = x_data[i] - half_width;
                        let right_edge = x_data[i] + half_width;

                        let (x0, y0) = self.viewport_to_canvas(left_edge, 0.0);
                        let (x1, y1) = self.viewport_to_canvas(right_edge, y_data[i]);
                        self.canvas.fill_rect(x0, y0, x1-x0, y1 - y0, 0.0, *color);
                    }
                }
            }
        }
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////// PRIVATE //////////////////////////////////////////////////////////////


    /// Convert viewport coordinates to canvas pixel coordinates
    fn viewport_to_canvas(&self, x: f32, y: f32) -> (f32, f32) {
        let vp = &self.viewport;
        let x_pos = (x - vp.x_min) / (vp.x_max - vp.x_min) * self.canvas.width();
        let y_pos = self.canvas.height() - (y - vp.y_min) / (vp.y_max - vp.y_min) * self.canvas.height();
        (x_pos, y_pos)
    }

    /// Convert canvas pixel coordinates to viewport coordinates
    fn canvas_to_viewport(&self, x: f32, y: f32) -> (f32, f32) {
        let vp = &self.viewport;
        let x_pos = vp.x_min + (x / self.canvas.width()) * (vp.x_max - vp.x_min);
        let y_pos = vp.y_max - (y / self.canvas.height()) * (vp.y_max - vp.y_min);
        (x_pos, y_pos)
    }

    /// Display a text box with coordinates (in plotter space) at the top right of the plot.
    fn show_coordinates(&self, x: f32, y: f32) {
        // Format coordinates in plotter (math) space
        let (x_pos, y_pos) = self.canvas_to_viewport(x, y);

        // Text measurement
        let text = format!("({:.2}, {:.2})", x_pos, y_pos);
        let font = format!("{}px {}", self.font_size, self.font_family);
        let width = self.canvas.measure_text_width(&text, &font);
        let height = self.font_size; // More padding
        let margin = self.font_size;

        // Top right in canvas pixel coordinates
        let x_px = self.canvas.width() - width;
        self.canvas.clear_rect(x_px - margin, 0.0, width + margin, height + margin);
        self.canvas.draw_text(&text, x_px, height, &font, canvas::BLACK);
    }

    /// Draw grid lines for reference
    fn draw_grid(&self) {
        const GRID_COLOR: (u8, u8, u8) = (231, 231, 231); // Light gray
        // Vertical grid lines
        for i in 0..=self.x_ticks {
            let x = self.viewport.x_min + (self.viewport.x_max - self.viewport.x_min) * i as f32 / self.x_ticks as f32;
            let (x_pixel, _) = self.viewport_to_canvas(x, 0.0);

            self.canvas.draw_line(x_pixel, 0.0, x_pixel, self.canvas.height(), 1.0, GRID_COLOR);
        }

        // Horizontal grid lines
        for i in 0..=self.y_ticks {
            let y = self.viewport.y_min + (self.viewport.y_max - self.viewport.y_min) * i as f32 / self.y_ticks as f32;
            let (_, y_pixel) = self.viewport_to_canvas(0.0, y);

            self.canvas.draw_line(0.0, y_pixel, self.canvas.width(), y_pixel, 1.0, GRID_COLOR);
        }
    }

    /// Draw axes (X and Y axis lines)
    pub fn draw_axes(&self) {
        // Set up text drawing
        let tick_length = self.font_size / 2.0;
        let font = format!("{}px {}", self.font_size, self.font_family);

        // X-axis /////////////////////////////////////////////////////////////
        if self.viewport.y_min <= 0.0 && self.viewport.y_max >= 0.0 {
            let (x_start, y_axis) = self.viewport_to_canvas(self.viewport.x_min, 0.0);
            let (x_end, _) = self.viewport_to_canvas(self.viewport.x_max, 0.0);

            self.canvas.draw_line(x_start, y_axis, x_end, y_axis, 2.0, canvas::BLACK);
        }

        // Ticks and labels
        if self.viewport.y_min <= 0.0 && self.viewport.y_max >= 0.0 {
            for i in 0..=self.x_ticks {
                let x_val = self.viewport.x_min + (self.viewport.x_max - self.viewport.x_min) * i as f32 / self.x_ticks as f32;
                let (x_pixel, y_pixel) = self.viewport_to_canvas(x_val, 0.0);

                // Draw tick mark
                self.canvas.draw_line(x_pixel, y_pixel - tick_length / 2.0, x_pixel, y_pixel + tick_length / 2.0, 2.0, canvas::BLACK);

                // Draw label
                let label = format!("{:.2}", x_val);
                let label_width = self.canvas.measure_text_width(&label, &font);
                self.canvas.draw_text(&label, x_pixel - label_width / 2.0, y_pixel + 1.5 * self.font_size, &font, canvas::BLACK);
            }
        }

        // Y-axis /////////////////////////////////////////////////////////////
        if self.viewport.x_min <= 0.0 && self.viewport.x_max >= 0.0 {
            let (x_axis, y_start) = self.viewport_to_canvas(0.0, self.viewport.y_min);
            let (_, y_end) = self.viewport_to_canvas(0.0, self.viewport.y_max);

            self.canvas.draw_line(x_axis, y_start, x_axis, y_end, 2.0, canvas::BLACK);
        }

        // Ticks and labels
        if self.viewport.x_min <= 0.0 && self.viewport.x_max >= 0.0 {
            for i in 0..=self.y_ticks {
                let y_val = self.viewport.y_min + (self.viewport.y_max - self.viewport.y_min) * i as f32 / self.y_ticks as f32;
                let (x_pixel, y_pixel) = self.viewport_to_canvas(0.0, y_val);

                // Skip the origin to avoid overlap
                if y_val.abs() < 0.001 { continue; }

                // Draw tick mark
                self.canvas.draw_line(x_pixel - tick_length / 2.0, y_pixel, x_pixel + tick_length / 2.0, y_pixel, 2.0, canvas::BLACK);

                // Draw label
                let label = format!("{:.2}", y_val);
                let label_width = self.canvas.measure_text_width(&label, &font);
                self.canvas.draw_text(&label, x_pixel - label_width - self.font_size / 2.0, y_pixel + self.font_size / 3.0, &font, canvas::BLACK);
            }
        }
    }
}

/// Private helper functions /////////////////////////////////////////////////////////////////
use std::cell::RefCell;
use std::collections::HashMap;

enum FunctionType {
    LINE,
    ARROW,
    HISTOGRAM,
}

/// Data structure for a single function to plot
struct FunctionData {
    style: FunctionType,
    /// X and Y data points for the function
    x_data: Vec<f32>,
    y_data: Vec<f32>,
    // RGB color
    color: (u8, u8, u8),
    /// Line width for the function
    line_width: f32,
    /// Histogram bar width (used for histogram style)
    bar_width: f32,
}

struct UpdateViewport {
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    operation: ViewportOperation,
}

enum ViewportOperation {
    Zoom,
    Pan,
}

struct Viewport {
    /// X-axis range
    x_min: f32,
    x_max: f32,
    x_auto: bool,
    y_min: f32,
    y_max: f32,
    y_auto: bool,
    preserve_aspect_ratio: bool,

    /// Optional viewport update for dynamic resizing or adjustments
    on_update: Option<UpdateViewport>,
}

thread_local! {
    // Global registry for Plotter instances by canvas_id (WASM: single-threaded, so RefCell is fine)
    static PLOTTER_REGISTRY: RefCell<HashMap<u32, Plotter>> = RefCell::new(HashMap::new());
}

struct PlotterEvents {
    // Keyboard state tracking for modifier keys
    ctrl_pressed: bool,
    shift_pressed: bool,
}

impl PlotterEvents {
    pub fn new() -> Self {
        PlotterEvents {
            ctrl_pressed: false,
            shift_pressed: false,
        }
    }
}

impl canvas::EventHandler for PlotterEvents {
    fn on_mouse_move(&mut self, canvas: &canvas::Canvas, x: f32, y: f32) {
        PLOTTER_REGISTRY.with(|reg| {
            if let Some(plotter) = reg.borrow_mut().get_mut(&canvas.id()) {
                // Always display coordinates in plotter space
                plotter.show_coordinates(x, y);

                // Handle active viewport operations
                if let Some(view) = &plotter.viewport.on_update {
                    match view.operation {
                        ViewportOperation::Pan => {
                            // For panning, update viewport in real-time
                            let (new_x, new_y) = plotter.canvas_to_viewport(x, y);
                            let dx = new_x - view.x_min;
                            let dy = new_y - view.y_min;

                            plotter.viewport.x_min -= dx;
                            plotter.viewport.x_max -= dx;
                            plotter.viewport.y_min -= dy;
                            plotter.viewport.y_max -= dy;
                            plotter.viewport.x_auto = false;
                            plotter.viewport.y_auto = false;

                            plotter.show();
                        }
                        _ => {}
                    }
                }
            }
        });
    }

    fn on_mouse_down(&mut self, canvas: &Canvas, x: f32, y: f32, button: canvas::MouseButton) {
        PLOTTER_REGISTRY.with(|reg| {
            if let Some(plotter) = reg.borrow_mut().get_mut(&canvas.id()) {
                let (x, y) = plotter.canvas_to_viewport(x, y);
                match button {
                    canvas::MouseButton::Left => {
                        // start pan operation with left mouse
                        plotter.viewport.on_update = Some(UpdateViewport {
                            x_min: x, x_max: x, y_min: y, y_max: y,
                            operation: ViewportOperation::Pan
                        });
                    },
                    canvas::MouseButton::Right => {
                        // Reset zoom to auto-range
                        plotter.reset_zoom();
                    },
                    _ => {}
                }
            }
        });
    }

    fn on_mouse_up(&mut self, canvas: &Canvas, _x: f32, _y: f32, _button: canvas::MouseButton) {
        PLOTTER_REGISTRY.with(|reg| {
            if let Some(plotter) = reg.borrow_mut().get_mut(&canvas.id()) {
                // End any active pan operation
                plotter.viewport.on_update = None;
                plotter.show();
            }
        });
    }

    fn on_key_down(&mut self, canvas: &Canvas, key_code: canvas::KeyCode) {
        PLOTTER_REGISTRY.with(|reg| {
            if let Some(plotter) = reg.borrow_mut().get_mut(&canvas.id()) {
                match key_code {
                    canvas::KeyCode::Key0  => { plotter.reset_zoom();      },
                    canvas::KeyCode::Ctrl  => { self.ctrl_pressed = true;  },
                    canvas::KeyCode::Shift => { self.shift_pressed = true; },
                    _ => {}
                }
            }
        });
    }

    fn on_key_up(&mut self, canvas: &Canvas, key_code: canvas::KeyCode) {
        PLOTTER_REGISTRY.with(|reg| {
            if let Some(_plotter) = reg.borrow_mut().get_mut(&canvas.id()) {
                match key_code {
                    canvas::KeyCode::Ctrl  => { self.ctrl_pressed = false;  },
                    canvas::KeyCode::Shift => { self.shift_pressed = false; },
                    _ => {}
                }
            }
        });
    }

    fn on_wheel(&mut self, canvas: &Canvas, x: f32, y: f32, delta_y: f32) {
        PLOTTER_REGISTRY.with(|reg| {
            if let Some(plotter) = reg.borrow_mut().get_mut(&canvas.id()) {
                let (viewport_x, viewport_y) = plotter.canvas_to_viewport(x, y);

                let factor = if delta_y > 0.0 { 0.9 } else { 1.1 };
                let (x_factor, y_factor) =
                    if self.ctrl_pressed { (factor, 1.0) }
                    else if self.shift_pressed { (1.0, factor) }
                    else { (factor, factor) };

                // Apply the appropriate zoom based on modifier keys
                plotter.zoom_at_point(x_factor, y_factor, viewport_x, viewport_y);
            }
        });
    }
}
