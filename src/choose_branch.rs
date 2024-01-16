use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct ChooseBranch(#[wasm_bindgen(skip)] pub citationberg::ChooseBranch);
#[wasm_bindgen]
impl ChooseBranch {
    #[wasm_bindgen(getter)]
    pub fn disambiguate(&self) -> Option<bool> {
        self.0.disambiguate
    }
    #[wasm_bindgen(setter)]
    pub fn set_disambiguate(&mut self, v: Option<bool>) {
        self.0.disambiguate = v;
    }
    #[wasm_bindgen(getter)]
    pub fn is_numeric(&self) -> Option<Vec<crate::taxonomy::Variable>> {
        self.0
            .is_numeric
            .clone()
            .map(|v| v.into_iter().map(|v| v.into()).collect())
    }
    #[wasm_bindgen(setter)]
    pub fn set_is_numeric(&mut self, v: Option<Vec<crate::taxonomy::Variable>>) {
        self.0.is_numeric = v.map(|v| v.into_iter().map(|v| v.into()).collect());
    }
    #[wasm_bindgen(getter)]
    pub fn is_uncertain_date(&self) -> Option<Vec<crate::taxonomy::DateVariable>> {
        self.0
            .is_uncertain_date
            .clone()
            .map(|v| v.into_iter().map(|v| v.into()).collect())
    }
    #[wasm_bindgen(setter)]
    pub fn set_is_uncertain_date(&mut self, v: Option<Vec<crate::taxonomy::DateVariable>>) {
        self.0.is_uncertain_date = v.map(|v| v.into_iter().map(|v| v.into()).collect());
    }
    #[wasm_bindgen(getter)]
    pub fn locator(&self) -> Option<Vec<crate::taxonomy::Locator>> {
        self.0
            .locator
            .clone()
            .map(|v| v.into_iter().map(|v| v.into()).collect())
    }
    #[wasm_bindgen(setter)]
    pub fn set_locator(&mut self, v: Option<Vec<crate::taxonomy::Locator>>) {
        self.0.locator = v.map(|v| v.into_iter().map(|v| v.into()).collect());
    }
    #[wasm_bindgen(getter)]
    pub fn position(&self) -> Option<Vec<crate::TestPosition>> {
        self.0
            .position
            .clone()
            .map(|v| v.into_iter().map(|v| v.into()).collect())
    }
    #[wasm_bindgen(setter)]
    pub fn set_position(&mut self, v: Option<Vec<crate::TestPosition>>) {
        self.0.position = v.map(|v| v.into_iter().map(|v| v.into()).collect());
    }
    #[wasm_bindgen(getter)]
    pub fn type_(&self) -> Option<Vec<crate::taxonomy::Kind>> {
        self.0
            .type_
            .clone()
            .map(|v| v.into_iter().map(|v| v.into()).collect())
    }
    #[wasm_bindgen(setter)]
    pub fn set_type_(&mut self, v: Option<Vec<crate::taxonomy::Kind>>) {
        self.0.type_ = v.map(|v| v.into_iter().map(|v| v.into()).collect());
    }
    #[wasm_bindgen(getter)]
    pub fn variable(&self) -> Option<Vec<crate::taxonomy::Variable>> {
        self.0
            .variable
            .clone()
            .map(|v| v.into_iter().map(|v| v.into()).collect())
    }
    #[wasm_bindgen(setter)]
    pub fn set_variable(&mut self, v: Option<Vec<crate::taxonomy::Variable>>) {
        self.0.variable = v.map(|v| v.into_iter().map(|v| v.into()).collect());
    }
    #[wasm_bindgen(getter, js_name = "match")]
    pub fn match_(&self) -> crate::ChooseMatch {
        self.0.match_.clone().into()
    }
    #[wasm_bindgen(setter)]
    pub fn set_match(&mut self, v: crate::ChooseMatch) {
        self.0.match_ = v.into();
    }
    #[wasm_bindgen(getter)]
    pub fn children(&self) -> Vec<crate::LayoutRenderingElement> {
        self.0
            .children
            .clone()
            .into_iter()
            .map(|v| v.into())
            .collect()
    }
    #[wasm_bindgen(setter)]
    pub fn set_children(&mut self, v: Vec<crate::LayoutRenderingElement>) {
        self.0.children = v.into_iter().map(|v| v.into()).collect();
    }
}
