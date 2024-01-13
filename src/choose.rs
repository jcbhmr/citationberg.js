use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct Choose(#[wasm_bindgen(skip)] pub citationberg::Choose);
#[wasm_bindgen]
impl Choose {}
