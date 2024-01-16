use citationberg;
use derive_more::{From, Into};
use duplicate::duplicate_item;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct ChooseMatch(#[wasm_bindgen(skip)] pub citationberg::ChooseMatch);
#[duplicate_item(
    variant;
    [All];
    [Any];
    [None];
)]
#[wasm_bindgen]
impl ChooseMatch {
    #[allow(non_snake_case)]
    pub fn variant() -> Self {
        citationberg::ChooseMatch::variant.into()
    }
}
#[wasm_bindgen]
impl ChooseMatch {
    #[wasm_bindgen(getter)]
    pub fn tag(&self) -> String {
        format!("{:?}", &self.0)
    }
    #[wasm_bindgen(getter)]
    pub fn val(&self) -> JsValue {
        JsValue::UNDEFINED
    }
}
#[wasm_bindgen]
impl ChooseMatch {
  pub fn test(&self, tests: Vec<JsValue>) -> bool {
    let tests: Vec<bool> = tests.into_iter().map(|v| v.as_bool().expect_throw("not a bool")).collect();
    self.0.clone().test(tests.into_iter())
  }
}
