extern crate reqwest;
extern crate serde;

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
    AddToPile(String, String, Vec<String>),
    DrawFromDeck(Option<String>, u16),
    // TODO:
    // DrawFromPile(...),
    CreateNewDeck,
    CreatePartialDeck(Vec<String>),
    ListPile(String, String),
    ShuffleDeck(String),
    ShufflePile(String, String),
}

///
///
impl RestPath<DeckOfCardsActions> for Deck {
    fn get_path(params: DeckOfCardsActions) -> Result<String, Error> {
        use DeckOfCardsActions::*;
        match params {
            AddToPile(deck_id, pile_name, codes) => Ok(format!(
                "/api/deck/{0}/pile/{1}/add/?cards={2}",
                deck_id,
                pile_name,
                codes.join(",")
            )),
            DrawFromDeck(deck_id, count) => match deck_id {
                Some(id) => Ok(format!("/api/deck/{0}/draw/?count={1}", id, count)),
                None => Ok(format!("/api/deck/new/draw/?count={0}", count)),
            },
            CreateNewDeck => Ok("/api/deck/new/".to_string()),
            CreatePartialDeck(codes) => {
                Ok(format!("/api/deck/new/shuffle/?cards={0}", codes.join(",")))
            }
            ListPile(deck_id, pile_name) => {
                Ok(format!("/api/deck/{0}/pile/{1}/list/", deck_id, pile_name))
            }
            ShuffleDeck(deck_id) => Ok(format!("/api/deck/{0}/shuffle/", deck_id)),
            ShufflePile(deck_id, pile_name) => Ok(format!(
                "/api/deck/{0}/pile/{1}/shuffle/",
                deck_id, pile_name
            )),
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
    /// Create a new `RestClient`.
    ///
    /// ### Params
    ///
    /// `base_url`: The base URL for the given Rest API
    pub fn new(base_url: String) -> Result<RestClient, Error> {
        let client = RestClient {
            base_url: base_url,
            client: reqwest::blocking::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()?,
        };

        Ok(client)
    }

    /// Make a sychronous GET request against a given URL.
    ///
    /// The calling type must implement the `RestPath` and
    /// `serde::de::Deserialize` Traits.
    ///
    /// ### Params
    /// `params`: parameter object for the given call
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
