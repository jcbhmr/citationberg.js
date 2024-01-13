use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct DatePart(#[wasm_bindgen(skip)] pub citationberg::DatePart);
#[wasm_bindgen]
impl DatePart {}
