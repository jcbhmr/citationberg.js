use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct LocaleInfo(#[wasm_bindgen(skip)] pub citationberg::LocaleInfo);
#[wasm_bindgen]
impl LocaleInfo {}
