use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct Citation(#[wasm_bindgen(skip)] pub citationberg::Citation);
#[wasm_bindgen]
impl Citation {}
