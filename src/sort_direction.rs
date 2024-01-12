use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub enum SortDirection {
    Ascending,
    Descending,
}
impl From<SortDirection> for citationberg::SortDirection {
    fn from(sort_direction: SortDirection) -> Self {
        match sort_direction {
            SortDirection::Ascending => Self::Ascending,
            SortDirection::Descending => Self::Descending,
        }
    }
}
impl From<citationberg::SortDirection> for SortDirection {
    fn from(sort_direction: citationberg::SortDirection) -> Self {
        match sort_direction {
            citationberg::SortDirection::Ascending => Self::Ascending,
            citationberg::SortDirection::Descending => Self::Descending,
        }
    }
}
