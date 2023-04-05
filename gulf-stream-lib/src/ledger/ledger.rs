use crate::err::Result;
use crate::state::block::Block;
use crate::state::blockchain::Blockchain;
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
    async fn build_block(&self) -> Result<Block> {
        todo!()
    }

    async fn can_build_block(&self) -> bool {
        self.mem_pool.lock().await.len() > 1
    }
}

#[tonic::async_trait]
trait BlockBuilder {
    async fn build_block(&self) -> Result<Block>;

    async fn can_build_block(&self) -> bool;
}
