use gulf_stream_lib::{
    pb::{node_client::NodeClient, SendTransactionRequest},
    state::transaction::{Transaction, TransactionMessage},
    utils::serde::BytesSerialize,
};

use ed25519_dalek::{Digest, Keypair, Sha512};
use rand::rngs::OsRng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = NodeClient::connect("http://[::1]:50051").await?;
    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);

    let msg = TransactionMessage::default();

    let request = tonic::Request::new(SendTransactionRequest {
        tx: Some(sign_tx(&keypair, msg, 1, 5).try_into().unwrap()),
    });

    let response = client.send_transaction(request).await?;

    println!("RESPONSE={:?}", response);

    let msg = TransactionMessage::default();

    let request = tonic::Request::new(SendTransactionRequest {
        tx: Some(sign_tx(&keypair, msg, 2, 12546654154).try_into().unwrap()),
    });

    let response = client.send_transaction(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

pub fn sign_tx(
    keypair: &Keypair,
    msg: TransactionMessage,
    blockheight: u64,
    gas: u64,
) -> Transaction {
    let mut prehashed: Sha512 = Sha512::new();

    prehashed.update(msg.serialize());

    let signature = keypair.sign_prehashed(prehashed, None).unwrap();

    Transaction {
        blockheight,
        payer: keypair.public.into(),
        msg,
        signature: signature.into(),
        gas,
    }
}
