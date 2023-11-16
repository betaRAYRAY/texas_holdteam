use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub enum PlayerStatusEnum {
    ACTIVE,
    FOLDED,
    OUT,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Player {
    pub name: String,
    pub status: PlayerStatusEnum,
    pub stack: i32,
    pub bet: i32,
    pub cards: Option<Vec<crate::models::card::Card>>,
}

