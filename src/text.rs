use citationberg;
use derive_more::{From, Into};
use duplicate::duplicate_item;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct Text(#[wasm_bindgen(skip)] pub citationberg::Text);
