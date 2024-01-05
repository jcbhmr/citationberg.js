// use crate::typst_community::citationberg::citationberg::*;
// use crate::*;
// use bindings::exports::*;
// use bindings::*;
// use citationberg_rs;
// extern crate derive_more;
// use derive_more::{Add, Display, From, Into};

// #[derive(Clone, From)]
// pub struct Citation(pub(crate) citationberg_rs::Citation);
// impl GuestCitation for Citation {
//     fn new(
//         sort: Option<ToDo>,
//         layout: ToDo,
//         disambiguate_add_givenname: bool,
//         givenname_disambiguation_rule: ToDo,
//         disambiguate_add_names: bool,
//         disambiguate_add_year_suffix: bool,
//         cite_group_delimiter: Option<String>,
//         collapse: Option<ToDo>,
//         year_suffix_delimiter: Option<String>,
//         after_collapse_delimiter: Option<String>,
//         near_note_distance: u32,
//         name_options: ToDo,
//     ) -> Self {
//         citationberg_rs::Citation {
//             sort: sort.map(|x| x.into()),
//             layout: layout.into(),
//             disambiguate_add_givenname,
//             givenname_disambiguation_rule: givenname_disambiguation_rule.into(),
//             disambiguate_add_names,
//             disambiguate_add_year_suffix,
//             cite_group_delimiter,
//             collapse: collapse.map(|x| x.into()),
//             year_suffix_delimiter,
//             after_collapse_delimiter,
//             near_note_distance,
//             name_options: name_options.into(),
//         }
//         .into()
//     }
//     fn sort(&self) -> Option<ToDo> {
//         self.0.sort.map(|x| x.into())
//     }
//     fn set_sort(&self, v: Option<ToDo>) {
//         self.0.sort = v.map(|x| x.into());
//     }
//     fn layout(&self) -> ToDo {
//         self.0.layout.into()
//     }
//     fn set_layout(&self, v: ToDo) {
//         self.0.layout = v.into();
//     }
//     fn disambiguate_add_givenname(&self) -> bool {
//         self.0.disambiguate_add_givenname
//     }
//     fn set_disambiguate_add_givenname(&self, v: bool) {
//         self.0.disambiguate_add_givenname = v;
//     }
//     fn givenname_disambiguation_rule(&self) -> ToDo {
//         self.0.givenname_disambiguation_rule.into()
//     }
//     fn set_givenname_disambiguation_rule(&self, v: ToDo) {
//         self.0.givenname_disambiguation_rule = v.into();
//     }
//     fn disambiguate_add_names(&self) -> bool {
//         self.0.disambiguate_add_names
//     }
//     fn set_disambiguate_add_names(&self, v: bool) {
//         self.0.disambiguate_add_names = v;
//     }
//     fn disambiguate_add_year_suffix(&self) -> bool {
//         self.0.disambiguate_add_year_suffix
//     }
//     fn set_disambiguate_add_year_suffix(&self, v: bool) {
//         self.0.disambiguate_add_year_suffix = v;
//     }
//     fn cite_group_delimiter(&self) -> Option<String> {
//         self.0.cite_group_delimiter
//     }
//     fn set_cite_group_delimiter(&self, v: Option<String>) {
//         self.0.cite_group_delimiter = v;
//     }
//     fn collapse(&self) -> Option<ToDo> {
//         self.0.collapse.map(|x| x.into())
//     }
//     fn set_collapse(&self, v: Option<ToDo>) {
//         self.0.collapse = v.map(|x| x.into());
//     }
//     fn year_suffix_delimiter2(&self) -> Option<String> {
//         self.0.year_suffix_delimiter
//     }
//     fn set_year_suffix_delimiter(&self, v: Option<String>) {
//         self.0.year_suffix_delimiter = v;
//     }
//     fn after_collapse_delimiter2(&self) -> Option<String> {
//         self.0.after_collapse_delimiter
//     }
//     fn set_after_collapse_delimiter(&self, v: Option<String>) {
//         self.0.after_collapse_delimiter = v;
//     }
//     fn near_note_distance(&self) -> u32 {
//         self.0.near_note_distance
//     }
//     fn set_near_note_distance(&self, v: u32) {
//         self.0.near_note_distance = v;
//     }
//     fn name_options(&self) -> ToDo {
//         self.0.name_options.into()
//     }
//     fn set_name_options(&self, v: ToDo) {
//         self.0.name_options = v.into();
//     }

//     fn default_cite_group_delimiter() -> String {
//         citationberg_rs::Citation::DEFAULT_CITE_GROUP_DELIMITER.into()
//     }
//     fn with_layout(layout: ToDo) -> OwnCitation {
//         OwnCitation::new(citationberg_rs::Citation::with_layout(layout.into()).into())
//     }
//     fn get_year_suffix_delimiter(&self) -> String {
//         self.0.get_year_suffix_delimiter().into()
//     }
//     fn get_after_collapse_delimiter(&self) -> String {
//         self.0.get_after_collapse_delimiter().into()
//     }
//     fn default_near_note_distance() -> u32 {
//         citationberg_rs::Citation::default_near_note_distance()
//     }
// }
