use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct EtAl(#[wasm_bindgen(skip)] pub citationberg::EtAl);
#[wasm_bindgen]
impl EtAl {}
