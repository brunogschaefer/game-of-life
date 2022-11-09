use wasm_bindgen::prelude::wasm_bindgen;


#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cells {
    DEAD = 0,
    ALIVE = 1,
}