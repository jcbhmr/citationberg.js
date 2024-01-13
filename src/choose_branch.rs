use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct ChooseBranch(#[wasm_bindgen(skip)] pub citationberg::ChooseBranch);
#[wasm_bindgen]
impl ChooseBranch {}
