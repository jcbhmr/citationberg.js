use citationberg;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub enum SortDirection {
    Ascending,
    Descending,
}
impl From<SortDirection> for citationberg::SortDirection {
    fn from(v: SortDirection) -> Self {
        match v {
            SortDirection::Ascending => Self::Ascending,
            SortDirection::Descending => Self::Descending,
        }
    }
}
impl From<citationberg::SortDirection> for SortDirection {
    fn from(v: citationberg::SortDirection) -> Self {
        match v {
            citationberg::SortDirection::Ascending => Self::Ascending,
            citationberg::SortDirection::Descending => Self::Descending,
        }
    }
}
