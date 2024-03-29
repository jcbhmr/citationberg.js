use crate::macros::my_parallel_enum;
use citationberg;
use wasm_bindgen::prelude::*;

my_parallel_enum!(Kind, citationberg::taxonomy::Kind, {
    Article,
    ArticleJournal,
    ArticleMagazine,
    ArticleNewspaper,
    Bill,
    Book,
    Broadcast,
    Chapter,
    Classic,
    Collection,
    Dataset,
    Document,
    Entry,
    EntryDictionary,
    EntryEncyclopedia,
    Event,
    Figure,
    Graphic,
    Hearing,
    Interview,
    LegalCase,
    Legislation,
    Manuscript,
    Map,
    MotionPicture,
    MusicalScore,
    Pamphlet,
    PaperConference,
    Patent,
    Performance,
    Periodical,
    PersonalCommunication,
    Post,
    PostWeblog,
    Regulation,
    Report,
    Review,
    ReviewBook,
    Software,
    Song,
    Speech,
    Standard,
    Thesis,
    Treaty,
    Webpage,
});
