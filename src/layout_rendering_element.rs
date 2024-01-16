use citationberg;
use derive_more::{From, Into};
use duplicate::duplicate_item;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct LayoutRenderingElement(#[wasm_bindgen(skip)] pub citationberg::LayoutRenderingElement);
#[duplicate_item(
    variant type_;
    [Text] [crate::Text];
    [Date] [crate::Date];
    [Number] [crate::Number];
    [Names] [crate::Names];
    [Label] [crate::Label];
    [Group] [crate::Group];
    [Choose] [crate::Choose];
)]
#[wasm_bindgen]
impl LayoutRenderingElement {
    #[allow(non_snake_case)]
    pub fn variant(v: type_) -> Self {
        citationberg::LayoutRenderingElement::variant(v.into()).into()
    }
}
#[wasm_bindgen]
impl LayoutRenderingElement {
    #[wasm_bindgen(getter)]
    pub fn tag(&self) -> String {
        match &self.0 {
            citationberg::LayoutRenderingElement::Text(_) => "Text".to_string(),
            citationberg::LayoutRenderingElement::Date(_) => "Date".to_string(),
            citationberg::LayoutRenderingElement::Number(_) => "Number".to_string(),
            citationberg::LayoutRenderingElement::Names(_) => "Names".to_string(),
            citationberg::LayoutRenderingElement::Label(_) => "Label".to_string(),
            citationberg::LayoutRenderingElement::Group(_) => "Group".to_string(),
            citationberg::LayoutRenderingElement::Choose(_) => "Choose".to_string(),
        }
    }
    #[wasm_bindgen(getter)]
    pub fn val(&self) -> JsValue {
        match &self.0 {
            citationberg::LayoutRenderingElement::Text(v) => crate::Text::from(v.clone()).into(),
            citationberg::LayoutRenderingElement::Date(v) => crate::Date::from(v.clone()).into(),
            citationberg::LayoutRenderingElement::Number(v) => {
                crate::Number::from(v.clone()).into()
            }
            citationberg::LayoutRenderingElement::Names(v) => crate::Names::from(v.clone()).into(),
            citationberg::LayoutRenderingElement::Label(v) => crate::Label::from(v.clone()).into(),
            citationberg::LayoutRenderingElement::Group(v) => crate::Group::from(v.clone()).into(),
            citationberg::LayoutRenderingElement::Choose(v) => {
                crate::Choose::from(v.clone()).into()
            }
        }
    }
}
#[wasm_bindgen]
impl LayoutRenderingElement {
    pub fn find_variable_element(
        &self,
        variable: crate::taxonomy::Variable,
        macros: Vec<crate::CslMacro>,
    ) -> Option<LayoutRenderingElement> {
        self.0
            .find_variable_element(
                variable.into(),
                macros
                    .into_iter()
                    .map(|v| v.into())
                    .collect::<Vec<_>>()
                    .as_slice(),
            )
            .map(|v| v.into())
    }
}
