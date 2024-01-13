use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct Date(#[wasm_bindgen(skip)] pub citationberg::Date);
#[wasm_bindgen]
impl Date {}
