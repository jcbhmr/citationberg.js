use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct IndependentStyle(#[wasm_bindgen(skip)] pub citationberg::IndependentStyle);
#[wasm_bindgen]
impl IndependentStyle {}
