use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct ElseBranch(#[wasm_bindgen(skip)] pub citationberg::ElseBranch);
#[wasm_bindgen]
impl ElseBranch {}
