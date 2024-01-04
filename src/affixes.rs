use crate::*;
use citationberg_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub struct Affixes(pub(crate) citationberg_rs::Affixes);

#[wasm_bindgen]
impl Affixes {
    #[wasm_bindgen(constructor)]
    pub fn new(prefix: Option<String>, suffix: Option<String>) -> Self {
        return Self {
            0: citationberg_rs::Affixes { prefix, suffix },
        };
    }

    #[wasm_bindgen(getter)]
    pub fn prefix(&self) -> Option<String> {
        return self.0.prefix.clone();
    }
    #[wasm_bindgen(getter)]
    pub fn suffix(&self) -> Option<String> {
        return self.0.suffix.clone();
    }
}
