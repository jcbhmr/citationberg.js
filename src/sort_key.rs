use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct SortKey(#[wasm_bindgen(skip)] pub citationberg::SortKey);
#[wasm_bindgen]
impl SortKey {
    #[wasm_bindgen(js_name = "Variable")]
    pub fn new_variable(
        SortKeyVariable {
            variable,
            sort_direction,
        }: SortKeyVariable,
    ) -> Self {
        Self(citationberg::SortKey::Variable {
            variable: variable.into(),
            sort_direction: sort_direction.into(),
        })
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct SortKeyVariable {
    pub variable: crate::taxonomy::Variable,
    pub sort_direction: crate::SortDirection,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct SortKeyMacroName {
    pub name: String,
    pub names_min: Option<u32>,
    pub names_use_first: Option<u32>,
    pub names_use_last: Option<bool>,
    pub sort_direction: crate::SortDirection,
}
