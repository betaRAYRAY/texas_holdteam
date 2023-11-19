use serde::Deserialize;
use strum_macros::EnumIter;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy, EnumIter)]
pub enum Suit {
    HEARTS,
    SPADES,
    CLUBS,
    DIAMONDS,
}
