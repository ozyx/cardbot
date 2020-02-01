pub mod rest_client;
extern crate serde;

use reqwest::Error;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Deck {
    error: Option<String>,
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
impl rest_client::RestPath<DeckOfCardsActions> for Deck {
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
