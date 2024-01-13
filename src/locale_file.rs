use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct LocaleFile(#[wasm_bindgen(skip)] pub citationberg::LocaleFile);
#[wasm_bindgen]
impl LocaleFile {}
