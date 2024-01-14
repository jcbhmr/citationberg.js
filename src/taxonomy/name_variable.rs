use crate::macros::my_parallel_enum;
use citationberg;
use wasm_bindgen::prelude::*;

my_parallel_enum!(NameVariable, citationberg::taxonomy::NameVariable, {
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
});
