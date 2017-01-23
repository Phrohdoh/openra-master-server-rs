#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

pub enum GameState {
    Waiting,
    Playing,
    Finished,
    Unknown(u8),
}

impl From<u8> for GameState {
    fn from(v: u8) -> Self {
        match v {
            1 => GameState::Waiting,
            2 => GameState::Playing,
            3 => GameState::Finished,
            x => GameState::Unknown(x),
        }
    }
}

#[derive(FromForm, Debug)]
pub struct PingData<'pd> {
    pub port: u16,
    pub name: &'pd str,
    pub state: u8,
    pub players: u8,
    pub bots: u8,
    pub mods: &'pd str,
    pub map: &'pd str,
    pub maxplayers: u8,
    pub spectators: u8,
    pub protected: u8, // 1 => true, 0 => false
    pub clients: u8,
    pub new: Option<u8>, // 1 => true, 0 => false
}

#[get("/ping?<ping_data>")]
fn ping(ping_data: PingData) {
    println!("{:?}", ping_data);
}

fn main() {
    rocket::ignite().mount("/", routes![ping]).launch();
}