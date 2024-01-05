use crate::typst_community::citationberg::citationberg::*;
use crate::*;
use bindings::exports::*;
use bindings::*;
use citationberg_rs;
use std::cell::*;

pub struct Affixes(RefCell<citationberg_rs::Affixes>);
impl GuestAffixes for Affixes {
    fn new(prefix: Option<String>, suffix: Option<String>) -> Self {
        let inner = citationberg_rs::Affixes { prefix, suffix };
        Affixes(RefCell::new(inner))
    }

    fn prefix(&self) -> Option<String> {
        let inner = self.0.borrow();
        inner.prefix.clone()
    }
    fn set_prefix(&self, v: Option<String>) {
        let mut inner = self.0.borrow_mut();
        inner.prefix = v;
    }
    fn suffix(&self) -> Option<String> {
        let inner = self.0.borrow();
        inner.suffix.clone()
    }
    fn set_suffix(&self, v: Option<String>) {
        let mut inner = self.0.borrow_mut();
        inner.suffix = v;
    }

    // fn clone(&self) -> OwnAffixes {
    //     let inner = self.0.borrow();
    //     let inner_clone = inner.clone();
    //     let wrapper = Affixes(RefCell::new(inner_clone));
    //     OwnAffixes::new(wrapper)
    // }
    // fn clone_from(&self, source: &Affixes) {
    //     let mut inner = self.0.borrow_mut();
    //     let source_inner = source.0.borrow();
    //     inner.clone_from(&source_inner);
    // }

    // fn default() -> OwnAffixes {
    //     let inner = citationberg_rs::Affixes::default();
    //     let wrapper = Affixes(RefCell::new(inner));
    //     OwnAffixes::new(wrapper)
    // }

    // fn eq(&self, other: &Affixes) -> bool {
    //     let inner = self.0.borrow();
    //     let other_inner = other.0.borrow();
    //     inner.eq(&other_inner)
    // }
    // fn ne(&self, other: &Affixes) -> bool {
    //     let inner = self.0.borrow();
    //     let other_inner = other.0.borrow();
    //     inner.ne(&other_inner)
    // }
}
