use crate::*;
use citationberg_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum NameVariable {
    Author,
    Chair,
    CollectionEditor,
    Compiler,
    Composer,
    ContainerAuthor,
    Contributor,
    Curator,
    Director,
    Editor,
    EditorialDirector,
    EditorTranslator,
    ExecutiveProducer,
    Guest,
    Host,
    Illustrator,
    Interviewer,
    Narrator,
    Organizer,
    OriginalAuthor,
    Performer,
    Producer,
    Recipient,
    ReviewedAuthor,
    ScriptWriter,
    SeriesCreator,
    Translator,
}

impl NameVariable {
    pub fn from_citationberg_rs(x: &citationberg_rs::taxonomy::NameVariable) -> Self {
        return match x {
            citationberg_rs::taxonomy::NameVariable::Author => NameVariable::Author,
            citationberg_rs::taxonomy::NameVariable::Chair => NameVariable::Chair,
            citationberg_rs::taxonomy::NameVariable::CollectionEditor => {
                NameVariable::CollectionEditor
            }
            citationberg_rs::taxonomy::NameVariable::Compiler => NameVariable::Compiler,
            citationberg_rs::taxonomy::NameVariable::Composer => NameVariable::Composer,
            citationberg_rs::taxonomy::NameVariable::ContainerAuthor => {
                NameVariable::ContainerAuthor
            }
            citationberg_rs::taxonomy::NameVariable::Contributor => NameVariable::Contributor,
            citationberg_rs::taxonomy::NameVariable::Curator => NameVariable::Curator,
            citationberg_rs::taxonomy::NameVariable::Director => NameVariable::Director,
            citationberg_rs::taxonomy::NameVariable::Editor => NameVariable::Editor,
            citationberg_rs::taxonomy::NameVariable::EditorialDirector => {
                NameVariable::EditorialDirector
            }
            citationberg_rs::taxonomy::NameVariable::EditorTranslator => {
                NameVariable::EditorTranslator
            }
            citationberg_rs::taxonomy::NameVariable::ExecutiveProducer => {
                NameVariable::ExecutiveProducer
            }
            citationberg_rs::taxonomy::NameVariable::Guest => NameVariable::Guest,
            citationberg_rs::taxonomy::NameVariable::Host => NameVariable::Host,
            citationberg_rs::taxonomy::NameVariable::Illustrator => NameVariable::Illustrator,
            citationberg_rs::taxonomy::NameVariable::Interviewer => NameVariable::Interviewer,
            citationberg_rs::taxonomy::NameVariable::Narrator => NameVariable::Narrator,
            citationberg_rs::taxonomy::NameVariable::Organizer => NameVariable::Organizer,
            citationberg_rs::taxonomy::NameVariable::OriginalAuthor => NameVariable::OriginalAuthor,
            citationberg_rs::taxonomy::NameVariable::Performer => NameVariable::Performer,
            citationberg_rs::taxonomy::NameVariable::Producer => NameVariable::Producer,
            citationberg_rs::taxonomy::NameVariable::Recipient => NameVariable::Recipient,
            citationberg_rs::taxonomy::NameVariable::ReviewedAuthor => NameVariable::ReviewedAuthor,
            citationberg_rs::taxonomy::NameVariable::ScriptWriter => NameVariable::ScriptWriter,
            citationberg_rs::taxonomy::NameVariable::SeriesCreator => NameVariable::SeriesCreator,
            citationberg_rs::taxonomy::NameVariable::Translator => NameVariable::Translator,
        };
    }
    pub fn to_citationberg_rs(&self) -> citationberg_rs::taxonomy::NameVariable {
        return match self {
            NameVariable::Author => citationberg_rs::taxonomy::NameVariable::Author,
            NameVariable::Chair => citationberg_rs::taxonomy::NameVariable::Chair,
            NameVariable::CollectionEditor => {
                citationberg_rs::taxonomy::NameVariable::CollectionEditor
            }
            NameVariable::Compiler => citationberg_rs::taxonomy::NameVariable::Compiler,
            NameVariable::Composer => citationberg_rs::taxonomy::NameVariable::Composer,
            NameVariable::ContainerAuthor => {
                citationberg_rs::taxonomy::NameVariable::ContainerAuthor
            }
            NameVariable::Contributor => citationberg_rs::taxonomy::NameVariable::Contributor,
            NameVariable::Curator => citationberg_rs::taxonomy::NameVariable::Curator,
            NameVariable::Director => citationberg_rs::taxonomy::NameVariable::Director,
            NameVariable::Editor => citationberg_rs::taxonomy::NameVariable::Editor,
            NameVariable::EditorialDirector => {
                citationberg_rs::taxonomy::NameVariable::EditorialDirector
            }
            NameVariable::EditorTranslator => {
                citationberg_rs::taxonomy::NameVariable::EditorTranslator
            }
            NameVariable::ExecutiveProducer => {
                citationberg_rs::taxonomy::NameVariable::ExecutiveProducer
            }
            NameVariable::Guest => citationberg_rs::taxonomy::NameVariable::Guest,
            NameVariable::Host => citationberg_rs::taxonomy::NameVariable::Host,
            NameVariable::Illustrator => citationberg_rs::taxonomy::NameVariable::Illustrator,
            NameVariable::Interviewer => citationberg_rs::taxonomy::NameVariable::Interviewer,
            NameVariable::Narrator => citationberg_rs::taxonomy::NameVariable::Narrator,
            NameVariable::Organizer => citationberg_rs::taxonomy::NameVariable::Organizer,
            NameVariable::OriginalAuthor => citationberg_rs::taxonomy::NameVariable::OriginalAuthor,
            NameVariable::Performer => citationberg_rs::taxonomy::NameVariable::Performer,
            NameVariable::Producer => citationberg_rs::taxonomy::NameVariable::Producer,
            NameVariable::Recipient => citationberg_rs::taxonomy::NameVariable::Recipient,
            NameVariable::ReviewedAuthor => citationberg_rs::taxonomy::NameVariable::ReviewedAuthor,
            NameVariable::ScriptWriter => citationberg_rs::taxonomy::NameVariable::ScriptWriter,
            NameVariable::SeriesCreator => citationberg_rs::taxonomy::NameVariable::SeriesCreator,
            NameVariable::Translator => citationberg_rs::taxonomy::NameVariable::Translator,
        };
    }
}
