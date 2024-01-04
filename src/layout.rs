use crate::*;
use citationberg_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
pub struct Layout(pub(crate) citationberg_rs::Layout);

#[wasm_bindgen]
impl Layout {
    #[wasm_bindgen(constructor)]
    pub fn new(
        elements: Vec<LayoutRenderingElement>,
        font_style: Option<FontStyle>,
        font_variant: Option<FontVariant>,
        font_weight: Option<FontWeight>,
        text_decoration: Option<TextDecoration>,
        vertical_align: Option<VerticalAlign>,
        prefix: Option<String>,
        suffix: Option<String>,
        delimiter: Option<String>,
    ) -> Self {
        return Self {
            0: citationberg_rs::Layout {
                elements: elements.into_iter().map(|x| x.0).collect(),
                font_style: font_style.map(|x| x.0),
                font_variant: font_variant.map(|x| x.0),
                font_weight: font_weight.map(|x| x.0),
                text_decoration: text_decoration.map(|x| x.0),
                vertical_align: vertical_align.map(|x| x.0),
                prefix,
                suffix,
                delimiter,
            },
        };
    }

    #[wasm_bindgen(getter)]
    pub fn elements() -> Vec<LayoutRenderingElement> {
        return self.0.elements;
    }
    #[wasm_bindgen(getter)]
    pub fn font_style() -> Option<FontStyle> {
        return self.0.font_style;
    }
    #[wasm_bindgen(getter)]
    pub fn font_variant() -> Option<FontVariant> {
        return self.0.font_variant;
    }
    #[wasm_bindgen(getter)]
    pub fn font_weight() -> Option<FontWeight> {
        return self.0.font_weight;
    }
    #[wasm_bindgen(getter)]
    pub fn text_decoration() -> Option<TextDecoration> {
        return self.0.text_decoration;
    }
    #[wasm_bindgen(getter)]
    pub fn vertical_align() -> Option<VerticalAlign> {
        return self.0.vertical_align;
    }
    #[wasm_bindgen(getter)]
    pub fn prefix() -> Option<String> {
        return self.0.prefix;
    }
    #[wasm_bindgen(getter)]
    pub fn suffix() -> Option<String> {
        return self.0.suffix;
    }
    #[wasm_bindgen(getter)]
    pub fn delimiter() -> Option<String> {
        return self.0.delimiter;
    }
}
