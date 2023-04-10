use std::vec;

use gulf_stream_lib::{
    pb::{node_client::NodeClient, SendTransactionRequest},
    state::{publick_key::PublicKey, transaction::Transaction},
};

use ed25519_dalek::{Digest, Keypair, Sha512};
use rand::rngs::OsRng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = NodeClient::connect("http://[::1]:50051").await?;
    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);

    let msg = vec![0, 1, 2, 3, 4, 5];

    let mut prehashed: Sha512 = Sha512::new();

    prehashed.update(msg.to_owned());

    let signature = keypair.sign_prehashed(prehashed, None).unwrap();

    let request = tonic::Request::new(SendTransactionRequest {
        tx: Some(
            Transaction {
                payer: PublicKey(keypair.public),
                msg,
                signature,
            }
            .into(),
        ),
    });

    let response = client.send_transaction(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
