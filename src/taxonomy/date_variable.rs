use crate::*;
use citationberg_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum DateVariable {
    Accessed,
    AvailableDate,
    EventDate,
    Issued,
    OriginalDate,
    Submitted,
}

impl DateVariable {
    pub fn from_citationberg_rs(x: &citationberg_rs::taxonomy::DateVariable) -> Self {
        return match x {
            citationberg_rs::taxonomy::DateVariable::Accessed => DateVariable::Accessed,
            citationberg_rs::taxonomy::DateVariable::AvailableDate => DateVariable::AvailableDate,
            citationberg_rs::taxonomy::DateVariable::EventDate => DateVariable::EventDate,
            citationberg_rs::taxonomy::DateVariable::Issued => DateVariable::Issued,
            citationberg_rs::taxonomy::DateVariable::OriginalDate => DateVariable::OriginalDate,
            citationberg_rs::taxonomy::DateVariable::Submitted => DateVariable::Submitted,
        };
    }
    pub fn to_citationberg_rs(&self) -> citationberg_rs::taxonomy::DateVariable {
        return match self {
            DateVariable::Accessed => citationberg_rs::taxonomy::DateVariable::Accessed,
            DateVariable::AvailableDate => citationberg_rs::taxonomy::DateVariable::AvailableDate,
            DateVariable::EventDate => citationberg_rs::taxonomy::DateVariable::EventDate,
            DateVariable::Issued => citationberg_rs::taxonomy::DateVariable::Issued,
            DateVariable::OriginalDate => citationberg_rs::taxonomy::DateVariable::OriginalDate,
            DateVariable::Submitted => citationberg_rs::taxonomy::DateVariable::Submitted,
        };
    }
}
