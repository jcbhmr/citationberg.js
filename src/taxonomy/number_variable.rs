use citationberg;
use derive_more::{From, Into};
use duplicate::duplicate_item;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct NumberVariable(#[wasm_bindgen(skip)] pub citationberg::taxonomy::NumberVariable);
#[duplicate_item(
    variant;
    [ChapterNumber];
    [CitationNumber];
    [CollectionNumber];
    [Edition];
    [FirstReferenceNoteNumber];
    [Issue];
    [Locator];
    [Number];
    [NumberOfPages];
    [NumberOfVolumes];
    [Page];
    [PageFirst];
    [PartNumber];
    [PrintingNumber];
    [Section];
    [SupplementNumber];
    [Version];
    [Volume];
)]
#[wasm_bindgen]
impl NumberVariable {
    #[allow(non_snake_case)]
    pub fn variant() -> Self {
        citationberg::taxonomy::NumberVariable::variant.into()
    }
}
#[wasm_bindgen]
impl NumberVariable {
    #[wasm_bindgen(getter)]
    pub fn tag(&self) -> String {
        // HACK: Use 'Debug' trait to get the tag.
        format!("{:?}", &self.0)
    }
    #[wasm_bindgen(getter)]
    pub fn val(&self) -> JsValue {
        JsValue::UNDEFINED
    }
}
#[wasm_bindgen]
impl NumberVariable {
    pub fn is_number_of_variable(&self) -> bool {
        self.0.clone().is_number_of_variable()
    }
}
