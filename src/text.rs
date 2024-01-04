use crate::*;
use citationberg_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub struct Text(pub(crate) citationberg_rs::Text);

#[wasm_bindgen]
impl Text {
    #[wasm_bindgen(constructor)]
    pub fn new(
        target: TextTarget,
        formatting: Formatting,
        affixes: Affixes,
        display: Option<Display>,
        quotes: bool,
        strip_periods: bool,
        text_case: Option<TextCase>,
    ) -> Self {
        return Self {
            0: citationberg_rs::Text {
                target: target.0,
                formatting: formatting.0,
                affixes: affixes.0,
                display: display.map(|x| x.0),
                quotes,
                strip_periods,
                text_case: text_case.map(|x| x.0),
            },
        };
    }

    #[wasm_bindgen(getter)]
    pub fn target(&self) -> TextTarget {
        return TextTarget {
            0: self.0.target.clone(),
        };
    }
    #[wasm_bindgen(getter)]
    pub fn formatting(&self) -> Formatting {
        return Formatting {
            0: self.0.formatting.clone(),
        };
    }
    #[wasm_bindgen(getter)]
    pub fn affixes(&self) -> Affixes {
        return Affixes {
            0: self.0.affixes.clone(),
        };
    }
    #[wasm_bindgen(getter)]
    pub fn display(&self) -> Option<Display> {
        return self.0.display.map(|x| Display { 0: x.clone() });
    }
    #[wasm_bindgen(getter)]
    pub fn quotes(&self) -> bool {
        return self.0.quotes;
    }
    #[wasm_bindgen(getter)]
    pub fn strip_periods(&self) -> bool {
        return self.0.strip_periods;
    }
    #[wasm_bindgen(getter)]
    pub fn text_case(&self) -> Option<TextCase> {
        return self.0.text_case.map(|x| TextCase { 0: x.clone() });
    }
}
