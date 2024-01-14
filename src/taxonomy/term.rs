use citationberg;
use derive_more::{From, Into};
use duplicate::duplicate_item;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct Term(#[wasm_bindgen(skip)] pub citationberg::taxonomy::Term);
#[duplicate_item(
    variant type_;
    [Kind] [crate::taxonomy::Kind];
    [NameVariable] [crate::taxonomy::NameVariable];
    [NumberVariable] [crate::taxonomy::NumberVariable];
    [Locator] [crate::taxonomy::Locator];
    [Other] [crate::taxonomy::OtherTerm];
)]
#[wasm_bindgen]
impl Term {
    #[allow(non_snake_case)]
    pub fn variant(v: type_) -> Self {
        citationberg::taxonomy::Term::variant(v.into()).into()
    }
}
#[wasm_bindgen]
impl Term {
    #[wasm_bindgen(getter)]
    pub fn tag(&self) -> String {
        match &self.0 {
            citationberg::taxonomy::Term::Kind(_) => "Kind".to_string(),
            citationberg::taxonomy::Term::NameVariable(_) => "NameVariable".to_string(),
            citationberg::taxonomy::Term::NumberVariable(_) => "NumberVariable".to_string(),
            citationberg::taxonomy::Term::Locator(_) => "Locator".to_string(),
            citationberg::taxonomy::Term::Other(_) => "Other".to_string(),
        }
    }
    #[wasm_bindgen(getter)]
    pub fn val(&self) -> JsValue {
        match &self.0 {
            citationberg::taxonomy::Term::Kind(v) => crate::taxonomy::Kind::from(v.clone()).into(),
            citationberg::taxonomy::Term::NameVariable(v) => {
                crate::taxonomy::NameVariable::from(v.clone()).into()
            }
            citationberg::taxonomy::Term::NumberVariable(v) => {
                crate::taxonomy::NumberVariable::from(v.clone()).into()
            }
            citationberg::taxonomy::Term::Locator(v) => {
                crate::taxonomy::Locator::from(v.clone()).into()
            }
            citationberg::taxonomy::Term::Other(v) => {
                crate::taxonomy::OtherTerm::from(v.clone()).into()
            }
        }
    }
}
#[wasm_bindgen]
impl Term {
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
