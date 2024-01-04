#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

use citationberg_rs;
use js_sys::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

mod affixes;
mod base_language;
mod bibliography;
mod choose;
mod choose_branch;
mod choose_match;
mod choose_test;
mod citation;
mod citation_format;
mod collapse;
mod csl_macro;
mod date;
mod date_any_form;
mod date_day_form;
mod date_form;
mod date_month_form;
mod date_part;
mod date_part_name;
mod date_parts;
mod date_strong_any_form;
mod delimiter_behaviour;
mod demote_non_dropping_particle;
mod dependent_style;
mod disambiguation_rule;
mod display;
mod else_branch;
mod et_al;
mod et_al_term;
mod field;
mod font_style;
mod font_variant;
mod font_weight;
mod formatting;
mod grammar_gender;
mod group;
mod independent_style;
mod independent_style_settings;
mod info_link;
mod info_link_rel;
mod inheritable_name_options;
mod label;
mod label_pluralize;
mod layout;
mod layout_rendering_element;
mod license;
mod local_string;
mod locale;
mod locale_code;
mod locale_file;
mod locale_info;
mod locale_options;
mod localized_term;
mod long_short_form;
mod name;
mod name_and;
mod name_as_sort_order;
mod name_form;
mod name_label_position;
mod name_options;
mod name_part;
mod name_part_name;
mod names;
mod names_child;
mod number;
mod number_form;
mod ordinal_lookup;
mod ordinal_match;
mod page_range_format;
mod purge_level;
mod rendering_element;
mod second_field_alignment;
mod sort;
mod sort_direction;
mod sort_key;
mod style;
mod style_attibution;
mod style_category;
mod style_class;
mod style_info;
mod style_validation_error;
mod subsequent_author_substitute_role;
mod substitute;
mod term_form;
mod terms;
mod test_position;
mod text;
mod text_case;
mod text_decoration;
mod text_target;
mod timestamp;
mod variableless_label;
mod vertical_align;

pub use affixes::*;
pub use base_language::*;
pub use bibliography::*;
pub use choose::*;
pub use choose_branch::*;
pub use choose_match::*;
pub use choose_test::*;
pub use citation::*;
pub use citation_format::*;
pub use collapse::*;
pub use csl_macro::*;
pub use date::*;
pub use date_any_form::*;
pub use date_day_form::*;
pub use date_form::*;
pub use date_month_form::*;
pub use date_part::*;
pub use date_part_name::*;
pub use date_parts::*;
pub use date_strong_any_form::*;
pub use delimiter_behaviour::*;
pub use demote_non_dropping_particle::*;
pub use dependent_style::*;
pub use disambiguation_rule::*;
pub use display::*;
pub use else_branch::*;
pub use et_al::*;
pub use et_al_term::*;
pub use field::*;
pub use font_style::*;
pub use font_variant::*;
pub use font_weight::*;
pub use formatting::*;
pub use grammar_gender::*;
pub use group::*;
pub use independent_style::*;
pub use independent_style_settings::*;
pub use info_link::*;
pub use info_link_rel::*;
pub use inheritable_name_options::*;
pub use label::*;
pub use label_pluralize::*;
pub use layout::*;
pub use layout_rendering_element::*;
pub use license::*;
pub use local_string::*;
pub use locale::*;
pub use locale_code::*;
pub use locale_file::*;
pub use locale_info::*;
pub use locale_options::*;
pub use localized_term::*;
pub use long_short_form::*;
pub use name::*;
pub use name_and::*;
pub use name_as_sort_order::*;
pub use name_form::*;
pub use name_label_position::*;
pub use name_options::*;
pub use name_part::*;
pub use name_part_name::*;
pub use names::*;
pub use names_child::*;
pub use number::*;
pub use number_form::*;
pub use ordinal_lookup::*;
pub use ordinal_match::*;
pub use page_range_format::*;
pub use purge_level::*;
pub use rendering_element::*;
pub use second_field_alignment::*;
pub use sort::*;
pub use sort_direction::*;
pub use sort_key::*;
pub use style::*;
pub use style_attibution::*;
pub use style_category::*;
pub use style_class::*;
pub use style_info::*;
pub use style_validation_error::*;
pub use subsequent_author_substitute_role::*;
pub use substitute::*;
pub use term_form::*;
pub use terms::*;
pub use test_position::*;
pub use text::*;
pub use text_case::*;
pub use text_decoration::*;
pub use text_target::*;
pub use timestamp::*;
pub use variableless_label::*;
pub use vertical_align::*;

pub mod taxonomy;

#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}
