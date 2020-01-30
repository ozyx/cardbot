#[macro_use]
extern crate serde;
extern crate reqwest;
extern crate serde_derive;
use reqwest::Error;

#[derive(Deserialize, Debug)]
pub struct Deck {
    deck_id: String,
    remaining: i32,
    shuffled: bool
}

#[derive(Deserialize, Debug)]
pub struct Card {
    image: String,
    value: String,
    suit: String,
    code: String
}

pub struct RestClient {
    base_url: String,
    deck: Deck
}

impl RestClient {

    pub fn new() {
        let base_url = "https://deckofcardsapi.com/api/deck/";
    }

    pub fn get_sync<T>(&mut self, url: String) -> Result<T, Error> where T: serde::de::DeserializeOwned {
        let res = reqwest::blocking::get(&url)?;
        let json = res.json()?;
        Ok(json)
    }

    pub fn post_sync(&mut self, url: String, body: String) -> Result<(), Error> {
        let client = reqwest::blocking::Client::new();
        let _res = client.post(&url).body(body).send()?;
        Ok(())
    }

    // pub fn new_deck() -> Result<String, Error> {
    //     let res = reqwest::blocking::get("https://deckofcardsapi.com/api/deck/new/shuffle/?deck_count=1")?;
    //     let deck: Deck = res.json()?;
    
    //     Ok(deck.deck_id)
    // }
}



pub struct CardGame {
    game_type: CardGameType,
    players: Vec<String>,
    leader: String,
    config: Config,
    deck_id: String
}

impl CardGame {
    pub fn run() {

    }

    fn draw() { 

    }
}

enum CardGameType {
    Blackjack,
    Poker
}

struct Config {
    num_decks: i16
}

impl Config {
    pub fn new(game_type: CardGameType) -> Result<Config, &'static str> {
        
        let decks;

        match game_type {
            CardGameType::Blackjack => decks = 6,
            CardGameType::Poker     => decks = 1
        }

        Ok(Config { num_decks: decks })
    }
}

