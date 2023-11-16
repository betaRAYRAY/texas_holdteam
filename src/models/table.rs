use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Table {
    #[serde(alias = "communityCards")]
    pub community_cards: Vec<crate::models::card::Card>,
    pub players: Vec<crate::models::player::Player>,
    pub round: i32,
    #[serde(alias = "smallBlind")]
    pub small_blind: i32,
    #[serde(alias = "minimumBet")]
    pub minimum_bet: i32,
    #[serde(alias = "minimumRaise")]
    pub minimum_raise: i32,
    pub pot: i32,
    #[serde(alias = "activePlayer")]
    pub active_player: i32,
    #[serde(alias = "currentDealer")]
    pub current_dealer: i32,
}

