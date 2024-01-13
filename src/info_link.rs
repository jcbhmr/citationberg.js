use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct InfoLink(#[wasm_bindgen(skip)] pub citationberg::InfoLink);
#[wasm_bindgen]
impl InfoLink {}
