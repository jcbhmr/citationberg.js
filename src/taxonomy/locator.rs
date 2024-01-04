use crate::*;
use citationberg_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum Locator {
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
}

impl Locator {
    pub fn from_citationberg_rs(x: &citationberg_rs::taxonomy::Locator) -> Self {
        return match x {
            citationberg_rs::taxonomy::Locator::Act => Locator::Act,
            citationberg_rs::taxonomy::Locator::Appendix => Locator::Appendix,
            citationberg_rs::taxonomy::Locator::ArticleLocator => Locator::ArticleLocator,
            citationberg_rs::taxonomy::Locator::Book => Locator::Book,
            citationberg_rs::taxonomy::Locator::Canon => Locator::Canon,
            citationberg_rs::taxonomy::Locator::Chapter => Locator::Chapter,
            citationberg_rs::taxonomy::Locator::Column => Locator::Column,
            citationberg_rs::taxonomy::Locator::Elocation => Locator::Elocation,
            citationberg_rs::taxonomy::Locator::Equation => Locator::Equation,
            citationberg_rs::taxonomy::Locator::Figure => Locator::Figure,
            citationberg_rs::taxonomy::Locator::Folio => Locator::Folio,
            citationberg_rs::taxonomy::Locator::Issue => Locator::Issue,
            citationberg_rs::taxonomy::Locator::Line => Locator::Line,
            citationberg_rs::taxonomy::Locator::Note => Locator::Note,
            citationberg_rs::taxonomy::Locator::Opus => Locator::Opus,
            citationberg_rs::taxonomy::Locator::Page => Locator::Page,
            citationberg_rs::taxonomy::Locator::Paragraph => Locator::Paragraph,
            citationberg_rs::taxonomy::Locator::Part => Locator::Part,
            citationberg_rs::taxonomy::Locator::Rule => Locator::Rule,
            citationberg_rs::taxonomy::Locator::Scene => Locator::Scene,
            citationberg_rs::taxonomy::Locator::Section => Locator::Section,
            citationberg_rs::taxonomy::Locator::SubVerbo => Locator::SubVerbo,
            citationberg_rs::taxonomy::Locator::Supplement => Locator::Supplement,
            citationberg_rs::taxonomy::Locator::Table => Locator::Table,
            citationberg_rs::taxonomy::Locator::Timestamp => Locator::Timestamp,
            citationberg_rs::taxonomy::Locator::Title => Locator::Title,
            citationberg_rs::taxonomy::Locator::TitleLocator => Locator::TitleLocator,
            citationberg_rs::taxonomy::Locator::Verse => Locator::Verse,
            citationberg_rs::taxonomy::Locator::Volume => Locator::Volume,
            citationberg_rs::taxonomy::Locator::Custom => Locator::Custom,
        };
    }
    pub fn to_citationberg_rs(&self) -> citationberg_rs::taxonomy::Locator {
        return match self {
            Locator::Act => citationberg_rs::taxonomy::Locator::Act,
            Locator::Appendix => citationberg_rs::taxonomy::Locator::Appendix,
            Locator::ArticleLocator => citationberg_rs::taxonomy::Locator::ArticleLocator,
            Locator::Book => citationberg_rs::taxonomy::Locator::Book,
            Locator::Canon => citationberg_rs::taxonomy::Locator::Canon,
            Locator::Chapter => citationberg_rs::taxonomy::Locator::Chapter,
            Locator::Column => citationberg_rs::taxonomy::Locator::Column,
            Locator::Elocation => citationberg_rs::taxonomy::Locator::Elocation,
            Locator::Equation => citationberg_rs::taxonomy::Locator::Equation,
            Locator::Figure => citationberg_rs::taxonomy::Locator::Figure,
            Locator::Folio => citationberg_rs::taxonomy::Locator::Folio,
            Locator::Issue => citationberg_rs::taxonomy::Locator::Issue,
            Locator::Line => citationberg_rs::taxonomy::Locator::Line,
            Locator::Note => citationberg_rs::taxonomy::Locator::Note,
            Locator::Opus => citationberg_rs::taxonomy::Locator::Opus,
            Locator::Page => citationberg_rs::taxonomy::Locator::Page,
            Locator::Paragraph => citationberg_rs::taxonomy::Locator::Paragraph,
            Locator::Part => citationberg_rs::taxonomy::Locator::Part,
            Locator::Rule => citationberg_rs::taxonomy::Locator::Rule,
            Locator::Scene => citationberg_rs::taxonomy::Locator::Scene,
            Locator::Section => citationberg_rs::taxonomy::Locator::Section,
            Locator::SubVerbo => citationberg_rs::taxonomy::Locator::SubVerbo,
            Locator::Supplement => citationberg_rs::taxonomy::Locator::Supplement,
            Locator::Table => citationberg_rs::taxonomy::Locator::Table,
            Locator::Timestamp => citationberg_rs::taxonomy::Locator::Timestamp,
            Locator::Title => citationberg_rs::taxonomy::Locator::Title,
            Locator::TitleLocator => citationberg_rs::taxonomy::Locator::TitleLocator,
            Locator::Verse => citationberg_rs::taxonomy::Locator::Verse,
            Locator::Volume => citationberg_rs::taxonomy::Locator::Volume,
            Locator::Custom => citationberg_rs::taxonomy::Locator::Custom,
        };
    }
}