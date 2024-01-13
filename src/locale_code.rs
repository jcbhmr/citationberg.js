use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct LocaleCode(#[wasm_bindgen(skip)] pub citationberg::LocaleCode);
#[wasm_bindgen]
impl LocaleCode {}
