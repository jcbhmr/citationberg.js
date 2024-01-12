/*pub struct ChooseBranch {
    pub disambiguate: Option<bool>,
    pub is_numeric: Option<Vec<Variable>>,
    pub is_uncertain_date: Option<Vec<DateVariable>>,
    pub locator: Option<Vec<Locator>>,
    pub position: Option<Vec<TestPosition>>,
    pub type_: Option<Vec<Kind>>,
    pub variable: Option<Vec<Variable>>,
    pub match_: ChooseMatch,
    pub children: Vec<LayoutRenderingElement>,
}
impl ChooseBranch
source
pub fn test(&self) -> Option<ChooseTest<'_>>
Retrieve the test of this branch. Valid CSL files must return Some here. */

use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct ChooseBranch(#[wasm_bindgen(skip)] pub citationberg::ChooseBranch);
#[wasm_bindgen]
