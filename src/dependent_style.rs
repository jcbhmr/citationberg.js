use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct DependentStyle(#[wasm_bindgen(skip)] pub citationberg::DependentStyle);
#[wasm_bindgen]
impl DependentStyle {}
