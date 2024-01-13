use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct License(#[wasm_bindgen(skip)] pub citationberg::License);
#[wasm_bindgen]
impl License {}
