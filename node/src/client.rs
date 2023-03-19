use gulf_stream_lib::{
    pb::{node_client::NodeClient, SendBlock},
    state::block::Block,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = NodeClient::connect("http://[::1]:50051").await?;

    let block1 = Block {
        index: 1,
        previous_blockhash: Block::genesis().blockhash,
        ..Default::default()
    };

    let block2 = Block {
        index: 1,
        previous_blockhash: block1.blockhash.clone(),
        ..Default::default()
    };

    let request = tonic::Request::new(SendBlock {
        block: Some(block1.into()),
    });

    let response = client.block_handler(request).await?;

    println!("RESPONSE={:?}", response);

    let request = tonic::Request::new(SendBlock {
        block: Some(block2.into()),
    });

    let response = client.block_handler(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
