#[macro_use]
extern crate rocket;
use iota::Client;

#[get("/<address>")]
async fn check(address: String) -> String {
    let iota = Client::build() // Crate a client instance builder
        .with_node("https://api.lb-0.testnet.chrysalis2.com") // chrysalis2 pubblic testnet node
        .unwrap()
        .finish()
        .unwrap();

    let balance = iota
        .get_address()
        .balance(&address.clone().into())
        .await
        .unwrap();

    format!("Address: {}, Balance: {}", address, balance)
}

#[launch]
fn rocket() -> _ {

    rocket::ignite().mount("/", routes![check])

}