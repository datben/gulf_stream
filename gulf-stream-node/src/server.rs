use std::sync::Arc;

use gulf_stream_lib::{
    ledger::ledger::Ledger,
    pb::node_server::NodeServer,
    rpc::rpc::GulfStreamRpc,
    state::{blockchain::Blockchain, link::Link},
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

    let ledger = Arc::new(Ledger {
        state: Mutex::new(Blockchain::default()),
        other_nodes: Mutex::new(vec![]),
        mem_pool: Mutex::new(vec![]),
    });

    let rpc = GulfStreamRpc {
        ledger: ledger.clone(),
    };

    let rpc_runtime = tokio::spawn(async move {
        Server::builder()
            .add_service(NodeServer::new(rpc))
            .serve(addr)
            .await
    });

    let printer_runtime = tokio::spawn(async move {
        loop {
            let latest_block = ledger
                .state
                .lock()
                .await
                .latest_links
                .get(0)
                .unwrap_or(&Arc::new(Link::default()))
                .block
                .clone();
            println!(
                "Ledger latest block : index = {:?}, blockhash = {}",
                latest_block.index, latest_block.blockhash
            );
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });

    let node_runtime = tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });

    rpc_runtime.await??;
    printer_runtime.await?;
    node_runtime.await?;

    Ok(())
}
