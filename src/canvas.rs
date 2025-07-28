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
        pub fn arc(x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32);
        pub fn begin_path();
        pub fn clear_rect(x: f32, y: f32, width: f32, height: f32);
        pub fn fill();
        pub fn fill_rect(x: f32, y: f32, width: f32, height: f32);
        pub fn height() -> f32;
        pub fn line_to(x: f32, y: f32);
        pub fn move_to(x: f32, y: f32);
        pub fn set_fill_style_color(r: u8, g: u8, b: u8, a: f32);
        pub fn set_line_width(width: f32);
        pub fn set_stroke_style_color(r: u8, g: u8, b: u8, a: f32);
        pub fn stroke();
        pub fn stroke_rect(x: f32, y: f32, width: f32, height: f32);
        pub fn width()  -> f32;
        pub fn fill_text(text_ptr: *const u8, text_len: usize, x: f32, y: f32);
        pub fn set_font(font_ptr: *const u8, font_len: usize);
        pub fn set_text_align(align_ptr: *const u8, align_len: usize);
    }
}

/// Basic calls on canvas API ////////////////////////////////////////////////////////////

pub fn arc(x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32) { unsafe { js::arc(x, y, radius, start_angle, end_angle);   } }
pub fn begin_path()                                                       { unsafe { js::begin_path();                                } }
pub fn clear_rect(x: f32, y: f32, width: f32, height: f32)                { unsafe { js::clear_rect(x, y, width, height);             } }
pub fn fill()                                                             { unsafe { js::fill();                                      } }
pub fn fill_rect(x: f32, y: f32, width: f32, height: f32)                 { unsafe { js::fill_rect(x, y, width, height);              } }
pub fn height() -> f32                                                    { unsafe { js::height()                                     } }
pub fn line_to(x: f32, y: f32)                                            { unsafe { js::line_to(x, y);                               } }
pub fn move_to(x: f32, y: f32)                                            { unsafe { js::move_to(x, y);                               } }
pub fn set_fill_color(r: u8, g: u8, b: u8, a: f32)                        { unsafe { js::set_fill_style_color(r, g, b, a);            } }
pub fn set_line_width(width: f32)                                         { unsafe { js::set_line_width(width);                       } }
pub fn set_stroke_color(r: u8, g: u8, b: u8, a: f32)                      { unsafe { js::set_stroke_style_color(r, g, b, a);          } }
pub fn stroke()                                                           { unsafe { js::stroke();                                    } }
pub fn stroke_rect(x: f32, y: f32, width: f32, height: f32)               { unsafe { js::stroke_rect(x, y, width, height);            } }
pub fn width() -> f32                                                     { unsafe { js::width()                                      } }
pub fn fill_text(text: &str, x: f32, y: f32)                              { unsafe { js::fill_text(text.as_ptr(), text.len(), x, y);  } }
pub fn set_font(font: &str)                                               { unsafe { js::set_font(font.as_ptr(), font.len());         } }
pub fn set_text_align(align: &str)                                        { unsafe { js::set_text_align(align.as_ptr(), align.len()); } }

/// More elaborated utility functions ////////////////////////////////////////////////////

/// Clears the entire canvas
pub fn clear() {
    clear_rect(0.0, 0.0, width(), height());
}

/// Draws a rectangle at (x, y) with a given width, height and color.
pub fn draw_rect(x: f32, y: f32, width: f32, height: f32, color: (u8, u8, u8)) {
    set_fill_color(color.0, color.1, color.2, 1.0);
    fill_rect(x, y, width, height);
}

/// Draws a circle at (x, y) with a given radius and color.
pub fn draw_circle(x: f32, y: f32, radius: f32, color: (u8, u8, u8)) {
    set_fill_color(color.0, color.1, color.2, 1.0);
    begin_path();
    arc(x, y, radius, 0.0, 2.0 * 3.14159);
    fill();
}

/// Draws a line from (x1, y1) to (x2, y2) with a given color and width.
pub fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32, color: (u8, u8, u8), width: f32) {
    set_stroke_color(color.0, color.1, color.2, 1.0);
    set_line_width(width);
    begin_path();
    move_to(x1, y1);
    line_to(x2, y2);
    stroke();
}

/// Draws a triangle centered at (x, y) with a given size and orientation angle (in radians).
pub fn draw_triangle(x: f32, y: f32, size: f32, angle: f32, color: (u8, u8, u8)) {
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
    set_fill_color(color.0, color.1, color.2, 1.0);
    begin_path();
    move_to(tip_x, tip_y);
    line_to(base1_x, base1_y);
    line_to(base2_x, base2_y);
    line_to(tip_x, tip_y);
    fill();
}

/// Draws an arrow from (x1, y1) to (x2, y2) with a given color and width.
pub fn draw_arrow(x1: f32, y1: f32, x2: f32, y2: f32, color: (u8, u8, u8), width: f32) {
    // Skip drawing if the length is too small to be visible
    let length = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
    if length < width { return; }

    // Draw the main line
    draw_line(x1, y1, x2, y2, color, width);

    let height = 6.0 * width;
    let angle = (y2 - y1).atan2(x2 - x1);

    // Draw arrowhead
    draw_triangle(x2 - height * angle.cos(), y2 - height * angle.sin(), height, angle, color);
}