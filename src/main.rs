use cardbot::*;

fn main() {
    let mut client = RestClient::new("https://deckofcardsapi.com".to_string())
        .expect("Something went egregiously wrong?");

    let mut data: Deck = client
        .get_sync(DeckOfCardsActions::DrawCard(Option::None, 1))
        .unwrap();

    println!("{:#?}", data);

    data = client
        .get_sync(DeckOfCardsActions::ShuffleDeck(data.get_deck_id()))
        .unwrap();

    println!("{:#?}", data);

    data = client
        .get_sync(DeckOfCardsActions::DrawCard(Some(data.get_deck_id()), 5))
        .unwrap();

    println!("{:#?}", data);
}
