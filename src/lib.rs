#[macro_use]
extern crate serde;
extern crate reqwest;

use reqwest::Error;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug)]
pub struct Deck {
    success: bool,
    deck_id: String,
    remaining: i32,
    cards: Option<Vec<Card>>,
    shuffled: Option<bool>,
}

impl Deck {
    pub fn get_deck_id(self) -> String {
        return self.deck_id.clone();
    }
}

#[derive(Deserialize, Debug)]
pub struct Card {
    image: String,
    value: String,
    suit: String,
    code: String,
}

impl Card {
    pub fn get_image(self) -> String {
        return self.image;
    }
    pub fn get_value(self) -> String {
        return self.value;
    }
    pub fn get_suit(self) -> String {
        return self.suit;
    }
    pub fn get_code(self) -> String {
        return self.code;
    }
}

pub enum DeckOfCardsActions {
    DrawCard(Option<String>, u16),
    NewDeck,
    PartialDeck(Vec<String>),
    ShuffleDeck(String),
}

impl RestPath<DeckOfCardsActions> for Deck {
    fn get_path(params: DeckOfCardsActions) -> Result<String, Error> {
        use DeckOfCardsActions::{DrawCard, NewDeck, PartialDeck, ShuffleDeck};
        match params {
            DrawCard(deck_id, count) => match deck_id {
                Some(id) => Ok(format!("/api/deck/{0}/draw/?count={1}", id, count)),
                None => Ok(format!("/api/deck/new/draw/?count={0}", count)),
            },
            NewDeck => Ok("/api/deck/new/".to_string()),
            PartialDeck(codes) => Ok(format!("/api/deck/new/shuffle/?cards={0}", codes.join(","))),
            ShuffleDeck(deck_id) => Ok(format!("/api/deck/{0}/shuffle/", deck_id)),
        }
    }
}

//---------------------------------------------------------------------------

pub trait RestPath<U> {
    fn get_path(params: U) -> Result<String, Error>;
}

pub struct RestClient {
    base_url: String,
    client: reqwest::blocking::Client,
}

impl RestClient {
    pub fn new(base_url: String) -> Result<RestClient, Error> {
        let client = RestClient {
            base_url: base_url,
            client: reqwest::blocking::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()?,
        };

        Ok(client)
    }

    pub fn get_sync<U, T>(&mut self, params: U) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned + RestPath<U>,
    {
        let path = T::get_path(params)?;
        let res = self
            .client
            .get((self.base_url.clone() + &path).as_str())
            .send()?;
        let json = res.json()?;

        Ok(json)
    }
}
