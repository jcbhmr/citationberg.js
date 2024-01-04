use crate::*;
use citationberg_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub struct Bibliography(pub(crate) citationberg_rs::Bibliography);

#[wasm_bindgen]
impl Bibliography {
    #[wasm_bindgen(constructor)]
    pub fn new(
        sort: Option<Sort>,
        layout: Layout,
        hanging_indent: bool,
        second_field_align: Option<SecondFieldAlign>,
        line_spacing: i16,
        entry_spacing: i16,
        subsequent_author_substitute: Option<String>,
        subsequent_author_substitute_rule: SubsequentAuthorSubstituteRule,
        name_options: InheritableNameOptions,
    ) -> Self {
        return Self {
            0: citationberg_rs::Bibliography {
                sort: sort.map(|x| x.0),
                layout,
                hanging_indent,
                second_field_align: second_field_align.map(|x| x.0),
                line_spacing,
                entry_spacing,
                subsequent_author_substitute,
                subsequent_author_substitute_rule: subsequent_author_substitute_rule.0,
                name_options: name_options.0,
            },
        };
    }

    #[wasm_bindgen(getter)]
    pub fn sort(&self) -> Option<Sort> {
        return self.0.sort;
    }
    #[wasm_bindgen(getter)]
    pub fn layout(&self) -> Layout {
        return self.0.layout;
    }
    #[wasm_bindgen(getter)]
    pub fn hanging_indent(&self) -> bool {
        return self.0.hanging_indent;
    }
    #[wasm_bindgen(getter)]
    pub fn second_field_align(&self) -> Option<SecondFieldAlign> {
        return self.0.second_field_align;
    }
    #[wasm_bindgen(getter)]
    pub fn line_spacing(&self) -> i16 {
        return self.0.line_spacing;
    }
    #[wasm_bindgen(getter)]
    pub fn entry_spacing(&self) -> i16 {
        return self.0.entry_spacing;
    }
    #[wasm_bindgen(getter)]
    pub fn subsequent_author_substitute(&self) -> Option<String> {
        return self.0.subsequent_author_substitute;
    }
    #[wasm_bindgen(getter)]
    pub fn subsequent_author_substitute_rule(&self) -> SubsequentAuthorSubstituteRule {
        return self.0.subsequent_author_substitute_rule;
    }
    #[wasm_bindgen(getter)]
    pub fn name_options(&self) -> InheritableNameOptions {
        return self.0.name_options;
    }
}

#[wasm_bindgen]
impl Bibliography {
    pub fn with_layout(layout: Layout) -> Self {
        return Self {
            0: citationberg_rs::Bibliography::with_layout(layout),
        };
    }
}
