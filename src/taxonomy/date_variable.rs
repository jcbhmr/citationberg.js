use citationberg;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub enum DateVariable {
    Accessed,
    AvailableDate,
    EventDate,
    Issued,
    OriginalDate,
    Submitted,
}
impl From<DateVariable> for citationberg::taxonomy::DateVariable {
    fn from(v: DateVariable) -> Self {
        match v {
            DateVariable::Accessed => Self::Accessed,
            DateVariable::AvailableDate => Self::AvailableDate,
            DateVariable::EventDate => Self::EventDate,
            DateVariable::Issued => Self::Issued,
            DateVariable::OriginalDate => Self::OriginalDate,
            DateVariable::Submitted => Self::Submitted,
        }
    }
}
impl From<citationberg::taxonomy::DateVariable> for DateVariable {
    fn from(v: citationberg::taxonomy::DateVariable) -> Self {
        match v {
            citationberg::taxonomy::DateVariable::Accessed => Self::Accessed,
            citationberg::taxonomy::DateVariable::AvailableDate => Self::AvailableDate,
            citationberg::taxonomy::DateVariable::EventDate => Self::EventDate,
            citationberg::taxonomy::DateVariable::Issued => Self::Issued,
            citationberg::taxonomy::DateVariable::OriginalDate => Self::OriginalDate,
            citationberg::taxonomy::DateVariable::Submitted => Self::Submitted,
        }
    }
}
