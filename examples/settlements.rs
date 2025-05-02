#[tokio::main]
async fn main() {
    let client = parkright::Client::from_env();

    let settlements = client.fetch_settlements().await.expect("fetching");

    println!("Fetched {} settlements", settlements.len());

    for settlement in settlements {
        println!("Fetching settlement {}", settlement.id);

        client
            .fetch_tickets(&[settlement.id])
            .await
            .expect("fetchign stuff fs");
    }
}
