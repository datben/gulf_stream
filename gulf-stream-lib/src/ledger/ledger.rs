use crate::state::block::Block;
use crate::state::blockchain::Blockchain;
use crate::state::blockhash::Blockhash;
use crate::state::transaction::Transaction;
use tokio::sync::Mutex;
use tonic::transport::Endpoint;

pub struct Ledger {
    pub state: Mutex<Blockchain>,
    pub mem_pool: Mutex<Vec<Transaction>>,
    pub other_nodes: Mutex<Vec<Endpoint>>,
}

#[tonic::async_trait]
impl BlockBuilder for Ledger {
    async fn try_build_block(
        &self,
        previous_index: u64,
        previous_blockhash: &Blockhash,
        transactions: Vec<Transaction>,
    ) -> Option<Block> {
        let can_build_block = self.mem_pool.lock().await.len() > 1;
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
                if blockhash.is_valid(0) {
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
trait BlockBuilder {
    async fn try_build_block(
        &self,
        previous_index: u64,
        previous_blockhash: &Blockhash,
        transactions: Vec<Transaction>,
    ) -> Option<Block>;
}
