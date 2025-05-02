#[tokio::main]
async fn main() {
    let client = parkright::Client::from_env();

    let id = std::env::args()
        .nth(1)
        .expect("first argument should be saettlementId")
        .parse::<u64>()
        .expect("first argument settlementId must be a u64");

    let tickets = client.fetch_tickets(&[id]).await.expect("fetching");

    println!("{} tickets in total", tickets.len());
}
