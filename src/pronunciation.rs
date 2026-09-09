use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum PronunciationSource {
    GoldDictionary,
    SilverDictionary,
    Stemming,
    RuleBased,
    Fallback,
    Character,
    Unknown
}
