use citationberg;
use derive_more::{From, Into};
use duplicate::duplicate_item;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct Variable(#[wasm_bindgen(skip)] pub citationberg::taxonomy::Variable);
#[duplicate_item(
    variant type_;
    [Standard] [crate::taxonomy::StandardVariable];
    [Number] [crate::taxonomy::NumberVariable];
    [Date] [crate::taxonomy::DateVariable];
    [Name] [crate::taxonomy::NameVariable];
)]
#[wasm_bindgen]
impl Variable {
    #[allow(non_snake_case)]
    pub fn variant(v: type_) -> Self {
        citationberg::taxonomy::Variable::variant(v.into()).into()
    }
}
#[wasm_bindgen]
impl Variable {
    #[wasm_bindgen(getter)]
    pub fn tag(&self) -> String {
        match &self.0 {
            citationberg::taxonomy::Variable::Standard(_) => "Standard".to_string(),
            citationberg::taxonomy::Variable::Number(_) => "Number".to_string(),
            citationberg::taxonomy::Variable::Date(_) => "Date".to_string(),
            citationberg::taxonomy::Variable::Name(_) => "Name".to_string(),
        }
    }
    #[wasm_bindgen(getter)]
    pub fn val(&self) -> JsValue {
        match &self.0 {
            citationberg::taxonomy::Variable::Standard(v) => {
                crate::taxonomy::StandardVariable::from(v.clone()).into()
            }
            citationberg::taxonomy::Variable::Number(v) => {
                crate::taxonomy::NumberVariable::from(v.clone()).into()
            }
            citationberg::taxonomy::Variable::Date(v) => {
                crate::taxonomy::DateVariable::from(v.clone()).into()
            }
            citationberg::taxonomy::Variable::Name(v) => {
                crate::taxonomy::NameVariable::from(v.clone()).into()
            }
        }
    }
}
#[wasm_bindgen]
impl Variable {
    pub fn is_number_of_variable(&self) -> bool {
        self.0.clone().is_number_of_variable()
    }
}
