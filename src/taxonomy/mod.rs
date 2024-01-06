use crate::typst_community::citationberg::citationberg;
use crate::typst_community::citationberg::citationberg_taxonomy::*;
use citationberg_rs;
use citationberg_rs::taxonomy as taxonomy_rs;

mod number_variable;
pub use number_variable::*;

pub struct Component;
impl Guest for Component {
    fn number_variable_is_number_of_variable(s: NumberVariable) -> bool {
        s.is_number_of_variable()
    }

    fn other_term_is_n_ordinal(s: OtherTerm) -> bool {
        s.is_n_ordinal()
    }
    fn other_term_is_ordinal(s: OtherTerm) -> bool {
        s.is_ordinal()
    }
    fn other_term_month(i: u8) -> Option<OtherTerm> {
        OtherTerm::month(i)
    }
    fn other_term_season(i: u8) -> Option<OtherTerm> {
        OtherTerm::season(i)
    }

    fn term_term_fallback(s: Term) -> Term {
        s.term_fallback()
    }

    fn term_is_ordinal(s: Term) -> bool {
        s.is_ordinal()
    }
    fn term_is_n_ordinal(s: Term) -> bool {
        s.is_n_ordinal()
    }
    fn term_is_gendered(s: Term) -> bool {
        s.is_gendered()
    }
    fn term_is_lexically_same(s: Term, other: Term) -> bool {
        s.is_lexically_same(other)
    }

    fn variable_is_number_of_variable(s: Variable) -> bool {
        s.is_number_of_variable()
    }
}
