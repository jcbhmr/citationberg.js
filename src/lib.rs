use citationberg;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Affixes(#[wasm_bindgen(skip)] pub citationberg::Affixes);
