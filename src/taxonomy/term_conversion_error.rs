use citationberg;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub enum TermConversionError {
    OutOfRange,
    Unknown,
}

impl From<TermConversionError> for citationberg::taxonomy::TermConversionError {
    fn from(v: TermConversionError) -> Self {
        match v {
            TermConversionError::OutOfRange => Self::OutOfRange,
            TermConversionError::Unknown => Self::Unknown,
        }
    }
}

impl From<citationberg::taxonomy::TermConversionError> for TermConversionError {
    fn from(v: citationberg::taxonomy::TermConversionError) -> Self {
        match v {
            citationberg::taxonomy::TermConversionError::OutOfRange => Self::OutOfRange,
            citationberg::taxonomy::TermConversionError::Unknown => Self::Unknown,
        }
    }
}
