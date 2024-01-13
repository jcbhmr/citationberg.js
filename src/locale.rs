use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct Locale(#[wasm_bindgen(skip)] pub citationberg::Locale);
#[wasm_bindgen]
impl Locale {}
