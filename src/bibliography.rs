use std::num::NonZeroI16;

use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct Bibliography(#[wasm_bindgen(skip)] pub citationberg::Bibliography);

#[wasm_bindgen]
impl Bibliography {
    #[wasm_bindgen(getter)]
    pub fn sort(&self) -> Option<crate::Sort> {
        self.0.sort.clone().map(|v| v.into())
    }
    #[wasm_bindgen(setter)]
    pub fn set_sort(&mut self, v: Option<crate::Sort>) {
        self.0.sort = v.map(|v| v.into());
    }
    #[wasm_bindgen(getter)]
    pub fn layout(&self) -> crate::Layout {
        self.0.layout.clone().into()
    }
    #[wasm_bindgen(setter)]
    pub fn set_layout(&mut self, v: crate::Layout) {
        self.0.layout = v.into();
    }
    #[wasm_bindgen(getter)]
    pub fn hanging_indent(&self) -> bool {
        self.0.hanging_indent
    }
    #[wasm_bindgen(setter)]
    pub fn set_hanging_indent(&mut self, v: bool) {
        self.0.hanging_indent = v;
    }
    #[wasm_bindgen(getter)]
    pub fn second_field_align(&self) -> Option<crate::SecondFieldAlign> {
        self.0.second_field_align.clone().map(|v| v.into())
    }
    #[wasm_bindgen(setter)]
    pub fn set_second_field_align(&mut self, v: Option<crate::SecondFieldAlign>) {
        self.0.second_field_align = v.map(|v| v.into());
    }
    #[wasm_bindgen(getter)]
    pub fn line_spacing(&self) -> i16 {
        self.0.line_spacing.get()
    }
    #[wasm_bindgen(setter)]
    pub fn set_line_spacing(&mut self, v: i16) {
        self.0.line_spacing = NonZeroI16::new(v).expect_throw("line_spacing must be non-zero");
    }
    #[wasm_bindgen(getter)]
    pub fn entry_spacing(&self) -> i16 {
        self.0.entry_spacing
    }
    #[wasm_bindgen(setter)]
    pub fn set_entry_spacing(&mut self, v: i16) {
        self.0.entry_spacing = v;
    }
    #[wasm_bindgen(getter)]
    pub fn subsequent_author_substitute(&self) -> Option<String> {
        self.0.subsequent_author_substitute.clone()
    }
    #[wasm_bindgen(setter)]
    pub fn set_subsequent_author_substitute(&mut self, v: Option<String>) {
        self.0.subsequent_author_substitute = v;
    }
    #[wasm_bindgen(getter)]
    pub fn subsequent_author_substitute_rule(&self) -> crate::SubsequentAuthorSubstituteRule {
        self.0.subsequent_author_substitute_rule.clone().into()
    }
    #[wasm_bindgen(setter)]
    pub fn set_subsequent_author_substitute_rule(
        &mut self,
        v: crate::SubsequentAuthorSubstituteRule,
    ) {
        self.0.subsequent_author_substitute_rule = v.into();
    }
    #[wasm_bindgen(getter)]
    pub fn name_options(&self) -> crate::InheritableNameOptions {
        self.0.name_options.clone().into()
    }
    #[wasm_bindgen(setter)]
    pub fn set_name_options(&mut self, v: crate::InheritableNameOptions) {
        self.0.name_options = v.into();
    }
}
#[wasm_bindgen]
impl Bibliography {
  pub fn with_layout(layout: crate::Layout) -> Self {
    citationberg::Bibliography::with_layout(layout.into()).into()
  }
}
