use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct Layout(#[wasm_bindgen(skip)] pub citationberg::Layout);
#[wasm_bindgen]
impl Layout {}
