use crate::*;
use citationberg_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum NumberVariable {
    ChapterNumber,
    CitationNumber,
    CollectionNumber,
    Edition,
    FirstReferenceNoteNumber,
    Issue,
    Locator,
    Number,
    NumberOfPages,
    NumberOfVolumes,
    Page,
    PageFirst,
    PartNumber,
    PrintingNumber,
    Section,
    SupplementNumber,
    Version,
    Volume,
}

impl NumberVariable {
    pub fn from_citationberg_rs(x: &citationberg_rs::taxonomy::NumberVariable) -> Self {
        return match x {
            citationberg_rs::taxonomy::NumberVariable::ChapterNumber => {
                NumberVariable::ChapterNumber
            }
            citationberg_rs::taxonomy::NumberVariable::CitationNumber => {
                NumberVariable::CitationNumber
            }
            citationberg_rs::taxonomy::NumberVariable::CollectionNumber => {
                NumberVariable::CollectionNumber
            }
            citationberg_rs::taxonomy::NumberVariable::Edition => NumberVariable::Edition,
            citationberg_rs::taxonomy::NumberVariable::FirstReferenceNoteNumber => {
                NumberVariable::FirstReferenceNoteNumber
            }
            citationberg_rs::taxonomy::NumberVariable::Issue => NumberVariable::Issue,
            citationberg_rs::taxonomy::NumberVariable::Locator => NumberVariable::Locator,
            citationberg_rs::taxonomy::NumberVariable::Number => NumberVariable::Number,
            citationberg_rs::taxonomy::NumberVariable::NumberOfPages => {
                NumberVariable::NumberOfPages
            }
            citationberg_rs::taxonomy::NumberVariable::NumberOfVolumes => {
                NumberVariable::NumberOfVolumes
            }
            citationberg_rs::taxonomy::NumberVariable::Page => NumberVariable::Page,
            citationberg_rs::taxonomy::NumberVariable::PageFirst => NumberVariable::PageFirst,
            citationberg_rs::taxonomy::NumberVariable::PartNumber => NumberVariable::PartNumber,
            citationberg_rs::taxonomy::NumberVariable::PrintingNumber => {
                NumberVariable::PrintingNumber
            }
            citationberg_rs::taxonomy::NumberVariable::Section => NumberVariable::Section,
            citationberg_rs::taxonomy::NumberVariable::SupplementNumber => {
                NumberVariable::SupplementNumber
            }
            citationberg_rs::taxonomy::NumberVariable::Version => NumberVariable::Version,
            citationberg_rs::taxonomy::NumberVariable::Volume => NumberVariable::Volume,
        };
    }
    pub fn to_citationberg_rs(&self) -> citationberg_rs::taxonomy::NumberVariable {
        return match self {
            NumberVariable::ChapterNumber => {
                citationberg_rs::taxonomy::NumberVariable::ChapterNumber
            }
            NumberVariable::CitationNumber => {
                citationberg_rs::taxonomy::NumberVariable::CitationNumber
            }
            NumberVariable::CollectionNumber => {
                citationberg_rs::taxonomy::NumberVariable::CollectionNumber
            }
            NumberVariable::Edition => citationberg_rs::taxonomy::NumberVariable::Edition,
            NumberVariable::FirstReferenceNoteNumber => {
                citationberg_rs::taxonomy::NumberVariable::FirstReferenceNoteNumber
            }
            NumberVariable::Issue => citationberg_rs::taxonomy::NumberVariable::Issue,
            NumberVariable::Locator => citationberg_rs::taxonomy::NumberVariable::Locator,
            NumberVariable::Number => citationberg_rs::taxonomy::NumberVariable::Number,
            NumberVariable::NumberOfPages => {
                citationberg_rs::taxonomy::NumberVariable::NumberOfPages
            }
            NumberVariable::NumberOfVolumes => {
                citationberg_rs::taxonomy::NumberVariable::NumberOfVolumes
            }
            NumberVariable::Page => citationberg_rs::taxonomy::NumberVariable::Page,
            NumberVariable::PageFirst => citationberg_rs::taxonomy::NumberVariable::PageFirst,
            NumberVariable::PartNumber => citationberg_rs::taxonomy::NumberVariable::PartNumber,
            NumberVariable::PrintingNumber => {
                citationberg_rs::taxonomy::NumberVariable::PrintingNumber
            }
            NumberVariable::Section => citationberg_rs::taxonomy::NumberVariable::Section,
            NumberVariable::SupplementNumber => {
                citationberg_rs::taxonomy::NumberVariable::SupplementNumber
            }
            NumberVariable::Version => citationberg_rs::taxonomy::NumberVariable::Version,
            NumberVariable::Volume => citationberg_rs::taxonomy::NumberVariable::Volume,
        };
    }

    pub fn from_u32(x: u32) -> Self {
        return match x {
            0 => NumberVariable::ChapterNumber,
            1 => NumberVariable::CitationNumber,
            2 => NumberVariable::CollectionNumber,
            3 => NumberVariable::Edition,
            4 => NumberVariable::FirstReferenceNoteNumber,
            5 => NumberVariable::Issue,
            6 => NumberVariable::Locator,
            7 => NumberVariable::Number,
            8 => NumberVariable::NumberOfPages,
            9 => NumberVariable::NumberOfVolumes,
            10 => NumberVariable::Page,
            11 => NumberVariable::PageFirst,
            12 => NumberVariable::PartNumber,
            13 => NumberVariable::PrintingNumber,
            14 => NumberVariable::Section,
            15 => NumberVariable::SupplementNumber,
            16 => NumberVariable::Version,
            17 => NumberVariable::Volume,
            _ => panic!("NumberVariable out of range: {}", x),
        };
    }
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn NumberVariable_is_number_of_variable(x: u32) -> bool {
    let v = NumberVariable::from_u32(x);
    return match v {
        NumberVariable::ChapterNumber => false,
        NumberVariable::CitationNumber => false,
        NumberVariable::CollectionNumber => false,
        NumberVariable::Edition => false,
        NumberVariable::FirstReferenceNoteNumber => false,
        NumberVariable::Issue => false,
        NumberVariable::Locator => false,
        NumberVariable::Number => true,
        NumberVariable::NumberOfPages => true,
        NumberVariable::NumberOfVolumes => true,
        NumberVariable::Page => false,
        NumberVariable::PageFirst => false,
        NumberVariable::PartNumber => false,
        NumberVariable::PrintingNumber => false,
        NumberVariable::Section => false,
        NumberVariable::SupplementNumber => false,
        NumberVariable::Version => false,
        NumberVariable::Volume => false,
    };
}
