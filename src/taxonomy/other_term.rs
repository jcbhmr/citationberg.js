use citationberg;
use derive_more::{From, Into};
use duplicate::duplicate_item;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, From, Into)]
pub struct OtherTerm(#[wasm_bindgen(skip)] pub citationberg::taxonomy::OtherTerm);
#[duplicate_item(
    variant;
    [Month01];
    [Month02];
    [Month03];
    [Month04];
    [Month05];
    [Month06];
    [Month07];
    [Month08];
    [Month09];
    [Month10];
    [Month11];
    [Month12];
    [Ordinal];
    // [OrdinalN(u8)];
    // [LongOrdinal(u8)];
    [OpenQuote];
    [CloseQuote];
    [OpenInnerQuote];
    [CloseInnerQuote];
    [PageRangeDelimiter];
    [Colon];
    [Comma];
    [Semicolon];
    [Season01];
    [Season02];
    [Season03];
    [Season04];
    [Anthropology];
    [Astronomy];
    [Biology];
    [Botany];
    [Chemistry];
    [Engineering];
    [GenericBase];
    [Geography];
    [Geology];
    [History];
    [Humanities];
    [Literature];
    [Math];
    [Medicine];
    [Philosophy];
    [Physics];
    [Psychology];
    [Sociology];
    [Science];
    [PoliticalScience];
    [SocialScience];
    [Theology];
    [Zoology];
    [Accessed];
    [Ad];
    [AdvanceOnlinePublication];
    [Album];
    [And];
    [AndOthers];
    [Anonymous];
    [At];
    [AudioRecording];
    [AvailableAt];
    [Bc];
    [Bce];
    [By];
    [Ce];
    [Circa];
    [Cited];
    [EtAl];
    [Film];
    [Forthcoming];
    [From];
    [Henceforth];
    [Ibid];
    [In];
    [InPress];
    [Internet];
    [Interview];
    [Letter];
    [LocCit];
    [NoDate];
    [NoPlace];
    [NoPublisher];
    [On];
    [Online];
    [OpCit];
    [OriginalWorkPublished];
    [PersonalCommunication];
    [Podcast];
    [PodcastEpisode];
    [Preprint];
    [PresentedAt];
    [RadioBroadcast];
    [RadioSeries];
    [RadioSeriesEpisode];
    [Reference];
    [Retrieved];
    [ReviewOf];
    [Scale];
    [SpecialIssue];
    [SpecialSection];
    [TelevisionBroadcast];
    [TelevisionSeries];
    [TelevisionSeriesEpisode];
    [Video];
    [WorkingPaper];
)]
#[wasm_bindgen]
impl OtherTerm {
    #[allow(non_snake_case)]
    pub fn variant() -> Self {
        citationberg::taxonomy::OtherTerm::variant.into()
    }
}
#[wasm_bindgen]
impl OtherTerm {
    #[allow(non_snake_case)]
    pub fn OrdinalN(v: u8) -> Self {
        citationberg::taxonomy::OtherTerm::OrdinalN(v).into()
    }
    #[allow(non_snake_case)]
    pub fn LongOrdinal(v: u8) -> Self {
        citationberg::taxonomy::OtherTerm::LongOrdinal(v).into()
    }
}
#[wasm_bindgen]
impl OtherTerm {
    #[wasm_bindgen(getter)]
    pub fn tag(&self) -> String {
        match &self.0 {
            citationberg::taxonomy::OtherTerm::OrdinalN(_) => "OrdinalN".to_string(),
            citationberg::taxonomy::OtherTerm::LongOrdinal(_) => "LongOrdinal".to_string(),
            // HACK: Use 'Debug' trait to get the tag.
            _ => format!("{:?}", &self.0),
        }
    }
    #[wasm_bindgen(getter)]
    pub fn val(&self) -> JsValue {
        match &self.0 {
            citationberg::taxonomy::OtherTerm::OrdinalN(v) => (*v).into(),
            citationberg::taxonomy::OtherTerm::LongOrdinal(v) => (*v).into(),
            _ => JsValue::UNDEFINED,
        }
    }
}
#[wasm_bindgen]
impl OtherTerm {
    pub fn is_n_ordinal(&self) -> bool {
        self.0.clone().is_n_ordinal()
    }
    pub fn is_ordinal(&self) -> bool {
        self.0.clone().is_ordinal()
    }
    pub fn month(i: u8) -> Option<OtherTerm> {
        citationberg::taxonomy::OtherTerm::month(i).map(|x| x.into())
    }
    pub fn season(i: u8) -> Option<OtherTerm> {
        citationberg::taxonomy::OtherTerm::season(i).map(|x| x.into())
    }
}
