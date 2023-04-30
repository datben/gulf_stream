use gulf_stream_lib::{
    ed25519::signature::Signature,
    pb::{node_client::NodeClient, SendTransactionRequest},
    state::transaction::TransactionMessage,
};

use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = NodeClient::connect("http://0.0.0.0:50051").await?;
    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);

    let msg = TransactionMessage::default();

    let request = tonic::Request::new(SendTransactionRequest {
        tx: Some(
            Signature::sign_payload(&keypair, 1, 5, msg)
                .try_into()
                .unwrap(),
        ),
    });

    let response = client.send_transaction(request).await?;

    println!("RESPONSE={:?}", response);

    let msg = TransactionMessage::default();

    let request = tonic::Request::new(SendTransactionRequest {
        tx: Some(
            Signature::sign_payload(&keypair, 2, 10, msg)
                .try_into()
                .unwrap(),
        ),
    });

    let response = client.send_transaction(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
