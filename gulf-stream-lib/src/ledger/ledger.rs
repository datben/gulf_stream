use std::net::SocketAddr;
use std::sync::Arc;

use crate::rpc::rpc::GulfStreamRpc;
use crate::state::block::{Block, TransactionState};
use crate::state::blockchain::Blockchain;
use crate::state::blockhash::Blockhash;
use crate::{
    pb::{node_server::NodeServer, SendBlockRequest},
    rpc::rpc::Broadcaster,
};
use tokio::sync::Mutex;
use tonic::transport::{Endpoint, Server};

#[derive(Default)]
pub struct Ledger {
    pub state: Mutex<Blockchain>,
    pub mem_pool: Mutex<Vec<TransactionState>>,
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

                if let Some(block) = ledger
                    .clone()
                    .try_build_block(latest_block.index, &latest_block.blockhash)
                    .await
                {
                    match ledger.state.lock().await.try_insert(&block) {
                        Ok(_) => ledger.mem_pool.lock().await.clear(),
                        Err(err) => println!("{:?}", err),
                    };

                    match ledger
                        .broadcast(SendBlockRequest {
                            block: Some(block.try_into().unwrap()),
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
    ) -> Option<Block> {
        let can_build_block = self.mem_pool.lock().await.len() > 0;
        return if can_build_block {
            let mut nonce = 0;
            let txs = self
                .mem_pool
                .lock()
                .await
                .clone()
                .into_iter()
                .filter(|tx| {
                    return tx.is_pending() && tx.into_tx().blockheight == previous_index + 1;
                })
                .collect();
            let raw_txs = TransactionState::get_raw_txs(&txs);
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
                        transactions: txs,
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

    async fn filter_mempool(&self) {
        let mut mempool = self.mem_pool.lock().await;
        *mempool = mempool
            .clone()
            .into_iter()
            .filter(|tx| tx.is_pending())
            .collect::<Vec<TransactionState>>();
    }
}

#[tonic::async_trait]
pub trait BlockBuilder {
    async fn try_build_block(
        &self,
        previous_index: u64,
        previous_blockhash: &Blockhash,
    ) -> Option<Block>;

    async fn filter_mempool(&self);
}

#[cfg(test)]
mod test {

    use crate::{
        ed25519::publickey::PublicKey,
        state::transaction::{Transaction, TransactionMessage},
    };

    use super::*;

    #[tokio::test]
    pub async fn filter_mempool() {
        let pk1 = PublicKey::random();
        let pk2 = PublicKey::random();
        let ledger = Ledger::default();
        ledger.mem_pool.lock().await.extend(vec![
            Transaction {
                blockheight: 1,
                msg: TransactionMessage::Mint { amount: 12 },
                payer: pk1.to_owned(),
                signature: Default::default(),
                gas: 0,
            }
            .into_tx_state(),
            Transaction {
                blockheight: 1,

                msg: TransactionMessage::Mint { amount: 57 },
                payer: pk2.to_owned(),
                signature: Default::default(),
                gas: 0,
            }
            .into_tx_state()
            .success(),
        ]);

        ledger.filter_mempool().await;

        assert!(ledger.mem_pool.lock().await.len() == 1);
        assert!(ledger.mem_pool.lock().await[0].is_pending())
    }
}
