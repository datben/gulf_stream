use gulf_stream_lib::{
    pb::{node_client::NodeClient, SendBlockRequest},
    state::block::Block,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = NodeClient::connect("http://[::1]:50051").await?;

    let genesis = Block::genesis();

    let block1 = Block::create_block(1, &genesis.blockhash, 0);
    let block2 = Block::create_block(2, &block1.blockhash, 0);

    let request = tonic::Request::new(SendBlockRequest {
        block: Some(block1.into()),
    });

    let response = client.send_block(request).await?;

    println!("RESPONSE={:?}", response);

    let request = tonic::Request::new(SendBlockRequest {
        block: Some(block2.into()),
    });

    let response = client.send_block(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
