use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct IndependentStyleSettings(
    #[wasm_bindgen(skip)] pub citationberg::IndependentStyleSettings,
);
#[wasm_bindgen]
impl IndependentStyleSettings {}
