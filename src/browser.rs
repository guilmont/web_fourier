mod js {
    #[link(wasm_import_module = "Browser")]
    extern "C" {
        pub fn alert(ptr: *const u8, len: usize);
    }
}

pub fn alert(msg: &str) {
    unsafe { js::alert(msg.as_ptr(), msg.len()); }
}