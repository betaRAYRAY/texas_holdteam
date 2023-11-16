use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub enum Suit {
    HEARTS,
    SPADES,
    CLUBS,
    DIAMONDS,
}

