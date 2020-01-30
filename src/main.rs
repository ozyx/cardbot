use cardbot::*;

// pub async fn new_game(num_decks: i16) -> Result<String, Error> {
//     let request_url = format!("https://deckofcardsapi.com/api/deck/new/shuffle/?deck_count={num_decks}",
//                               num_decks = num_decks.to_string());
//     println!("{}", request_url);
//     let response = reqwest::get(&request_url).await?;
//     let deck_id: Deck = response.json::<Deck>().await?;
//     print!("{}", deck_id.deck_id);
//     Ok(deck_id.deck_id)
// }

fn main() {
    // let deck_id: String = Client::new_deck().unwrap();
    // println!("{}", deck_id);
}