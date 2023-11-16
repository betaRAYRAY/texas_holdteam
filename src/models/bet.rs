use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Bet {
    pub bet: i32
}