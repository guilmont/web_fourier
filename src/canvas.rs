#![allow(dead_code)]

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
    }
}

/// Basic calls on canvas API ////////////////////////////////////////////////////////////

pub fn arc(x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32) { unsafe { js::arc(x, y, radius, start_angle, end_angle); } }
pub fn begin_path()                                                       { unsafe { js::begin_path();                              } }
pub fn clear_rect(x: f32, y: f32, width: f32, height: f32)                { unsafe { js::clear_rect(x, y, width, height);           } }
pub fn fill()                                                             { unsafe { js::fill();                                    } }
pub fn fill_rect(x: f32, y: f32, width: f32, height: f32)                 { unsafe { js::fill_rect(x, y, width, height);            } }
pub fn height() -> f32                                                    { unsafe { js::height()                                   } }
pub fn line_to(x: f32, y: f32)                                            { unsafe { js::line_to(x, y);                             } }
pub fn move_to(x: f32, y: f32)                                            { unsafe { js::move_to(x, y);                             } }
pub fn set_fill_color(r: u8, g: u8, b: u8, a: f32)                        { unsafe { js::set_fill_style_color(r, g, b, a);          } }
pub fn set_line_width(width: f32)                                         { unsafe { js::set_line_width(width);                     } }
pub fn set_stroke_color(r: u8, g: u8, b: u8, a: f32)                      { unsafe { js::set_stroke_style_color(r, g, b, a);        } }
pub fn stroke()                                                           { unsafe { js::stroke();                                  } }
pub fn stroke_rect(x: f32, y: f32, width: f32, height: f32)               { unsafe { js::stroke_rect(x, y, width, height);          } }
pub fn width() -> f32                                                     { unsafe { js::width()                                    } }

/// More elaborated utility functions ////////////////////////////////////////////////////

pub fn clear() {
    clear_rect(0.0, 0.0, width(), height());
}

pub fn draw_rect(x: f32, y: f32, width: f32, height: f32, r: u8, g: u8, b: u8) {
    set_fill_color(r, g, b, 1.0);
    fill_rect(x, y, width, height);
}

pub fn draw_circle(x: f32, y: f32, radius: f32, r: u8, g: u8, b: u8) {
    set_fill_color(r, g, b, 1.0);
    begin_path();
    arc(x, y, radius, 0.0, 2.0 * 3.14159);
    fill();
}

pub fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32, r: u8, g: u8, b: u8, width: f32) {
    set_stroke_color(r, g, b, 1.0);
    set_line_width(width);
    begin_path();
    move_to(x1, y1);
    line_to(x2, y2);
    stroke();
}
