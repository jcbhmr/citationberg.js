use citationberg;
use wasm_bindgen::prelude::*;

use crate::macros::my_parallel_enum;

my_parallel_enum!(Locator, citationberg::taxonomy::Locator, {
    Act,
    Appendix,
    ArticleLocator,
    Book,
    Canon,
    Chapter,
    Column,
    Elocation,
    Equation,
    Figure,
    Folio,
    Issue,
    Line,
    Note,
    Opus,
    Page,
    Paragraph,
    Part,
    Rule,
    Scene,
    Section,
    SubVerbo,
    Supplement,
    Table,
    Timestamp,
    Title,
    TitleLocator,
    Verse,
    Volume,
    Custom,
});
