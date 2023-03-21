use crate::pb::node_client::NodeClient;
use crate::pb::node_server::Node;
use crate::pb::{SendBlockRequest, SendBlockResponse};
use crate::state::block::Block;
use crate::state::blockchain::Blockchain;
use tokio::sync::Mutex;
use tonic::transport::Endpoint;
use tonic::{Request, Response, Status};

pub struct Ledger {
    pub state: Mutex<Blockchain>,
    pub other_nodes: Mutex<Vec<Endpoint>>,
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

#[tonic::async_trait]
impl Broadcaster for Ledger {
    async fn broadcast(&self, request: SendBlockRequest) -> Result<(), Vec<Endpoint>> {
        let nodes = self.other_nodes.lock().await.to_owned();
        let mut failed_request = vec![];
        for endpoint in nodes.iter() {
            if let Ok(mut client) = NodeClient::connect(endpoint.to_owned()).await {
                if let Err(_) = client
                    .send_block(tonic::Request::new(request.to_owned()))
                    .await
                {
                    failed_request.push(endpoint.to_owned())
                }
            }
        }
        if failed_request.is_empty() {
            Ok(())
        } else {
            Err(failed_request)
        }
    }
}

#[tonic::async_trait]
trait Broadcaster {
    async fn broadcast(&self, request: SendBlockRequest) -> Result<(), Vec<Endpoint>>;
}
