use gulf_stream_lib::{
    ledger::ledger::Ledger, pb::node_server::NodeServer, state::blockchain::Blockchain,
};
use tokio::sync::Mutex;
use tonic::transport::Server;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    port: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let addr = format!("[::1]:{}", args.port).parse()?;

    let ledger = Ledger {
        state: Mutex::new(Blockchain::default()),
        other_nodes: Mutex::new(vec![]),
    };

    Server::builder()
        .add_service(NodeServer::new(ledger))
        .serve(addr)
        .await?;

    Ok(())
}
