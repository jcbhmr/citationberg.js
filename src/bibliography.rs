use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct Bibliography(#[wasm_bindgen(skip)] pub citationberg::Bibliography);
#[wasm_bindgen]
impl Bibliography {}
