#![allow(dead_code)]

/// Matplotlib-inspired color palette ////////////////////////////////////////////////////

// Basic colors
pub const BLACK: (u8, u8, u8) = (0, 0, 0);
pub const DARK_GRAY: (u8, u8, u8) = (64, 64, 64);
pub const LIGHT_GRAY: (u8, u8, u8) = (200, 200, 200);
pub const WHITE: (u8, u8, u8) = (255, 255, 255);
pub const RED: (u8, u8, u8) = (255, 0, 0);
pub const GREEN: (u8, u8, u8) = (0, 255, 0);
pub const BLUE: (u8, u8, u8) = (0, 0, 255);
pub const MAGENTA: (u8, u8, u8) = (255, 0, 255);
pub const YELLOW: (u8, u8, u8) = (255, 255, 0);
pub const CYAN: (u8, u8, u8) = (0, 255, 255);

// Matplotlib default color cycle (C0-C9)
pub const TAB_BLUE: (u8, u8, u8) = (31, 119, 180);    // #1f77b4
pub const TAB_ORANGE: (u8, u8, u8) = (255, 127, 14);  // #ff7f0e
pub const TAB_GREEN: (u8, u8, u8) = (44, 160, 44);    // #2ca02c
pub const TAB_RED: (u8, u8, u8) = (214, 39, 40);      // #d62728
pub const TAB_PURPLE: (u8, u8, u8) = (148, 103, 189); // #9467bd
pub const TAB_BROWN: (u8, u8, u8) = (140, 86, 75);    // #8c564b
pub const TAB_PINK: (u8, u8, u8) = (227, 119, 194);   // #e377c2
pub const TAB_GRAY: (u8, u8, u8) = (127, 127, 127);   // #7f7f7f
pub const TAB_OLIVE: (u8, u8, u8) = (188, 189, 34);   // #bcbd22
pub const TAB_CYAN: (u8, u8, u8) = (23, 190, 207);    // #17becf


/// API imported from JavaScript at the browser //////////////////////////////////////////
mod js {
    #[link(wasm_import_module = "Canvas")]
    extern "C" {
        pub fn arc(canvas_id: u32, x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32);
        pub fn begin_path(canvas_id: u32);
        pub fn clear_rect(canvas_id: u32, x: f32, y: f32, width: f32, height: f32);
        pub fn fill(canvas_id: u32);
        pub fn fill_rect(canvas_id: u32, x: f32, y: f32, width: f32, height: f32);
        pub fn height(canvas_id: u32) -> f32;
        pub fn line_to(canvas_id: u32, x: f32, y: f32);
        pub fn move_to(canvas_id: u32, x: f32, y: f32);
        pub fn set_fill_style_color(canvas_id: u32, r: u8, g: u8, b: u8, a: f32);
        pub fn set_line_width(canvas_id: u32, width: f32);
        pub fn set_stroke_style_color(canvas_id: u32, r: u8, g: u8, b: u8, a: f32);
        pub fn stroke(canvas_id: u32);
        pub fn stroke_rect(canvas_id: u32, x: f32, y: f32, width: f32, height: f32);
        pub fn width(canvas_id: u32) -> f32;
        pub fn fill_text(canvas_id: u32, text_ptr: *const u8, text_len: usize, x: f32, y: f32);
        pub fn set_font(canvas_id: u32, font_ptr: *const u8, font_len: usize);
        pub fn set_text_align(canvas_id: u32, align_ptr: *const u8, align_len: usize);
    }
}

/// Basic calls on canvas API ////////////////////////////////////////////////////////////
/// These are low-level functions used internally by the Canvas struct.
/// Prefer using the Canvas struct for higher-level operations.

fn arc(canvas_id: u32, x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32) { unsafe { js::arc(canvas_id, x, y, radius, start_angle, end_angle);   } }
fn begin_path(canvas_id: u32)                                                         { unsafe { js::begin_path(canvas_id);                                } }
fn clear_rect(canvas_id: u32, x: f32, y: f32, width: f32, height: f32)                { unsafe { js::clear_rect(canvas_id, x, y, width, height);             } }
fn fill(canvas_id: u32)                                                               { unsafe { js::fill(canvas_id);                                      } }
fn fill_rect(canvas_id: u32, x: f32, y: f32, width: f32, height: f32)                 { unsafe { js::fill_rect(canvas_id, x, y, width, height);              } }
fn height(canvas_id: u32) -> f32                                                      { unsafe { js::height(canvas_id)                                     } }
fn line_to(canvas_id: u32, x: f32, y: f32)                                            { unsafe { js::line_to(canvas_id, x, y);                               } }
fn move_to(canvas_id: u32, x: f32, y: f32)                                            { unsafe { js::move_to(canvas_id, x, y);                               } }
fn set_fill_color(canvas_id: u32, r: u8, g: u8, b: u8, a: f32)                        { unsafe { js::set_fill_style_color(canvas_id, r, g, b, a);            } }
fn set_line_width(canvas_id: u32, width: f32)                                         { unsafe { js::set_line_width(canvas_id, width);                       } }
fn set_stroke_color(canvas_id: u32, r: u8, g: u8, b: u8, a: f32)                      { unsafe { js::set_stroke_style_color(canvas_id, r, g, b, a);          } }
fn stroke(canvas_id: u32)                                                             { unsafe { js::stroke(canvas_id);                                    } }
fn stroke_rect(canvas_id: u32, x: f32, y: f32, width: f32, height: f32)               { unsafe { js::stroke_rect(canvas_id, x, y, width, height);            } }
fn width(canvas_id: u32) -> f32                                                       { unsafe { js::width(canvas_id)                                      } }
fn fill_text(canvas_id: u32, text: &str, x: f32, y: f32)                              { unsafe { js::fill_text(canvas_id, text.as_ptr(), text.len(), x, y);  } }
fn set_font(canvas_id: u32, font: &str)                                               { unsafe { js::set_font(canvas_id, font.as_ptr(), font.len());         } }
fn set_text_align(canvas_id: u32, align: &str)                                        { unsafe { js::set_text_align(canvas_id, align.as_ptr(), align.len()); } }

/// Canvas object that encapsulates canvas operations ///////////////////////////////////

pub struct Canvas {
    id: u32,
}

impl Canvas {
    /// Create a new Canvas instance for the given canvas ID
    pub fn new(canvas_id: u32) -> Self {
        Self {
            id: canvas_id,
        }
    }

    /// Get the canvas ID
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get canvas width
    pub fn width(&self) -> f32 {
        width(self.id)
    }

    /// Get canvas height
    pub fn height(&self) -> f32 {
        height(self.id)
    }

    /// Basic drawing operations
    pub fn arc(&self, x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32) {
        arc(self.id, x, y, radius, start_angle, end_angle);
    }

    pub fn begin_path(&self) {
        begin_path(self.id);
    }

    pub fn clear_rect(&self, x: f32, y: f32, width: f32, height: f32) {
        clear_rect(self.id, x, y, width, height);
    }

    pub fn fill(&self) {
        fill(self.id);
    }

    pub fn fill_rect(&self, x: f32, y: f32, width: f32, height: f32) {
        fill_rect(self.id, x, y, width, height);
    }

    pub fn line_to(&self, x: f32, y: f32) {
        line_to(self.id, x, y);
    }

    pub fn move_to(&self, x: f32, y: f32) {
        move_to(self.id, x, y);
    }

    pub fn set_fill_color(&self, r: u8, g: u8, b: u8, a: f32) {
        set_fill_color(self.id, r, g, b, a);
    }

    pub fn set_line_width(&self, width: f32) {
        set_line_width(self.id, width);
    }

    pub fn set_stroke_color(&self, r: u8, g: u8, b: u8, a: f32) {
        set_stroke_color(self.id, r, g, b, a);
    }

    pub fn stroke(&self) {
        stroke(self.id);
    }

    pub fn stroke_rect(&self, x: f32, y: f32, width: f32, height: f32) {
        stroke_rect(self.id, x, y, width, height);
    }

    pub fn fill_text(&self, text: &str, x: f32, y: f32) {
        fill_text(self.id, text, x, y);
    }

    pub fn set_font(&self, font: &str) {
        set_font(self.id, font);
    }

    pub fn set_text_align(&self, align: &str) {
        set_text_align(self.id, align);
    }

    /// Higher-level drawing operations
    
    /// Clears the entire canvas
    pub fn clear(&self) {
        self.clear_rect(0.0, 0.0, self.width(), self.height());
    }

    /// Draws a rectangle at (x, y) with a given width, height and color.
    pub fn draw_rect(&self, x: f32, y: f32, rect_width: f32, rect_height: f32, color: (u8, u8, u8)) {
        self.set_fill_color(color.0, color.1, color.2, 1.0);
        self.fill_rect(x, y, rect_width, rect_height);
    }

    /// Draws a circle at (x, y) with a given radius and color.
    pub fn draw_circle(&self, x: f32, y: f32, radius: f32, color: (u8, u8, u8)) {
        self.set_fill_color(color.0, color.1, color.2, 1.0);
        self.begin_path();
        self.arc(x, y, radius, 0.0, 2.0 * 3.14159);
        self.fill();
    }

    /// Draws a line from (x1, y1) to (x2, y2) with a given color and width.
    pub fn draw_line(&self, x1: f32, y1: f32, x2: f32, y2: f32, color: (u8, u8, u8), line_width: f32) {
        self.set_stroke_color(color.0, color.1, color.2, 1.0);
        self.set_line_width(line_width);
        self.begin_path();
        self.move_to(x1, y1);
        self.line_to(x2, y2);
        self.stroke();
    }

    /// Draws a triangle centered at (x, y) with a given size and orientation angle (in radians).
    pub fn draw_triangle(&self, x: f32, y: f32, size: f32, angle: f32, color: (u8, u8, u8)) {
        let h = size; // height from center to tip
        let w = size * 0.6; // width of the base
        // Calculate the three vertices
        let tip_x = x + h * angle.cos();
        let tip_y = y + h * angle.sin();
        let base_angle1 = angle + std::f32::consts::PI * 2.0 / 3.0;
        let base_angle2 = angle - std::f32::consts::PI * 2.0 / 3.0;
        let base1_x = x + w * base_angle1.cos();
        let base1_y = y + w * base_angle1.sin();
        let base2_x = x + w * base_angle2.cos();
        let base2_y = y + w * base_angle2.sin();
        self.set_fill_color(color.0, color.1, color.2, 1.0);
        self.begin_path();
        self.move_to(tip_x, tip_y);
        self.line_to(base1_x, base1_y);
        self.line_to(base2_x, base2_y);
        self.line_to(tip_x, tip_y);
        self.fill();
    }

    /// Draws an arrow from (x1, y1) to (x2, y2) with a given color and width.
    pub fn draw_arrow(&self, x1: f32, y1: f32, x2: f32, y2: f32, color: (u8, u8, u8), line_width: f32) {
        // Skip drawing if the length is too small to be visible
        let length = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
        if length < line_width { return; }

        // Draw the main line
        self.draw_line(x1, y1, x2, y2, color, line_width);

        let height = 6.0 * line_width;
        let angle = (y2 - y1).atan2(x2 - x1);

        // Draw arrowhead
        self.draw_triangle(x2 - height * angle.cos(), y2 - height * angle.sin(), height, angle, color);
    }
}