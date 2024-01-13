use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct Group(#[wasm_bindgen(skip)] pub citationberg::Group);
#[wasm_bindgen]
impl Group {}
