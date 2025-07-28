#![allow(dead_code)]

use crate::canvas;

/// Mathematical canvas plotting engine with customizable viewport
pub struct Plotter {
    canvas_width: f32,
    canvas_height: f32,

    viewport: Viewport,
    x_ticks: u32,
    y_ticks: u32,
    font_size: f32,

    // Additional fields can be added for more features like grid lines, axes, etc.
    data: Vec<FunctionData>,
}

impl Plotter {
    /// Create a new plotting canvas with auto-detected dimensions
    pub fn new() -> Self {
        Self {
            canvas_width: canvas::width(),
            canvas_height: canvas::height(),
            viewport: Viewport { x_min: 0.0, x_max: 1.0, y_min: 0.0, y_max: 1.0, x_auto: true, y_auto: true },
            x_ticks: 10,
            y_ticks: 10,
            font_size: 12.0,
            data: Vec::new(),
        }
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

    /// Plot a single function as a line
    pub fn plot_line(&mut self, x_data: &[f32], y_data: &[f32], color: (u8, u8, u8), line_width: f32) -> Result<(), String> {
        if x_data.len() != y_data.len() {
            return Err("x_data and y_data must have the same length".to_string());
        }
        if x_data.len() < 2 {
            return Err("At least two data points are required to plot a line".to_string());
        }
        self.data.push(FunctionData { x_data: x_data.to_vec(), y_data: y_data.to_vec(), color, line_width });
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
            self.set_x_range(x_min - 0.1 * range, x_max + 0.1 * range);
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

        canvas::clear();
        self.draw_grid();
        self.draw_axes();

        for func in &self.data {
            canvas::begin_path();
            canvas::set_stroke_color(func.color.0, func.color.1, func.color.2, 1.0);
            canvas::set_line_width(func.line_width);

            // Move to first point
            let (x_pixel, y_pixel) = self.transform_point(func.x_data[0], func.y_data[0]);
            canvas::move_to(x_pixel, y_pixel);

            // Draw lines to subsequent points
            for i in 1..func.x_data.len() {
                let (x_pixel, y_pixel) = self.transform_point(func.x_data[i], func.y_data[i]);
                canvas::line_to(x_pixel, y_pixel);
            }

            canvas::stroke();
        }
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////// PRIVATE //////////////////////////////////////////////////////////////

    /// Convert mathematical coordinates to canvas pixel coordinates
    fn transform_point(&self, x: f32, y: f32) -> (f32, f32) {
        let x_norm = (x - self.viewport.x_min) / (self.viewport.x_max - self.viewport.x_min) * self.canvas_width;
        let y_norm = self.canvas_height - (y - self.viewport.y_min) / (self.viewport.y_max - self.viewport.y_min) * self.canvas_height;
        (x_norm, y_norm)
    }

    /// Draw grid lines for reference
    fn draw_grid(&self) {
        canvas::set_stroke_color(canvas::LIGHT_GRAY.0, canvas::LIGHT_GRAY.1, canvas::LIGHT_GRAY.2, 0.3);
        canvas::set_line_width(1.0);

        // Vertical grid lines
        for i in 0..=self.x_ticks {
            let x = self.viewport.x_min + (self.viewport.x_max - self.viewport.x_min) * i as f32 / self.x_ticks as f32;
            let (x_pixel, _) = self.transform_point(x, 0.0);

            canvas::begin_path();
            canvas::move_to(x_pixel, 0.0);
            canvas::line_to(x_pixel, self.canvas_height);
            canvas::stroke();
        }

        // Horizontal grid lines
        for i in 0..=self.y_ticks {
            let y = self.viewport.y_min + (self.viewport.y_max - self.viewport.y_min) * i as f32 / self.y_ticks as f32;
            let (_, y_pixel) = self.transform_point(0.0, y);

            canvas::begin_path();
            canvas::move_to(0.0, y_pixel);
            canvas::line_to(self.canvas_width, y_pixel);
            canvas::stroke();
        }
    }

    /// Draw axes (X and Y axis lines)
    pub fn draw_axes(&self) {
        canvas::set_stroke_color(canvas::BLACK.0, canvas::BLACK.1, canvas::BLACK.2, 1.0);
        canvas::set_line_width(2.0);

        // Set up text drawing
        canvas::set_fill_color(canvas::BLACK.0, canvas::BLACK.1, canvas::BLACK.2, 1.0);
        canvas::set_font(&format!("{}px monospace", self.font_size));
        canvas::set_text_align("center");
        let tick_length = self.font_size / 2.0;

        // X-axis /////////////////////////////////////////////////////////////
        if self.viewport.y_min <= 0.0 && self.viewport.y_max >= 0.0 {
            let (x_start, y_axis) = self.transform_point(self.viewport.x_min, 0.0);
            let (x_end, _) = self.transform_point(self.viewport.x_max, 0.0);

            canvas::begin_path();
            canvas::move_to(x_start, y_axis);
            canvas::line_to(x_end, y_axis);
            canvas::stroke();
        }

        // Ticks and labels
        if self.viewport.y_min <= 0.0 && self.viewport.y_max >= 0.0 {
            let (_, y_axis) = self.transform_point(0.0, 0.0);

            for i in 0..=self.x_ticks {
                let x_val = self.viewport.x_min + (self.viewport.x_max - self.viewport.x_min) * i as f32 / self.x_ticks as f32;
                let (x_pixel, _) = self.transform_point(x_val, 0.0);

                // Draw tick mark
                canvas::begin_path();
                canvas::move_to(x_pixel, y_axis - tick_length / 2.0);
                canvas::line_to(x_pixel, y_axis + tick_length / 2.0);
                canvas::stroke();

                // Draw label
                if x_val.abs() < 0.001 {
                    canvas::fill_text("0", x_pixel, y_axis + self.font_size + 5.0);
                } else {
                    canvas::fill_text(&format!("{:.1}", x_val), x_pixel, y_axis + self.font_size + 5.0);
                }
            }
        }

        // Y-axis /////////////////////////////////////////////////////////////
        if self.viewport.x_min <= 0.0 && self.viewport.x_max >= 0.0 {
            let (x_axis, y_start) = self.transform_point(0.0, self.viewport.y_min);
            let (_, y_end) = self.transform_point(0.0, self.viewport.y_max);

            canvas::begin_path();
            canvas::move_to(x_axis, y_start);
            canvas::line_to(x_axis, y_end);
            canvas::stroke();
        }

        // Ticks and labels
        if self.viewport.x_min <= 0.0 && self.viewport.x_max >= 0.0 {
            let (x_axis, _) = self.transform_point(0.0, 0.0);
            canvas::set_text_align("right");

            for i in 0..=self.y_ticks {
                let y_val = self.viewport.y_min + (self.viewport.y_max - self.viewport.y_min) * i as f32 / self.y_ticks as f32;
                let (_, y_pixel) = self.transform_point(0.0, y_val);

                // Skip the origin to avoid overlap
                if y_val.abs() < 0.001 {
                    continue;
                }

                // Draw tick mark
                canvas::begin_path();
                canvas::move_to(x_axis - tick_length / 2.0, y_pixel);
                canvas::line_to(x_axis + tick_length / 2.0, y_pixel);
                canvas::stroke();

                // Draw label
                canvas::fill_text(&format!("{:.1}", y_val), x_axis - 10.0, y_pixel + self.font_size / 3.0);
            }
        }
    }
}

/// Data structure for a single function to plot
struct FunctionData {
    x_data: Vec<f32>,
    y_data: Vec<f32>,
    color: (u8, u8, u8), // RGB color
    line_width: f32,
}

struct Viewport {
    /// X-axis range
    x_min: f32,
    x_max: f32,
    // automatically calculated based on data
    x_auto: bool,

    /// Y-axis range
    y_min: f32,
    y_max: f32,
    // automatically calculated based on data
    y_auto: bool,
}