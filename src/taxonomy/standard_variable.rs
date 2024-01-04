use crate::*;
use citationberg_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum StandardVariable {
    Abstract,
    Annote,
    Archive,
    ArchiveCollection,
    ArchiveLocation,
    ArchivePlace,
    Authority,
    CallNumber,
    CitationKey,
    CitationLabel,
    CollectionTitle,
    ContainerTitle,
    ContainerTitleShort,
    Dimensions,
    Division,
    DOI,
    Event,
    EventTitle,
    EventPlace,
    Genre,
    ISBN,
    ISSN,
    Jurisdiction,
    Keyword,
    Language,
    License,
    Medium,
    Note,
    OriginalPublisher,
    OriginalPublisherPlace,
    OriginalTitle,
    PartTitle,
    PMCID,
    PMID,
    Publisher,
    PublisherPlace,
    References,
    ReviewedGenre,
    ReviewedTitle,
    Scale,
    Source,
    Status,
    Title,
    TitleShort,
    URL,
    VolumeTitle,
    YearSuffix,
}

impl StandardVariable {
    pub fn from_citationberg_rs(x: &citationberg_rs::taxonomy::StandardVariable) -> Self {
        return match x {
            citationberg_rs::taxonomy::StandardVariable::Abstract => StandardVariable::Abstract,
            citationberg_rs::taxonomy::StandardVariable::Annote => StandardVariable::Annote,
            citationberg_rs::taxonomy::StandardVariable::Archive => StandardVariable::Archive,
            citationberg_rs::taxonomy::StandardVariable::ArchiveCollection => {
                StandardVariable::ArchiveCollection
            }
            citationberg_rs::taxonomy::StandardVariable::ArchiveLocation => {
                StandardVariable::ArchiveLocation
            }
            citationberg_rs::taxonomy::StandardVariable::ArchivePlace => {
                StandardVariable::ArchivePlace
            }
            citationberg_rs::taxonomy::StandardVariable::Authority => StandardVariable::Authority,
            citationberg_rs::taxonomy::StandardVariable::CallNumber => StandardVariable::CallNumber,
            citationberg_rs::taxonomy::StandardVariable::CitationKey => {
                StandardVariable::CitationKey
            }
            citationberg_rs::taxonomy::StandardVariable::CitationLabel => {
                StandardVariable::CitationLabel
            }
            citationberg_rs::taxonomy::StandardVariable::CollectionTitle => {
                StandardVariable::CollectionTitle
            }
            citationberg_rs::taxonomy::StandardVariable::ContainerTitle => {
                StandardVariable::ContainerTitle
            }
            citationberg_rs::taxonomy::StandardVariable::ContainerTitleShort => {
                StandardVariable::ContainerTitleShort
            }
            citationberg_rs::taxonomy::StandardVariable::Dimensions => StandardVariable::Dimensions,
            citationberg_rs::taxonomy::StandardVariable::Division => StandardVariable::Division,
            citationberg_rs::taxonomy::StandardVariable::DOI => StandardVariable::DOI,
            citationberg_rs::taxonomy::StandardVariable::Event => StandardVariable::Event,
            citationberg_rs::taxonomy::StandardVariable::EventTitle => StandardVariable::EventTitle,
            citationberg_rs::taxonomy::StandardVariable::EventPlace => StandardVariable::EventPlace,
            citationberg_rs::taxonomy::StandardVariable::Genre => StandardVariable::Genre,
            citationberg_rs::taxonomy::StandardVariable::ISBN => StandardVariable::ISBN,
            citationberg_rs::taxonomy::StandardVariable::ISSN => StandardVariable::ISSN,
            citationberg_rs::taxonomy::StandardVariable::Jurisdiction => {
                StandardVariable::Jurisdiction
            }
            citationberg_rs::taxonomy::StandardVariable::Keyword => StandardVariable::Keyword,
            citationberg_rs::taxonomy::StandardVariable::Language => StandardVariable::Language,
            citationberg_rs::taxonomy::StandardVariable::License => StandardVariable::License,
            citationberg_rs::taxonomy::StandardVariable::Medium => StandardVariable::Medium,
            citationberg_rs::taxonomy::StandardVariable::Note => StandardVariable::Note,
            citationberg_rs::taxonomy::StandardVariable::OriginalPublisher => {
                StandardVariable::OriginalPublisher
            }
            citationberg_rs::taxonomy::StandardVariable::OriginalPublisherPlace => {
                StandardVariable::OriginalPublisherPlace
            }
            citationberg_rs::taxonomy::StandardVariable::OriginalTitle => {
                StandardVariable::OriginalTitle
            }
            citationberg_rs::taxonomy::StandardVariable::PartTitle => StandardVariable::PartTitle,
            citationberg_rs::taxonomy::StandardVariable::PMCID => StandardVariable::PMCID,
            citationberg_rs::taxonomy::StandardVariable::PMID => StandardVariable::PMID,
            citationberg_rs::taxonomy::StandardVariable::Publisher => StandardVariable::Publisher,
            citationberg_rs::taxonomy::StandardVariable::PublisherPlace => {
                StandardVariable::PublisherPlace
            }
            citationberg_rs::taxonomy::StandardVariable::References => StandardVariable::References,
            citationberg_rs::taxonomy::StandardVariable::ReviewedGenre => {
                StandardVariable::ReviewedGenre
            }
            citationberg_rs::taxonomy::StandardVariable::ReviewedTitle => {
                StandardVariable::ReviewedTitle
            }
            citationberg_rs::taxonomy::StandardVariable::Scale => StandardVariable::Scale,
            citationberg_rs::taxonomy::StandardVariable::Source => StandardVariable::Source,
            citationberg_rs::taxonomy::StandardVariable::Status => StandardVariable::Status,
            citationberg_rs::taxonomy::StandardVariable::Title => StandardVariable::Title,
            citationberg_rs::taxonomy::StandardVariable::TitleShort => StandardVariable::TitleShort,
            citationberg_rs::taxonomy::StandardVariable::URL => StandardVariable::URL,
            citationberg_rs::taxonomy::StandardVariable::VolumeTitle => {
                StandardVariable::VolumeTitle
            }
            citationberg_rs::taxonomy::StandardVariable::YearSuffix => StandardVariable::YearSuffix,
        };
    }
    pub fn to_citationberg_rs(&self) -> citationberg_rs::taxonomy::StandardVariable {
        return match self {
            StandardVariable::Abstract => citationberg_rs::taxonomy::StandardVariable::Abstract,
            StandardVariable::Annote => citationberg_rs::taxonomy::StandardVariable::Annote,
            StandardVariable::Archive => citationberg_rs::taxonomy::StandardVariable::Archive,
            StandardVariable::ArchiveCollection => {
                citationberg_rs::taxonomy::StandardVariable::ArchiveCollection
            }
            StandardVariable::ArchiveLocation => {
                citationberg_rs::taxonomy::StandardVariable::ArchiveLocation
            }
            StandardVariable::ArchivePlace => {
                citationberg_rs::taxonomy::StandardVariable::ArchivePlace
            }
            StandardVariable::Authority => citationberg_rs::taxonomy::StandardVariable::Authority,
            StandardVariable::CallNumber => citationberg_rs::taxonomy::StandardVariable::CallNumber,
            StandardVariable::CitationKey => {
                citationberg_rs::taxonomy::StandardVariable::CitationKey
            }
            StandardVariable::CitationLabel => {
                citationberg_rs::taxonomy::StandardVariable::CitationLabel
            }
            StandardVariable::CollectionTitle => {
                citationberg_rs::taxonomy::StandardVariable::CollectionTitle
            }
            StandardVariable::ContainerTitle => {
                citationberg_rs::taxonomy::StandardVariable::ContainerTitle
            }
            StandardVariable::ContainerTitleShort => {
                citationberg_rs::taxonomy::StandardVariable::ContainerTitleShort
            }
            StandardVariable::Dimensions => citationberg_rs::taxonomy::StandardVariable::Dimensions,
            StandardVariable::Division => citationberg_rs::taxonomy::StandardVariable::Division,
            StandardVariable::DOI => citationberg_rs::taxonomy::StandardVariable::DOI,
            StandardVariable::Event => citationberg_rs::taxonomy::StandardVariable::Event,
            StandardVariable::EventTitle => citationberg_rs::taxonomy::StandardVariable::EventTitle,
            StandardVariable::EventPlace => citationberg_rs::taxonomy::StandardVariable::EventPlace,
            StandardVariable::Genre => citationberg_rs::taxonomy::StandardVariable::Genre,
            StandardVariable::ISBN => citationberg_rs::taxonomy::StandardVariable::ISBN,
            StandardVariable::ISSN => citationberg_rs::taxonomy::StandardVariable::ISSN,
            StandardVariable::Jurisdiction => {
                citationberg_rs::taxonomy::StandardVariable::Jurisdiction
            }
            StandardVariable::Keyword => citationberg_rs::taxonomy::StandardVariable::Keyword,
            StandardVariable::Language => citationberg_rs::taxonomy::StandardVariable::Language,
            StandardVariable::License => citationberg_rs::taxonomy::StandardVariable::License,
            StandardVariable::Medium => citationberg_rs::taxonomy::StandardVariable::Medium,
            StandardVariable::Note => citationberg_rs::taxonomy::StandardVariable::Note,
            StandardVariable::OriginalPublisher => {
                citationberg_rs::taxonomy::StandardVariable::OriginalPublisher
            }
            StandardVariable::OriginalPublisherPlace => {
                citationberg_rs::taxonomy::StandardVariable::OriginalPublisherPlace
            }
            StandardVariable::OriginalTitle => {
                citationberg_rs::taxonomy::StandardVariable::OriginalTitle
            }
            StandardVariable::PartTitle => citationberg_rs::taxonomy::StandardVariable::PartTitle,
            StandardVariable::PMCID => citationberg_rs::taxonomy::StandardVariable::PMCID,
            StandardVariable::PMID => citationberg_rs::taxonomy::StandardVariable::PMID,
            StandardVariable::Publisher => citationberg_rs::taxonomy::StandardVariable::Publisher,
            StandardVariable::PublisherPlace => {
                citationberg_rs::taxonomy::StandardVariable::PublisherPlace
            }
            StandardVariable::References => citationberg_rs::taxonomy::StandardVariable::References,
            StandardVariable::ReviewedGenre => {
                citationberg_rs::taxonomy::StandardVariable::ReviewedGenre
            }
            StandardVariable::ReviewedTitle => {
                citationberg_rs::taxonomy::StandardVariable::ReviewedTitle
            }
            StandardVariable::Scale => citationberg_rs::taxonomy::StandardVariable::Scale,
            StandardVariable::Source => citationberg_rs::taxonomy::StandardVariable::Source,
            StandardVariable::Status => citationberg_rs::taxonomy::StandardVariable::Status,
            StandardVariable::Title => citationberg_rs::taxonomy::StandardVariable::Title,
            StandardVariable::TitleShort => citationberg_rs::taxonomy::StandardVariable::TitleShort,
            StandardVariable::URL => citationberg_rs::taxonomy::StandardVariable::URL,
            StandardVariable::VolumeTitle => {
                citationberg_rs::taxonomy::StandardVariable::VolumeTitle
            }
            StandardVariable::YearSuffix => citationberg_rs::taxonomy::StandardVariable::YearSuffix,
        };
    }
}
