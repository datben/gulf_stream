use crate::pb::node_server::Node;
use crate::pb::{SendBlockRequest, SendBlockResponse};
use crate::state::block::Block;
use crate::state::blockchain::Blockchain;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

pub struct Ledger {
    pub state: Mutex<Blockchain>,
}

#[tonic::async_trait]
impl Node for Ledger {
    async fn send_block(
        &self,
        request: Request<SendBlockRequest>,
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
