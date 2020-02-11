use cardbot::*;

fn main() {
    let mut client = rest_client::RestClient::new("https://deckofcardsapi.com".to_string())
        .expect("Something went egregiously wrong?");

    let mut data: DeckResponse = client
        .get_sync(ApiActions::DrawFromDeck(Option::None, 1))
        .unwrap();

    println!("{:#?}", data);

    data = client
        .get_sync(ApiActions::ShuffleDeck(data.get_deck_id()))
        .unwrap();

    println!("{:#?}", data);

    data = client
        .get_sync(ApiActions::DrawFromDeck(Some(data.get_deck_id()), 5))
        .unwrap();

    println!("{:#?}", data);

    data = client
        .get_sync(ApiActions::CreatePartialDeck(vec!["KD", "KS", "KC", "KH"]))
        .unwrap();
    println!("{:#?}", data);

    data = client
        .get_sync(ApiActions::DrawFromDeck(Some(data.get_deck_id()), 2))
        .unwrap();

    let cards = data.get_cards().clone();
    data = client
        .get_sync(ApiActions::AddToPile(
            data.get_deck_id(),
            "jessepile",
            to_code_str(cards),
        ))
        .unwrap();
    println!("{:#?}", data);

    data = client
        .get_sync(ApiActions::DrawFromPile(
            data.get_deck_id(),
            "jessepile",
            None,
            Some(1),
        ))
        .expect("hmmm");
    println!("{:#?}", data);
}
