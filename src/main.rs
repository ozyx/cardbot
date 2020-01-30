use cardbot::*;

fn main() {
   let mut client = RestClient::new().unwrap();
   let data: DrawCard = client.get_sync((Option::None, 1)).unwrap();
   
}