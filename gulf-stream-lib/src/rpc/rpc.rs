use std::sync::Arc;

use crate::ed25519::publickey::PublicKey;
use crate::err::GulfStreamError;
use crate::ledger::ledger::{Explorer, Ledger};
use crate::pb::node_client::NodeClient;
use crate::pb::node_server::Node;
use crate::pb::{
    GenericResponse, GetBalanceRequest, GetBalanceResponse, GetHistoryRequest, SendBlockRequest,
    SendTransactionRequest, TransactionHistory,
};
use crate::state::block::Block;
use crate::state::transaction::Transaction;
use crate::utils::serde::BytesDeserialize;
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

        if !(tx.sign_is_valid() && tx.tx_msg_is_valid()) {
            return Err(GulfStreamError::TxIsNotValid.into());
        }

        let reply = GenericResponse {
            message: format!("Tx {:?} inserted", tx.signature),
        };

        self.ledger.mem_pool.lock().await.push(tx);

        return Ok(Response::new(reply));
    }

    async fn get_history(
        &self,
        _request: Request<GetHistoryRequest>,
    ) -> Result<Response<TransactionHistory>, Status> {
        let history = self.ledger.get_transaction_history().await;
        let reply = TransactionHistory {
            transactions: history
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()
                .unwrap(),
        };
        return Ok(Response::new(reply));
    }

    async fn get_balance(
        &self,
        request: Request<GetBalanceRequest>,
    ) -> Result<Response<GetBalanceResponse>, Status> {
        let balance =
            self.ledger.state.lock().await.get_latest().get_balance(
                &PublicKey::deserialize(&mut &request.into_inner().address[..]).unwrap(),
            );
        let reply = GetBalanceResponse {
            balance: balance.to_u64().unwrap(),
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
pub trait Broadcaster {
    async fn broadcast(&self, request: SendBlockRequest) -> Result<(), Vec<Endpoint>>;
}
