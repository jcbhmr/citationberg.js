#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

// Loads 'wit/*.wit' from the root. This generates a 'target/bindings/**.rs'
// file with the generated Rust-side of the WIT types. We then need to fill in
// the blanks with the actual implementation.
cargo_component_bindings::generate!();

use crate::typst_community::citationberg::citationberg::*;
use bindings::exports::*;
use bindings::*;
use citationberg_rs;

pub mod taxonomy;

pub struct Component;
impl Guest for Component {
  
}
