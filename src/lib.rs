mod canvas;
mod math;
mod console;

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

        canvas::draw_circle(x, y, radius, r, g, b);
    }
}
