use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct SortKey(#[wasm_bindgen(skip)] pub citationberg::SortKey);
#[wasm_bindgen]
impl SortKey {
    pub fn create_variable(
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
    pub fn create_macro_name(
        SortKeyMacroName {
            name,
            names_min,
            names_use_first,
            names_use_last,
            sort_direction,
        }: SortKeyMacroName,
    ) -> Self {
        Self(citationberg::SortKey::MacroName {
            name,
            names_min,
            names_use_first,
            names_use_last,
            sort_direction: sort_direction.into(),
        })
    }

    #[wasm_bindgen(getter, js_name = "Variable")]
    pub fn variable(&self) -> Option<SortKeyVariable> {
        match &self.0 {
            citationberg::SortKey::Variable {
                variable,
                sort_direction,
            } => Some(SortKeyVariable {
                variable: variable.clone().into(),
                sort_direction: sort_direction.clone().into(),
            }),
            _ => None,
        }
    }
    #[wasm_bindgen(getter, js_name = "MacroName")]
    pub fn macro_name(&self) -> Option<SortKeyMacroName> {
        match &self.0 {
            citationberg::SortKey::MacroName {
                name,
                names_min,
                names_use_first,
                names_use_last,
                sort_direction,
            } => Some(SortKeyMacroName {
                name: name.clone(),
                names_min: names_min.clone(),
                names_use_first: names_use_first.clone(),
                names_use_last: names_use_last.clone(),
                sort_direction: sort_direction.clone().into(),
            }),
            _ => None,
        }
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
