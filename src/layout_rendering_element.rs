use crate::*;
use citationberg_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub struct LayoutRenderingElement(pub(crate) citationberg_rs::LayoutRenderingElement);

// Text(Text),
// Date(Date),
// Number(Number),
// Names(Names),
// Label(Label),
// Group(Group),
// Choose(Choose),

#[wasm_bindgen]
impl LayoutRenderingElement {
    pub fn Text(value: Text) -> Self {
        return Self {
            0: citationberg_rs::LayoutRenderingElement::Text(value.0),
        };
    }
    pub fn Date(value: Date) -> Self {
        return Self {
            0: citationberg_rs::LayoutRenderingElement::Date(value.0),
        };
    }
    pub fn Number(value: Number) -> Self {
        return Self {
            0: citationberg_rs::LayoutRenderingElement::Number(value.0),
        };
    }
    pub fn Names(value: Names) -> Self {
        return Self {
            0: citationberg_rs::LayoutRenderingElement::Names(value.0),
        };
    }
    pub fn Label(value: Label) -> Self {
        return Self {
            0: citationberg_rs::LayoutRenderingElement::Label(value.0),
        };
    }
    pub fn Group(value: Group) -> Self {
        return Self {
            0: citationberg_rs::LayoutRenderingElement::Group(value.0),
        };
    }
    pub fn Choose(value: Choose) -> Self {
        return Self {
            0: citationberg_rs::LayoutRenderingElement::Choose(value.0),
        };
    }

    #[wasm_bindgen(getter)]
    pub fn type_name(&self) -> String {
        return match self.0 {
            citationberg_rs::LayoutRenderingElement::Text(_) => "Text".to_string(),
            citationberg_rs::LayoutRenderingElement::Date(_) => "Date".to_string(),
            citationberg_rs::LayoutRenderingElement::Number(_) => "Number".to_string(),
            citationberg_rs::LayoutRenderingElement::Names(_) => "Names".to_string(),
            citationberg_rs::LayoutRenderingElement::Label(_) => "Label".to_string(),
            citationberg_rs::LayoutRenderingElement::Group(_) => "Group".to_string(),
            citationberg_rs::LayoutRenderingElement::Choose(_) => "Choose".to_string(),
        };
    }
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> JsValue {
        return match self.0 {
            citationberg_rs::LayoutRenderingElement::Text(value) => JsValue::from(value.0),
            citationberg_rs::LayoutRenderingElement::Date(value) => JsValue::from(value.0),
            citationberg_rs::LayoutRenderingElement::Number(value) => JsValue::from(value.0),
            citationberg_rs::LayoutRenderingElement::Names(value) => JsValue::from(value.0),
            citationberg_rs::LayoutRenderingElement::Label(value) => JsValue::from(value.0),
            citationberg_rs::LayoutRenderingElement::Group(value) => JsValue::from(value.0),
            citationberg_rs::LayoutRenderingElement::Choose(value) => JsValue::from(value.0),
        };
    }
}
