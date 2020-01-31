use cardbot::*;

fn main() {
    let mut client = rest_client::RestClient::new("https://deckofcardsapi.com".to_string())
        .expect("Something went egregiously wrong?");

    let mut data: Deck = client
        .get_sync(DeckOfCardsActions::DrawFromDeck(Option::None, 1))
        .unwrap();

    println!("{:#?}", data);

    data = client
        .get_sync(DeckOfCardsActions::ShuffleDeck(data.get_deck_id()))
        .unwrap();

    println!("{:#?}", data);

    data = client
        .get_sync(DeckOfCardsActions::DrawFromDeck(
            Some(data.get_deck_id()),
            5,
        ))
        .unwrap();

    println!("{:#?}", data);
}
