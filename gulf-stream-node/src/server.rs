use gulf_stream_lib::{
    ledger::ledger::Ledger, pb::node_server::NodeServer, state::blockchain::Blockchain,
};
use tokio::sync::Mutex;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let ledger = Ledger {
        state: Mutex::new(Blockchain::default()),
    };

    Server::builder()
        .add_service(NodeServer::new(ledger))
        .serve(addr)
        .await?;

    Ok(())
}
