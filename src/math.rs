#![allow(dead_code)]
mod js {
    #[link(wasm_import_module = "Math")]
    extern "C" {
        pub fn random() -> f32;
    }
}

pub fn random() -> f32 { unsafe { js::random() } }
