use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct CslMacro(#[wasm_bindgen(skip)] pub citationberg::CslMacro);
#[wasm_bindgen]
impl CslMacro {}
