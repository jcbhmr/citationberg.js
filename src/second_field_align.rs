use crate::macros::my_parallel_enum;
use citationberg;
use wasm_bindgen::prelude::*;

my_parallel_enum!(SecondFieldAlign, citationberg::SecondFieldAlign, {
  Margin,
  Flush,
});
