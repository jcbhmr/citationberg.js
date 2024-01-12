use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct Sort(#[wasm_bindgen(skip)] pub citationberg::Sort);
#[wasm_bindgen]
impl Sort {
    #[wasm_bindgen(getter)]
    pub fn keys(&self) -> Vec<crate::SortKey> {
        self.0.keys.iter().map(|key| key.clone().into()).collect()
    }
}
