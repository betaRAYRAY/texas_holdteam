use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub enum Rank {
    #[serde(alias = "A")]
    A,
    #[serde(alias = "K")]
    K,
    #[serde(alias = "Q")]
    Q,
    #[serde(alias = "J")]
    J,
    #[serde(alias = "10")]
    _10,
    #[serde(alias = "9")]
    _9,
    #[serde(alias = "8")]
    _8,
    #[serde(alias = "7")]
    _7,
    #[serde(alias = "6")]
    _6,
    #[serde(alias = "5")]
    _5,
    #[serde(alias = "4")]
    _4,
    #[serde(alias = "3")]
    _3,
    #[serde(alias = "2")]
    _2,
}

