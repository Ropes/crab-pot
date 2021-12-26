mod utils;

use wasm_bindgen::prelude::*;
use std::{collections::btree_set::Union, fmt};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Call the JS alert() callback.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
extern {
    unsafe fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    alert(s);
}
