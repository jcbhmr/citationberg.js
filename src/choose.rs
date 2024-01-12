/*pub struct Choose {
    pub if_: ChooseBranch,
    pub else_if: Vec<ChooseBranch>,
    pub otherwise: Option<ElseBranch>,
    pub delimiter: Option<String>,
}

impl Choose
source
pub fn branches(&self) -> impl Iterator<Item = &ChooseBranch>
Return an iterator over all branches with a condition.

source
pub fn find_variable_element(
    &self,
    variable: Variable,
    macros: &[CslMacro]
) -> Option<LayoutRenderingElement>
Find the child element that renders the given variable.*/

use citationberg;
use derive_more::{From, Into};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct Choose(#[wasm_bindgen(skip)] pub citationberg::Choose);
#[wasm_bindgen]
impl Choose {
    #[wasm_bindgen(getter)]
    pub fn if_(&self) -> crate::ChooseBranch {
        self.0.if_.clone().into()
    }
    #[wasm_bindgen(setter)]
    pub fn set_if(&mut self, v: crate::ChooseBranch) {
        self.0.if_ = v.into();
    }
    #[wasm_bindgen(getter)]
    pub fn else_if(&self) -> Vec<crate::ChooseBranch> {
        self.0.else_if.iter().map(|x| x.clone().into()).collect()
    }
    #[wasm_bindgen(setter)]
    pub fn set_else_if(&mut self, v: Vec<crate::ChooseBranch>) {
        self.0.else_if = v.into_iter().map(|x| x.into()).collect();
    }
    #[wasm_bindgen(getter)]
    pub fn otherwise(&self) -> Option<crate::ElseBranch> {
        self.0.otherwise.clone().map(|x| x.into())
    }
    #[wasm_bindgen(setter)]
    pub fn set_otherwise(&mut self, v: Option<crate::ElseBranch>) {
        self.0.otherwise = v.map(|x| x.into());
    }
    #[wasm_bindgen(getter)]
    pub fn delimiter(&self) -> Option<String> {
        self.0.delimiter.clone()
    }
    #[wasm_bindgen(setter)]
    pub fn set_delimiter(&mut self, v: Option<String>) {
        self.0.delimiter = v;
    }
    pub fn branches(&self) -> Vec<crate::ChooseBranch> {
        self.branches().map(|x| x.clone().into()).collect()
    }
}
