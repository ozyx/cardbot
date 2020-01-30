#[macro_use]
extern crate serde;
extern crate reqwest;

use reqwest::Error;
use serde::Deserialize;
use std::time::Duration;

// TODO: This probably shouldn't be entirely public
#[derive(Deserialize, Debug)]
pub struct Deck {
    pub success: bool,
    pub deck_id: String,
    pub remaining: i32,
    pub cards: Vec<Card>
}

#[derive(Deserialize, Debug)]
pub struct Card {
    image: String,
    value: String,
    suit: String,
    code: String,
}

#[derive(Deserialize, Debug)]
pub struct DrawCard {
    pub deck_id: Option<String>,
    pub count: u16,
}

impl RestPath<DrawCard> for Deck {
    fn get_path(params: DrawCard) -> Result<String, Error> {

        match params.deck_id {
            Some(id) => Ok(format!(
                "https://deckofcardsapi.com/api/deck/{0}/draw/?count={1}",
                id, params.count
            )),
            None => Ok(format!(
                "https://deckofcardsapi.com/api/deck/new/draw/?count={0}",
                params.count
            )),
        }
    }
}

pub trait CardApi {
    fn base_url() -> &'static str {
        "https://deckofcardsapi.com"
    }
}

pub trait RestPath<U> {
    fn get_path(params: U) -> Result<String, Error>;
}

pub struct RestClient {
    client: reqwest::blocking::Client,
}

impl RestClient {
    pub fn new() -> Result<RestClient, Error> {
        Ok(RestClient {
            client: reqwest::blocking::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()?,
        })
    }

    pub fn get_sync<U, T>(&mut self, params: U) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned + RestPath<U>,
    {
        let res = self.client.get(T::get_path(params)?.as_str()).send()?;
        let json = res.json()?;
        Ok(json)
    }
}
