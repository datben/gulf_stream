use std::net::SocketAddr;
use std::sync::Arc;

use crate::rpc::rpc::GulfStreamRpc;
use crate::state::block::Block;
use crate::state::blockchain::Blockchain;
use crate::state::blockhash::Blockhash;
use crate::state::transaction::Transaction;
use crate::{
    pb::{node_server::NodeServer, SendBlockRequest},
    rpc::rpc::Broadcaster,
};
use tokio::sync::Mutex;
use tonic::transport::{Endpoint, Server};

pub struct Ledger {
    pub state: Mutex<Blockchain>,
    pub mem_pool: Mutex<Vec<Transaction>>,
    pub other_nodes: Mutex<Vec<Endpoint>>,
}

impl Ledger {
    pub fn run_rpc(
        self: Arc<Ledger>,
        socket: SocketAddr,
    ) -> tokio::task::JoinHandle<std::result::Result<(), tonic::transport::Error>> {
        let rpc = GulfStreamRpc {
            ledger: self.clone(),
        };
        tokio::spawn(async move {
            Server::builder()
                .add_service(NodeServer::new(rpc))
                .serve(socket)
                .await
        })
    }

    pub fn run_logs(self: Arc<Ledger>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                let ledger = self.clone();
                let latest_block = ledger
                    .clone()
                    .state
                    .lock()
                    .await
                    .latest_links
                    .get(0)
                    .unwrap()
                    .block
                    .clone();
                println!(
                    "Ledger latest block : index = {:?}, blockhash = {}, tx = {}",
                    latest_block.index,
                    latest_block.blockhash,
                    latest_block.transactions.len()
                );
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        })
    }

    pub fn run_node(self: Arc<Ledger>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                let ledger = self.clone();
                let latest_block = ledger
                    .clone()
                    .state
                    .lock()
                    .await
                    .latest_links
                    .get(0)
                    .unwrap()
                    .block
                    .clone();

                let mempool = ledger.mem_pool.lock().await.clone();
                if let Some(block) = ledger
                    .clone()
                    .try_build_block(latest_block.index, &latest_block.blockhash, mempool)
                    .await
                {
                    match ledger.state.lock().await.try_insert(&block) {
                        Ok(_) => ledger.mem_pool.lock().await.clear(),
                        Err(err) => println!("{:?}", err),
                    };

                    match ledger
                        .broadcast(SendBlockRequest {
                            block: Some(block.into()),
                        })
                        .await
                    {
                        Ok(_) => println!("Block broadcasted"),
                        Err(err) => println!("Broadcast failed for : {:?}", err),
                    };
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        })
    }
}

#[tonic::async_trait]
impl BlockBuilder for Ledger {
    async fn try_build_block(
        &self,
        previous_index: u64,
        previous_blockhash: &Blockhash,
        transactions: Vec<Transaction>,
    ) -> Option<Block> {
        let can_build_block = self.mem_pool.lock().await.len() > 0;
        return if can_build_block {
            let mut nonce = 0;
            let raw_txs = transactions
                .iter()
                .flat_map(|tx| Into::<Vec<u8>>::into(tx.to_owned()))
                .collect::<Vec<u8>>();
            loop {
                let blockhash = Blockhash::from_raw_data(
                    previous_index + 1,
                    previous_blockhash,
                    &raw_txs,
                    nonce,
                );
                if blockhash.is_valid(1) {
                    return Some(Block {
                        index: previous_index + 1,
                        blockhash,
                        previous_blockhash: previous_blockhash.to_owned(),
                        transactions,
                        nonce,
                    });
                } else {
                    nonce += 1;
                }
            }
        } else {
            None
        };
    }
}

#[tonic::async_trait]
pub trait BlockBuilder {
    async fn try_build_block(
        &self,
        previous_index: u64,
        previous_blockhash: &Blockhash,
        transactions: Vec<Transaction>,
    ) -> Option<Block>;
}
