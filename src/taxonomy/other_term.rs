use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct OtherTerm(#[wasm_bindgen(skip)] pub citationberg::taxonomy::OtherTerm);
#[wasm_bindgen]
impl OtherTerm {
    pub fn is_n_ordinal(&self) -> bool {
        self.0.clone().is_n_ordinal()
    }
    pub fn is_ordinal(&self) -> bool {
        self.0.clone().is_ordinal()
    }
    pub fn month(i: u8) -> Option<OtherTerm> {
        citationberg::taxonomy::OtherTerm::month(i).map(|x| x.into())
    }
    pub fn season(i: u8) -> Option<OtherTerm> {
        citationberg::taxonomy::OtherTerm::season(i).map(|x| x.into())
    }
}
