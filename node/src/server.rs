use gulf_stream_lib::pb::node_server::{Node, NodeServer};
use gulf_stream_lib::pb::{SendBlock, SendBlockResponse};
use gulf_stream_lib::state::block::Block;
use gulf_stream_lib::state::blockchain::Blockchain;
use tokio::sync::Mutex;
use tonic::{transport::Server, Request, Response, Status};

pub struct GulfStreamNode {
    pub state: Mutex<Blockchain>,
}

#[tonic::async_trait]
impl Node for GulfStreamNode {
    async fn block_handler(
        &self,
        request: Request<SendBlock>,
    ) -> Result<Response<SendBlockResponse>, Status> {
        let block: Block = request.into_inner().block.unwrap().into();

        if let Err(err) = self.state.lock().await.try_insert(&block) {
            return Err(err.into());
        }

        let reply = SendBlockResponse {
            message: format!("Block {} inserted", block.blockhash),
        };

        return Ok(Response::new(reply));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let node = GulfStreamNode {
        state: Mutex::new(Blockchain::default()),
    };

    Server::builder()
        .add_service(NodeServer::new(node))
        .serve(addr)
        .await?;

    Ok(())
}
