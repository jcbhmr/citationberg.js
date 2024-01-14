use crate::macros::my_parallel_enum;
use citationberg;
use wasm_bindgen::prelude::*;

my_parallel_enum!(DateVariable, citationberg::taxonomy::DateVariable, {
    Accessed,
    AvailableDate,
    EventDate,
    Issued,
    OriginalDate,
    Submitted,
});
