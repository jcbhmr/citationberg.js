use citationberg;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
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

impl From<NameVariable> for citationberg::taxonomy::NameVariable {
    fn from(v: NameVariable) -> Self {
        match v {
            NameVariable::Author => Self::Author,
            NameVariable::Chair => Self::Chair,
            NameVariable::CollectionEditor => Self::CollectionEditor,
            NameVariable::Compiler => Self::Compiler,
            NameVariable::Composer => Self::Composer,
            NameVariable::ContainerAuthor => Self::ContainerAuthor,
            NameVariable::Contributor => Self::Contributor,
            NameVariable::Curator => Self::Curator,
            NameVariable::Director => Self::Director,
            NameVariable::Editor => Self::Editor,
            NameVariable::EditorialDirector => Self::EditorialDirector,
            NameVariable::EditorTranslator => Self::EditorTranslator,
            NameVariable::ExecutiveProducer => Self::ExecutiveProducer,
            NameVariable::Guest => Self::Guest,
            NameVariable::Host => Self::Host,
            NameVariable::Illustrator => Self::Illustrator,
            NameVariable::Interviewer => Self::Interviewer,
            NameVariable::Narrator => Self::Narrator,
            NameVariable::Organizer => Self::Organizer,
            NameVariable::OriginalAuthor => Self::OriginalAuthor,
            NameVariable::Performer => Self::Performer,
            NameVariable::Producer => Self::Producer,
            NameVariable::Recipient => Self::Recipient,
            NameVariable::ReviewedAuthor => Self::ReviewedAuthor,
            NameVariable::ScriptWriter => Self::ScriptWriter,
            NameVariable::SeriesCreator => Self::SeriesCreator,
            NameVariable::Translator => Self::Translator,
        }
    }
}

impl From<citationberg::taxonomy::NameVariable> for NameVariable {
    fn from(v: citationberg::taxonomy::NameVariable) -> Self {
        match v {
            citationberg::taxonomy::NameVariable::Author => Self::Author,
            citationberg::taxonomy::NameVariable::Chair => Self::Chair,
            citationberg::taxonomy::NameVariable::CollectionEditor => Self::CollectionEditor,
            citationberg::taxonomy::NameVariable::Compiler => Self::Compiler,
            citationberg::taxonomy::NameVariable::Composer => Self::Composer,
            citationberg::taxonomy::NameVariable::ContainerAuthor => Self::ContainerAuthor,
            citationberg::taxonomy::NameVariable::Contributor => Self::Contributor,
            citationberg::taxonomy::NameVariable::Curator => Self::Curator,
            citationberg::taxonomy::NameVariable::Director => Self::Director,
            citationberg::taxonomy::NameVariable::Editor => Self::Editor,
            citationberg::taxonomy::NameVariable::EditorialDirector => Self::EditorialDirector,
            citationberg::taxonomy::NameVariable::EditorTranslator => Self::EditorTranslator,
            citationberg::taxonomy::NameVariable::ExecutiveProducer => Self::ExecutiveProducer,
            citationberg::taxonomy::NameVariable::Guest => Self::Guest,
            citationberg::taxonomy::NameVariable::Host => Self::Host,
            citationberg::taxonomy::NameVariable::Illustrator => Self::Illustrator,
            citationberg::taxonomy::NameVariable::Interviewer => Self::Interviewer,
            citationberg::taxonomy::NameVariable::Narrator => Self::Narrator,
            citationberg::taxonomy::NameVariable::Organizer => Self::Organizer,
            citationberg::taxonomy::NameVariable::OriginalAuthor => Self::OriginalAuthor,
            citationberg::taxonomy::NameVariable::Performer => Self::Performer,
            citationberg::taxonomy::NameVariable::Producer => Self::Producer,
            citationberg::taxonomy::NameVariable::Recipient => Self::Recipient,
            citationberg::taxonomy::NameVariable::ReviewedAuthor => Self::ReviewedAuthor,
            citationberg::taxonomy::NameVariable::ScriptWriter => Self::ScriptWriter,
            citationberg::taxonomy::NameVariable::SeriesCreator => Self::SeriesCreator,
            citationberg::taxonomy::NameVariable::Translator => Self::Translator,
        }
    }
}
