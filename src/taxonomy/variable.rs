use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct Variable(#[wasm_bindgen(skip)] pub citationberg::taxonomy::Variable);
#[wasm_bindgen]
impl Variable {
    #[wasm_bindgen(js_name = "Standard")]
    pub fn new_standard(v: crate::taxonomy::StandardVariable) -> Self {
        citationberg::taxonomy::Variable::Standard(v.into()).into()
    }
    #[wasm_bindgen(js_name = "Number")]
    pub fn new_number(v: crate::taxonomy::NumberVariable) -> Self {
        citationberg::taxonomy::Variable::Number(v.into()).into()
    }
    #[wasm_bindgen(js_name = "Date")]
    pub fn new_date(v: crate::taxonomy::DateVariable) -> Self {
        citationberg::taxonomy::Variable::Date(v.into()).into()
    }
    #[wasm_bindgen(js_name = "Name")]
    pub fn new_name(v: crate::taxonomy::NameVariable) -> Self {
        citationberg::taxonomy::Variable::Name(v.into()).into()
    }

    pub fn is_number_of_variable(&self) -> bool {
        self.0.clone().is_number_of_variable()
    }
}
