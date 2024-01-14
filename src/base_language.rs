use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(From, Into)]
pub struct BaseLanguage(#[wasm_bindgen(skip)] pub citationberg::BaseLanguage);
#[wasm_bindgen]
impl BaseLanguage {
    #[allow(non_snake_case)]
    pub fn Iso639_1(v: &[u8]) -> Self {
        citationberg::BaseLanguage::Iso639_1(v.try_into().unwrap_throw()).into()
    }
    #[allow(non_snake_case)]
    pub fn Iana(v: String) -> Self {
        citationberg::BaseLanguage::Iana(v).into()
    }
    #[allow(non_snake_case)]
    pub fn Unregistered(v: &[u8]) -> Self {
        citationberg::BaseLanguage::Unregistered(v.try_into().unwrap_throw()).into()
    }
}
#[wasm_bindgen]
impl BaseLanguage {
    #[wasm_bindgen(getter)]
    pub fn tag(&self) -> String {
        match &self.0 {
            citationberg::BaseLanguage::Iso639_1(_) => "Iso639_1".to_string(),
            citationberg::BaseLanguage::Iana(_) => "Iana".to_string(),
            citationberg::BaseLanguage::Unregistered(_) => "Unregistered".to_string(),
        }
    }
    #[wasm_bindgen(getter)]
    pub fn val(&self) -> JsValue {
        match &self.0 {
            citationberg::BaseLanguage::Iso639_1(v) => todo!(), // v.as_slice().into(),
            citationberg::BaseLanguage::Iana(v) => v.clone().into(),
            citationberg::BaseLanguage::Unregistered(v) => todo!(), // v.as_slice().into(),
        }
    }
}

impl Clone for BaseLanguage {
    fn clone(&self) -> Self {
        match &self.0 {
            citationberg::BaseLanguage::Iso639_1(v) => {
                citationberg::BaseLanguage::Iso639_1(v.clone())
            }
            citationberg::BaseLanguage::Iana(v) => citationberg::BaseLanguage::Iana(v.clone()),
            citationberg::BaseLanguage::Unregistered(v) => {
                citationberg::BaseLanguage::Unregistered(v.clone())
            }
        }
        .into()
    }
    fn clone_from(&mut self, source: &Self) {
        match (&mut self.0, &source.0) {
            (
                citationberg::BaseLanguage::Iso639_1(v),
                citationberg::BaseLanguage::Iso639_1(source),
            ) => v.clone_from(source),
            (citationberg::BaseLanguage::Iana(v), citationberg::BaseLanguage::Iana(source)) => {
                v.clone_from(source)
            }
            (
                citationberg::BaseLanguage::Unregistered(v),
                citationberg::BaseLanguage::Unregistered(source),
            ) => v.clone_from(source),
            _ => panic!("cannot clone across BaseLanguage variants"),
        }
        .into()
    }
}
