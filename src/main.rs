#[macro_use] extern crate rocket;
use rocket::serde::json::Json;

mod models;
mod logic;

#[post("/", format = "json", data = "<table>")]
fn index(table: Json<models::table::Table>) -> Json<models::bet::Bet> {
    Json(logic::strategy::decide(table))
}

#[launch]
fn rocket() -> _ {
    //logic::strategy::testSimulateWinProbability();

    rocket::build().mount("/", routes![index])
}