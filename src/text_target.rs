use crate::*;
use citationberg_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub struct TextTarget(pub(crate) citationberg_rs::TextTarget);

// pub enum TextTarget {
//   Variable {
//       var: Variable,
//       form: LongShortForm,
//   },
//   Macro {
//       name: String,
//   },
//   Term {
//       term: Term,
//       form: TermForm,
//       plural: bool,
//   },
//   Value {
//       val: String,
//   },
// }

#[wasm_bindgen]
pub struct TextTarget_Variable {
    pub var: Variable,
    pub form: LongShortForm,
}
#[wasm_bindgen]
pub struct TextTarget_Macro {
    pub name: String,
}
#[wasm_bindgen]
pub struct TextTarget_Term {
    pub term: Term,
    pub form: TermForm,
    pub plural: bool,
}
#[wasm_bindgen]
pub struct TextTarget_Value {
    pub val: String,
}

#[wasm_bindgen]
impl TextTarget {
    pub fn Variable(var: Variable, form: LongShortForm) -> Self {
        return Self {
            0: citationberg_rs::TextTarget::Variable {
                var: var.0,
                form: form.0,
            },
        };
    }
    pub fn Macro(name: String) -> Self {
        return Self {
            0: citationberg_rs::TextTarget::Macro { name },
        };
    }
    pub fn Term(term: Term, form: TermForm, plural: bool) -> Self {
        return Self {
            0: citationberg_rs::TextTarget::Term {
                term: term.0,
                form: form.0,
                plural,
            },
        };
    }
    pub fn Value(val: String) -> Self {
        return Self {
            0: citationberg_rs::TextTarget::Value { val },
        };
    }

    #[wasm_bindgen(getter)]
    pub fn type_name(&self) -> String {
        return match self.0 {
            citationberg_rs::TextTarget::Variable { .. } => "Variable".to_string(),
            citationberg_rs::TextTarget::Macro { .. } => "Macro".to_string(),
            citationberg_rs::TextTarget::Term { .. } => "Term".to_string(),
            citationberg_rs::TextTarget::Value { .. } => "Value".to_string(),
        };
    }
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> JsValue {
        return match self.0 {
            citationberg_rs::TextTarget::Variable { var, form } => TextTarget_Variable {
                var: Variable { 0: var.clone() },
                form: LongShortForm { 0: form.clone() },
            },
            citationberg_rs::TextTarget::Macro { name } => TextTarget_Macro { name },
            citationberg_rs::TextTarget::Term { term, form, plural } => TextTarget_Term {
                term: Term { 0: term.clone() },
                form: TermForm { 0: form.clone() },
                plural,
            },
            citationberg_rs::TextTarget::Value { val } => TextTarget_Value { val },
        };
    }
}
