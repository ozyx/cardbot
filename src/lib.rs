#[macro_use]
extern crate serde;
extern crate reqwest;
extern crate serde_derive;
use reqwest::Error;

#[derive(Deserialize, Debug)]
pub struct Deck {
    deck_id: String,
    remaining: i32,
    shuffled: bool,
}

#[derive(Deserialize, Debug)]
pub struct Card {
    image: String,
    value: String,
    suit: String,
    code: String,
}

pub struct DrawCard {
    deck_id: Option<String>,
    count: u16,
}

impl RestPath<(Option<String>, u16)> for DrawCard {
    fn get_path(params: (Option<String>, u16)) -> Result<String, Error> {
        let deck_id = params.0;
        let count = params.1;

        match deck_id {
            Some(id) => Ok(format!(
                "https://deckofcardsapi.com/api/deck/{0}/draw/?count={1}",
                id, count
            )),
            None => Ok(format!(
                "https://deckofcardsapi.com/api/deck/new/draw/?count={0}",
                count
            )),
        }
    }
}

pub trait RestPath<U> {
    fn get_path(params: U) -> Result<String, Error>;
}

pub struct RestClient {}

impl RestClient {
    pub fn get_sync<U, T>(&mut self, params: U) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned + RestPath<U>,
    {
        let res = reqwest::blocking::get(T::get_path(params)?.as_str())?;
        let json = res.json()?;
        Ok(json)
    }

    // // TODO:
    // pub fn post_sync(&mut self, url: String, body: String) -> Result<(), Error> {
    //     let client = reqwest::blocking::Client::new();
    //     let _res = client.post(&url).body(body).send()?;
    //     Ok(())
    // }
}
