use gulf_stream_lib::{
    ed25519::publickey::PublicKey,
    pb::{node_client::NodeClient, GetBalanceRequest, GetHistoryRequest},
    state::transaction::Transaction,
    utils::serde::BytesSerialize,
};
use rocket::{get, launch, routes};

#[get("/")]
fn index() -> &'static str {
    "GULF STREAM EXPLORER"
}

#[get("/history")]
async fn history() -> String {
    let mut client = NodeClient::connect("http://[::1]:50051").await.unwrap();

    let request = tonic::Request::new(GetHistoryRequest {});

    let response = client.get_history(request).await.unwrap();

    let response = response
        .into_inner()
        .transactions
        .into_iter()
        .map(|tx| {
            let tx: Transaction = tx.try_into().unwrap();
            tx.format()
        })
        .collect::<Vec<String>>();

    format!("{:?}", response)
}

#[get("/balance/<address>")]
async fn get_balance(address: String) -> String {
    let mut client = NodeClient::connect("http://[::1]:50051").await.unwrap();

    let request = tonic::Request::new(GetBalanceRequest {
        address: PublicKey::try_from_str(&address).unwrap().serialize(),
    });

    let response = client.get_balance(request).await.unwrap();

    format!("{:?}", response.into_inner().balance)
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![index, history, get_balance])
}
