use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct Formatting(#[wasm_bindgen(skip)] pub citationberg::Formatting);
#[wasm_bindgen]
impl Formatting {}
