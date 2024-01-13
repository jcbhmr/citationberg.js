use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct NumberVariable(#[wasm_bindgen(skip)] pub citationberg::taxonomy::NumberVariable);
#[wasm_bindgen]
impl NumberVariable {
    #[wasm_bindgen(js_name = "CollectionNumber")]
    pub fn new_collection_number() -> Self {
        citationberg::taxonomy::NumberVariable::CollectionNumber.into()
    }
    #[wasm_bindgen(js_name = "Edition")]
    pub fn new_edition() -> Self {
        citationberg::taxonomy::NumberVariable::Edition.into()
    }
    #[wasm_bindgen(js_name = "FirstReferenceNoteNumber")]
    pub fn new_first_reference_note_number() -> Self {
        citationberg::taxonomy::NumberVariable::FirstReferenceNoteNumber.into()
    }
    #[wasm_bindgen(js_name = "Issue")]
    pub fn new_issue() -> Self {
        citationberg::taxonomy::NumberVariable::Issue.into()
    }
    #[wasm_bindgen(js_name = "Locator")]
    pub fn new_locator() -> Self {
        citationberg::taxonomy::NumberVariable::Locator.into()
    }
    #[wasm_bindgen(js_name = "Number")]
    pub fn new_number() -> Self {
        citationberg::taxonomy::NumberVariable::Number.into()
    }
    #[wasm_bindgen(js_name = "NumberOfPages")]
    pub fn new_number_of_pages() -> Self {
        citationberg::taxonomy::NumberVariable::NumberOfPages.into()
    }
    #[wasm_bindgen(js_name = "NumberOfVolumes")]
    pub fn new_number_of_volumes() -> Self {
        citationberg::taxonomy::NumberVariable::NumberOfVolumes.into()
    }
    #[wasm_bindgen(js_name = "Page")]
    pub fn new_page() -> Self {
        citationberg::taxonomy::NumberVariable::Page.into()
    }
    #[wasm_bindgen(js_name = "PageFirst")]
    pub fn new_page_first() -> Self {
        citationberg::taxonomy::NumberVariable::PageFirst.into()
    }
    #[wasm_bindgen(js_name = "PartNumber")]
    pub fn new_part_number() -> Self {
        citationberg::taxonomy::NumberVariable::PartNumber.into()
    }
    #[wasm_bindgen(js_name = "PrintingNumber")]
    pub fn new_printing_number() -> Self {
        citationberg::taxonomy::NumberVariable::PrintingNumber.into()
    }
    #[wasm_bindgen(js_name = "Section")]
    pub fn new_section() -> Self {
        citationberg::taxonomy::NumberVariable::Section.into()
    }
    #[wasm_bindgen(js_name = "SupplementNumber")]
    pub fn new_supplement_number() -> Self {
        citationberg::taxonomy::NumberVariable::SupplementNumber.into()
    }
    #[wasm_bindgen(js_name = "Version")]
    pub fn new_version() -> Self {
        citationberg::taxonomy::NumberVariable::Version.into()
    }
    #[wasm_bindgen(js_name = "Volume")]
    pub fn new_volume() -> Self {
        citationberg::taxonomy::NumberVariable::Volume.into()
    }

    pub fn is_number_of_variable(&self) -> bool {
        self.0.clone().is_number_of_variable()
    }
}
