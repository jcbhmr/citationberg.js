use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct LocalString(#[wasm_bindgen(skip)] pub citationberg::LocalString);
#[wasm_bindgen]
impl LocalString {}
