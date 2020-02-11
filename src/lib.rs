pub mod rest_client;

extern crate serde;

use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct DeckResponse {
    error: Option<String>,
    success: bool,
    deck_id: String,
    remaining: Option<u16>,
    cards: Option<Vec<Card>>,
    shuffled: Option<bool>,
    piles: Option<HashMap<String, Pile>>,
}

#[derive(Deserialize, Debug)]
pub struct Pile {
    remaining: Option<u16>,
}

impl DeckResponse {
    pub fn get_deck_id(&self) -> String {
        return self.deck_id.clone();
    }
    pub fn get_cards(&self) -> Vec<Card> {
        match &self.cards {
            Some(drawn_cards) => drawn_cards.to_vec(),
            None => vec![],
        }
    }
}

pub fn to_code_str(cards: Vec<Card>) -> String {
    let mut codes: Vec<String> = vec![];
    for card in cards {
        codes.push(card.get_code().to_string());
    }
    codes.join(",")
}

#[derive(Clone, Debug, Deserialize)]
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

pub enum ApiActions<'a> {
    AddToPile(String, &'a str, String),
    DrawFromDeck(Option<String>, u16),
    // TODO:
    DrawFromPile(String, &'a str, Option<&'a str>, Option<u16>),
    CreateNewDeck,
    CreatePartialDeck(Vec<&'a str>),
    ListPile(String, String),
    ShuffleDeck(String),
    ShufflePile(String, String),
}

/// Construct the proper REST path given the type
/// of call
impl rest_client::RestPath<ApiActions<'_>> for DeckResponse {
    fn get_path(params: ApiActions) -> Result<String, Error> {
        use ApiActions::*;
        match params {
            AddToPile(deck_id, pile_name, codes) => Ok(format!(
                "/api/deck/{0}/pile/{1}/add/?cards={2}",
                deck_id, pile_name, codes
            )),
            DrawFromDeck(deck_id, count) => match deck_id {
                Some(id) => Ok(format!("/api/deck/{0}/draw/?count={1}", id, count)),
                None => Ok(format!("/api/deck/new/draw/?count={0}", count)),
            },
            DrawFromPile(deck_id, pile_name, codes, count) => match codes {
                Some(code_str) => Ok(format!(
                    "/api/deck/{0}/pile/{1}/draw/?cards={2}",
                    deck_id, pile_name, code_str
                )),
                None => match count {
                    Some(cards_num) => Ok(format!(
                        "/api/deck/{0}/pile/{1}/draw/?count={2}",
                        deck_id, pile_name, cards_num
                    )),
                    None => panic!("need either code str or count to draw from pile"),
                },
            },
            CreateNewDeck => Ok(format!("/api/deck/new/")),
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
