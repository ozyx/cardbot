use cardbot::*;

fn main() {
    let mut client = RestClient::new().unwrap();
    let data: Deck = client
        .get_sync(DrawCard {
            deck_id: Option::None,
            count: 1,
        })
        .unwrap();
    println!("{:?}", data);
    let data2: Deck = client.get_sync(DrawCard {
        deck_id: Some(data.deck_id),
        count: 5
    }).unwrap();
    println!("{:?}", data2);
}
