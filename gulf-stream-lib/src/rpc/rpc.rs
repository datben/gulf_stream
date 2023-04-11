use std::sync::Arc;

use crate::err::GulfStreamError;
use crate::ledger::ledger::Ledger;
use crate::pb::node_client::NodeClient;
use crate::pb::node_server::Node;
use crate::pb::{GenericResponse, SendBlockRequest, SendTransactionRequest};
use crate::state::block::Block;
use crate::state::transaction::Transaction;
use tonic::transport::Endpoint;
use tonic::{Request, Response, Status};

pub struct GulfStreamRpc {
    pub ledger: Arc<Ledger>,
}

#[tonic::async_trait]
impl Node for GulfStreamRpc {
    async fn send_block(
        &self,
        request: Request<SendBlockRequest>,
    ) -> Result<Response<GenericResponse>, Status> {
        let block: Block = request.into_inner().block.unwrap().try_into().unwrap();

        if let Err(err) = self.ledger.state.lock().await.try_insert(&block) {
            return Err(err.into());
        }

        let reply = GenericResponse {
            message: format!("Block {} inserted", block.blockhash),
        };

        return Ok(Response::new(reply));
    }

    async fn send_transaction(
        &self,
        request: Request<SendTransactionRequest>,
    ) -> Result<Response<GenericResponse>, Status> {
        let tx: Transaction = request.into_inner().tx.unwrap().try_into().unwrap();

        if !tx.is_valid().map_err(Into::<Status>::into)? {
            return Err(GulfStreamError::TxIsNotValid.into());
        }

        let reply = GenericResponse {
            message: format!("Tx {:?} inserted", tx.signature),
        };

        self.ledger.mem_pool.lock().await.push(tx);

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
pub trait Broadcaster {
    async fn broadcast(&self, request: SendBlockRequest) -> Result<(), Vec<Endpoint>>;
}
