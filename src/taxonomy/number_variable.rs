use crate::typst_community::citationberg::citationberg;
use crate::typst_community::citationberg::citationberg_taxonomy::*;
use citationberg_rs;
use citationberg_rs::taxonomy as taxonomy_rs;

impl NumberVariable {
    pub fn is_number_of_variable(&self) -> bool {}
}

impl From<NumberVariable> for taxonomy_rs::NumberVariable {
    fn from(value: NumberVariable) -> Self {
        match value {
            NumberVariable::ChapterNumber => taxonomy_rs::NumberVariable::ChapterNumber,
            NumberVariable::CitationNumber => taxonomy_rs::NumberVariable::CitationNumber,
            NumberVariable::CollectionNumber => taxonomy_rs::NumberVariable::CollectionNumber,
            NumberVariable::Edition => taxonomy_rs::NumberVariable::Edition,
            NumberVariable::FirstReferenceNoteNumber => {
                taxonomy_rs::NumberVariable::FirstReferenceNoteNumber
            }
            NumberVariable::Issue => taxonomy_rs::NumberVariable::Issue,
            NumberVariable::Locator => taxonomy_rs::NumberVariable::Locator,
            NumberVariable::Number => taxonomy_rs::NumberVariable::Number,
            NumberVariable::NumberOfPages => taxonomy_rs::NumberVariable::NumberOfPages,
            NumberVariable::NumberOfVolumes => taxonomy_rs::NumberVariable::NumberOfVolumes,
            NumberVariable::Page => taxonomy_rs::NumberVariable::Page,
            NumberVariable::PageFirst => taxonomy_rs::NumberVariable::PageFirst,
            NumberVariable::PartNumber => taxonomy_rs::NumberVariable::PartNumber,
            NumberVariable::PrintingNumber => taxonomy_rs::NumberVariable::PrintingNumber,
            NumberVariable::Section => taxonomy_rs::NumberVariable::Section,
            NumberVariable::SupplementNumber => taxonomy_rs::NumberVariable::SupplementNumber,
            NumberVariable::Version => taxonomy_rs::NumberVariable::Version,
            NumberVariable::Volume => taxonomy_rs::NumberVariable::Volume,
        }
    }
}
impl From<taxonomy_rs::NumberVariable> for NumberVariable {
    fn from(value: taxonomy_rs::NumberVariable) -> Self {
        match value {
            taxonomy_rs::NumberVariable::ChapterNumber => NumberVariable::ChapterNumber,
            taxonomy_rs::NumberVariable::CitationNumber => NumberVariable::CitationNumber,
            taxonomy_rs::NumberVariable::CollectionNumber => NumberVariable::CollectionNumber,
            taxonomy_rs::NumberVariable::Edition => NumberVariable::Edition,
            taxonomy_rs::NumberVariable::FirstReferenceNoteNumber => {
                NumberVariable::FirstReferenceNoteNumber
            }
            taxonomy_rs::NumberVariable::Issue => NumberVariable::Issue,
            taxonomy_rs::NumberVariable::Locator => NumberVariable::Locator,
            taxonomy_rs::NumberVariable::Number => NumberVariable::Number,
            taxonomy_rs::NumberVariable::NumberOfPages => NumberVariable::NumberOfPages,
            taxonomy_rs::NumberVariable::NumberOfVolumes => NumberVariable::NumberOfVolumes,
            taxonomy_rs::NumberVariable::Page => NumberVariable::Page,
            taxonomy_rs::NumberVariable::PageFirst => NumberVariable::PageFirst,
            taxonomy_rs::NumberVariable::PartNumber => NumberVariable::PartNumber,
            taxonomy_rs::NumberVariable::PrintingNumber => NumberVariable::PrintingNumber,
            taxonomy_rs::NumberVariable::Section => NumberVariable::Section,
            taxonomy_rs::NumberVariable::SupplementNumber => NumberVariable::SupplementNumber,
            taxonomy_rs::NumberVariable::Version => NumberVariable::Version,
            taxonomy_rs::NumberVariable::Volume => NumberVariable::Volume,
        }
    }
}
