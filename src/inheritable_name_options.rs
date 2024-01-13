use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct InheritableNameOptions(#[wasm_bindgen(skip)] pub citationberg::InheritableNameOptions);
#[wasm_bindgen]
impl InheritableNameOptions {}
