use crate::macros::my_parallel_enum;
use citationberg;
use wasm_bindgen::prelude::*;

my_parallel_enum!(TestPosition, citationberg::TestPosition, {
  First,
  Subsequent,
  IbidWithLocator,
  Ibid,
  NearNote,
});
