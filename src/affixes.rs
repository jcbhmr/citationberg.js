use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct Affixes(#[wasm_bindgen(skip)] pub citationberg::Affixes);
#[wasm_bindgen]
impl Affixes {
    #[wasm_bindgen(getter)]
    pub fn prefix(&self) -> Option<String> {
        self.0.prefix.clone()
    }
    #[wasm_bindgen(setter)]
    pub fn set_prefix(&mut self, v: Option<String>) {
        self.0.prefix = v;
    }
    #[wasm_bindgen(getter)]
    pub fn suffix(&self) -> Option<String> {
        self.0.suffix.clone()
    }
    #[wasm_bindgen(setter)]
    pub fn set_suffix(&mut self, v: Option<String>) {
        self.0.suffix = v;
    }
}
