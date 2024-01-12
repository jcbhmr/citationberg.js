
use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct Variable(#[wasm_bindgen(skip)] pub citationberg::taxonomy::Variable);
#[wasm_bindgen]
impl Variable {}
