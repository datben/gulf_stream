use std::net::SocketAddr;
use std::ops::Add;
use std::sync::Arc;

use crate::ed25519::publickey::PublicKey;
use crate::rpc::rpc::GulfStreamRpc;
use crate::state::block::{Block, TransactionState};
use crate::state::blockchain::Blockchain;
use crate::state::blockhash::Blockhash;
use crate::state::transaction::Transaction;
use crate::{
    pb::{node_server::NodeServer, SendBlockRequest},
    rpc::rpc::Broadcaster,
};
use tokio::sync::Mutex;
use tonic::transport::{Endpoint, Server};

#[derive(Default)]
pub struct Ledger {
    pub state: Mutex<Blockchain>,
    pub mem_pool: Mutex<Vec<Transaction>>,
    pub transaction_history: Mutex<Vec<TransactionState>>,
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
                println!(
                    "Ledger mempool : {} txs",
                    ledger.clone().mem_pool.lock().await.len(),
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
                    match self
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
            let mut txs = self
                .mem_pool
                .lock()
                .await
                .clone()
                .into_iter()
                .filter(|tx| {
                    return tx.blockheight == previous_index + 1;
                })
                .collect::<Vec<Transaction>>();

            txs.sort_by(|a, b| b.cmp(a));

            let involved_pk: Vec<PublicKey> = Transaction::get_involved_pk_from_txs(&txs);

            let state = self.state.lock().await;

            let mut balance_deltas = state.get_latest().get_balances(&involved_pk);

            let mut valid_txs: Vec<Transaction> = vec![];
            let mut valid_index: Vec<usize> = vec![];

            txs.into_iter().enumerate().for_each(|(index, tx)| {
                let tx_balance_deltas = tx.get_balance_deltas();
                tx_balance_deltas.iter().for_each(|(pk, delta)| {
                    if let Some(balance_delta) = balance_deltas.get_mut(pk) {
                        let delta_if_executed = delta.add(balance_delta.to_owned());
                        if delta_if_executed.is_positive_or_nil() {
                            *balance_delta = delta_if_executed;
                            valid_txs.push(tx.clone());
                            valid_index.push(index);
                        }
                    } else {
                        balance_deltas.insert(pk.to_owned(), delta.to_owned());
                    }
                });
            });

            if valid_txs.is_empty() {
                return None;
            }

            let raw_txs = Transaction::get_raw_txs(&valid_txs);
            loop {
                let blockhash = Blockhash::from_raw_data(
                    previous_index + 1,
                    previous_blockhash,
                    &raw_txs,
                    nonce,
                );
                if blockhash.is_valid(1) {
                    let block = Block {
                        index: previous_index + 1,
                        blockhash,
                        previous_blockhash: previous_blockhash.to_owned(),
                        transactions: valid_txs.clone(),
                        nonce,
                    };

                    return match self.state.lock().await.try_insert(&block) {
                        Ok(_) => {
                            {
                                let mut mempool_guard = self.mem_pool.lock().await;
                                valid_index.iter().rev().for_each(|index| {
                                    mempool_guard.swap_remove(*index);
                                });
                            }
                            {
                                let mut history_guard = self.transaction_history.lock().await;
                                valid_txs.iter().for_each(|tx| {
                                    history_guard.push(TransactionState::Success(tx.clone()));
                                });
                            }
                            Some(block)
                        }
                        Err(_) => None,
                    };
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
    ) -> Option<Block>;
}
