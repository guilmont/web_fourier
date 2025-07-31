#![allow(dead_code)]

use crate::canvas;

/// Mathematical canvas plotting engine with customizable viewport
pub struct Plotter {
    canvas: canvas::Canvas,
    canvas_width: f32,
    canvas_height: f32,

    viewport: Viewport,
    x_ticks: u32,
    y_ticks: u32,
    font_size: f32,

    hide_axes: bool,

    // Additional fields can be added for more features like grid lines, axes, etc.
    data: Vec<FunctionData>,
}

impl Plotter {
    /// Create a new plotting canvas with auto-detected dimensions
    pub fn new(canvas_id: u32) -> Self {
        let canvas = canvas::Canvas::new(canvas_id);
        let canvas_width = canvas.width();
        let canvas_height = canvas.height();

        let viewport = Viewport {
            x_min: 0.0,
            x_max: 1.0,
            y_min: 0.0,
            y_max: 1.0,
            x_auto: true,
            y_auto: true,
            preserve_aspect_ratio: false,
        };

        // Initialize viewport to full canvas size
        Self {
            canvas,
            canvas_width,
            canvas_height,
            viewport,
            x_ticks: 10,
            y_ticks: 10,
            font_size: 12.0,
            hide_axes: false,
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
    /// Hide the axes
    pub fn hide_axes(&mut self) { self.hide_axes = true; }
    /// Set preserve aspect ratio when drawing
    /// This is useful for ensuring that circles appear as circles, etc.
    pub fn preserve_aspect_ratio(&mut self, preserve: bool) { self.viewport.preserve_aspect_ratio = preserve; }

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

        // Adjust viewport for aspect ratio if needed
        if self.viewport.preserve_aspect_ratio {
            let x_range = self.viewport.x_max - self.viewport.x_min;
            let y_range = self.viewport.y_max - self.viewport.y_min;
            let aspect_ratio = self.canvas_width / self.canvas_height;
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
        if !self.hide_axes {
            self.draw_grid();
            self.draw_axes();
        }

        for func in &self.data {
            match func.style {
                FunctionType::LINE => {
                    self.canvas.begin_path();
                    self.canvas.set_stroke_color(func.color.0, func.color.1, func.color.2, 1.0);
                    self.canvas.set_line_width(func.line_width);
                    // Move to first point
                    let (x_pixel, y_pixel) = self.transform_point(func.x_data[0], func.y_data[0]);
                    self.canvas.move_to(x_pixel, y_pixel);
                    // Draw lines to subsequent points
                    for i in 1..func.x_data.len() {
                        let (x_pixel, y_pixel) = self.transform_point(func.x_data[i], func.y_data[i]);
                        self.canvas.line_to(x_pixel, y_pixel);
                    }
                    self.canvas.stroke();
                },
                FunctionType::ARROW => {
                    let start_x = self.transform_point(func.x_data[0], func.y_data[0]);
                    let end_x = self.transform_point(func.x_data[1], func.y_data[1]);
                    self.canvas.draw_arrow(start_x.0, start_x.1, end_x.0, end_x.1, func.color, func.line_width);
                },
                FunctionType::HISTOGRAM => {
                    // For each bin, draw a vertical bar centered at x_data[i] with height y_data[i]
                    let x_data = &func.x_data;
                    let y_data = &func.y_data;
                    let color = &func.color;
                    let bar_width = func.bar_width;
                    self.canvas.set_fill_color(color.0, color.1, color.2, 0.8);
                    self.canvas.set_line_width(func.line_width);

                    for i in 0..x_data.len() {
                        // Calculate left and right edges of the bar
                        let (x0, y0) = self.transform_point(x_data[i], 0.0);
                        let (x1, y1) = self.transform_point(x_data[i] + bar_width, y_data[i]);
                        self.canvas.fill_rect(x0, y0, x1-x0, y1 - y0);
                    }
                }
            }
        }
    }

    /// Display a text box with coordinates (in plotter space) at the top right of the plot.
    pub fn show_coordinates(&self, x: f32, y: f32) {
        // Format coordinates in plotter (math) space
        let vp = &self.viewport;
        let x_pos = vp.x_min + (x / self.canvas.width()) * (vp.x_max - vp.x_min);
        let y_pos = vp.y_max - (y / self.canvas.height()) * (vp.y_max - vp.y_min);

        // Text measurement
        let text = format!("({:.2}, {:.2})", x_pos, y_pos);
        let font = format!("{}px monospace", self.font_size);
        let width = self.canvas.measure_text_width(&text, &font) * 1.5; // More padding
        let height = self.font_size * 1.7; // More padding
        let margin = self.font_size * 0.5;

        // Top right in canvas pixel coordinates
        let x_px = self.canvas_width - margin;
        let y_px = self.font_size + margin;
        self.canvas.clear_rect(x_px - width, 0.0, width + margin, height + margin * 0.5);
        self.canvas.set_fill_color(crate::canvas::BLACK.0, crate::canvas::BLACK.1, crate::canvas::BLACK.2, 1.0);
        self.canvas.set_text_align("right");
        self.canvas.set_font(&font);
        self.canvas.fill_text(&text, x_px, y_px);
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
        self.canvas.set_stroke_color(canvas::LIGHT_GRAY.0, canvas::LIGHT_GRAY.1, canvas::LIGHT_GRAY.2, 0.3);
        self.canvas.set_line_width(1.0);

        // Vertical grid lines
        for i in 0..=self.x_ticks {
            let x = self.viewport.x_min + (self.viewport.x_max - self.viewport.x_min) * i as f32 / self.x_ticks as f32;
            let (x_pixel, _) = self.transform_point(x, 0.0);

            self.canvas.begin_path();
            self.canvas.move_to(x_pixel, 0.0);
            self.canvas.line_to(x_pixel, self.canvas_height);
            self.canvas.stroke();
        }

        // Horizontal grid lines
        for i in 0..=self.y_ticks {
            let y = self.viewport.y_min + (self.viewport.y_max - self.viewport.y_min) * i as f32 / self.y_ticks as f32;
            let (_, y_pixel) = self.transform_point(0.0, y);

            self.canvas.begin_path();
            self.canvas.move_to(0.0, y_pixel);
            self.canvas.line_to(self.canvas_width, y_pixel);
            self.canvas.stroke();
        }
    }

    /// Draw axes (X and Y axis lines)
    pub fn draw_axes(&self) {
        self.canvas.set_stroke_color(canvas::BLACK.0, canvas::BLACK.1, canvas::BLACK.2, 1.0);
        self.canvas.set_line_width(2.0);

        // Set up text drawing
        self.canvas.set_fill_color(canvas::BLACK.0, canvas::BLACK.1, canvas::BLACK.2, 1.0);
        self.canvas.set_font(&format!("{}px monospace", self.font_size));
        self.canvas.set_text_align("center");
        let tick_length = self.font_size / 2.0;

        // X-axis /////////////////////////////////////////////////////////////
        if self.viewport.y_min <= 0.0 && self.viewport.y_max >= 0.0 {
            let (x_start, y_axis) = self.transform_point(self.viewport.x_min, 0.0);
            let (x_end, _) = self.transform_point(self.viewport.x_max, 0.0);

            self.canvas.begin_path();
            self.canvas.move_to(x_start, y_axis);
            self.canvas.line_to(x_end, y_axis);
            self.canvas.stroke();
        }

        // Ticks and labels
        if self.viewport.y_min <= 0.0 && self.viewport.y_max >= 0.0 {
            let (_, y_axis) = self.transform_point(0.0, 0.0);

            for i in 0..=self.x_ticks {
                let x_val = self.viewport.x_min + (self.viewport.x_max - self.viewport.x_min) * i as f32 / self.x_ticks as f32;
                let (x_pixel, _) = self.transform_point(x_val, 0.0);

                // Draw tick mark
                self.canvas.begin_path();
                self.canvas.move_to(x_pixel, y_axis - tick_length / 2.0);
                self.canvas.line_to(x_pixel, y_axis + tick_length / 2.0);
                self.canvas.stroke();

                // Draw label
                if x_val.abs() < 0.001 {
                    self.canvas.fill_text("0", x_pixel, y_axis + self.font_size + 5.0);
                } else {
                    self.canvas.fill_text(&format!("{:.2}", x_val), x_pixel, y_axis + self.font_size + 5.0);
                }
            }
        }

        // Y-axis /////////////////////////////////////////////////////////////
        if self.viewport.x_min <= 0.0 && self.viewport.x_max >= 0.0 {
            let (x_axis, y_start) = self.transform_point(0.0, self.viewport.y_min);
            let (_, y_end) = self.transform_point(0.0, self.viewport.y_max);

            self.canvas.begin_path();
            self.canvas.move_to(x_axis, y_start);
            self.canvas.line_to(x_axis, y_end);
            self.canvas.stroke();
        }

        // Ticks and labels
        if self.viewport.x_min <= 0.0 && self.viewport.x_max >= 0.0 {
            let (x_axis, _) = self.transform_point(0.0, 0.0);
            self.canvas.set_text_align("right");

            for i in 0..=self.y_ticks {
                let y_val = self.viewport.y_min + (self.viewport.y_max - self.viewport.y_min) * i as f32 / self.y_ticks as f32;
                let (_, y_pixel) = self.transform_point(0.0, y_val);

                // Skip the origin to avoid overlap
                if y_val.abs() < 0.001 {
                    continue;
                }

                // Draw tick mark
                self.canvas.begin_path();
                self.canvas.move_to(x_axis - tick_length / 2.0, y_pixel);
                self.canvas.line_to(x_axis + tick_length / 2.0, y_pixel);
                self.canvas.stroke();

                // Draw label
                self.canvas.fill_text(&format!("{:.2}", y_val), x_axis - 10.0, y_pixel + self.font_size / 3.0);
            }
        }
    }
}

/// Private helper functions /////////////////////////////////////////////////////////////////
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

#[derive(Debug)]
pub struct Viewport {
    /// X-axis range
    pub x_min: f32,
    pub x_max: f32,
    pub x_auto: bool,
    pub y_min: f32,
    pub y_max: f32,
    pub y_auto: bool,
    pub preserve_aspect_ratio: bool,
}