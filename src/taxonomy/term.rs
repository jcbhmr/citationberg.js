use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct Term(#[wasm_bindgen(skip)] pub citationberg::taxonomy::Term);
#[wasm_bindgen]
impl Term {
    #[wasm_bindgen(js_name = "Kind")]
    pub fn new_kind(v: crate::taxonomy::Kind) -> Self {
        citationberg::taxonomy::Term::Kind(v.into()).into()
    }
    #[wasm_bindgen(js_name = "NameVariable")]
    pub fn new_name_variable(v: crate::taxonomy::NameVariable) -> Self {
        citationberg::taxonomy::Term::NameVariable(v.into()).into()
    }
    #[wasm_bindgen(js_name = "NumberVariable")]
    pub fn new_number_variable(v: crate::taxonomy::NumberVariable) -> Self {
        citationberg::taxonomy::Term::NumberVariable(v.into()).into()
    }
    #[wasm_bindgen(js_name = "Locator")]
    pub fn new_locator(v: crate::taxonomy::Locator) -> Self {
        citationberg::taxonomy::Term::Locator(v.into()).into()
    }
    #[wasm_bindgen(js_name = "Other")]
    pub fn new_other(v: crate::taxonomy::OtherTerm) -> Self {
        citationberg::taxonomy::Term::Other(v.into()).into()
    }

    pub fn term_fallback(&self) -> Self {
        self.0.clone().term_fallback().into()
    }
    pub fn is_ordinal(&self) -> bool {
        self.0.clone().is_ordinal()
    }
    pub fn is_n_ordinal(&self) -> bool {
        self.0.clone().is_n_ordinal()
    }
    pub fn is_gendered(&self) -> bool {
        self.0.clone().is_gendered()
    }
    pub fn is_lexically_same(&self, other: &Term) -> bool {
        self.0.clone().is_lexically_same(other.0.clone())
    }
}
