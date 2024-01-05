#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

// Loads 'wit/*.wit' from the root. This generates a 'target/bindings/**.rs'
// file with the generated Rust-side of the WIT types. We then need to fill in
// the blanks with the actual implementation.
cargo_component_bindings::generate!();

use crate::typst_community::citationberg::citationberg::*;
use bindings::exports::*;
use bindings::*;
use citationberg_rs;

mod affixes;
mod citation;

pub use affixes::*;
pub use citation::*;

pub struct Component;

// impl Guest for Component {
//     //#region bibliography
//     fn bibliography_with_layout(layout: ToDo) -> Bibliography {
//         todo!()
//     }
//     //#endregion

//     //#region choose
//     fn choose_branches(s: Choose) -> Vec<ToDo> {
//         todo!()
//     }
//     fn choose_find_variable_element(s: Choose, variable: ToDo, macros: Vec<ToDo>) -> Option<ToDo> {
//         todo!()
//     }
//     //#endregion

//     //#region choose-branch
//     fn choose_branch_test(s: ChooseBranch) -> Option<ToDo> {
//         todo!()
//     }
//     //#endregion
// }
