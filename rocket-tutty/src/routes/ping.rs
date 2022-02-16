use rocket::*;

#[get("/ping")]
pub fn ping_fn() -> String {
    "PONG BITCH!".to_string()
}
