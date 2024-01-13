use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct Label(#[wasm_bindgen(skip)] pub citationberg::Label);
#[wasm_bindgen]
impl Label {}
