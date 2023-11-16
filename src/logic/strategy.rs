use rocket::serde::json::Json;

pub fn decide(_table: Json<crate::models::table::Table>) -> crate::models::bet::Bet {
    // TODO: Add Poker Logic Here... :)

    return crate::models::bet::Bet{bet: _table.minimum_bet}
 }
