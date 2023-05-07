use gulf_stream_lib::{
    ed25519::{publickey::PublicKey, signature::Signature},
    pb::{node_client::NodeClient, SendTransactionRequest},
    state::transaction::TransactionMessage,
};

use anyhow::Result;
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

#[tokio::main]
async fn main() -> Result<()> {
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

    let keypair2: Keypair = Keypair::generate(&mut csprng);

    let msg = TransactionMessage::Transfer {
        to: PublicKey(keypair2.public),
        amount: 17,
    };

    let request = tonic::Request::new(SendTransactionRequest {
        tx: Some(
            Signature::sign_payload(&keypair, 3, 2, msg)
                .try_into()
                .unwrap(),
        ),
    });

    let response = client.send_transaction(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
