use std::sync::Arc;

use gulf_stream_lib::{
    ledger::ledger::*, pb::node_server::NodeServer, rpc::rpc::GulfStreamRpc,
    state::blockchain::Blockchain,
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
    let ledger_printer_pointer = ledger.clone();
    let printer_runtime = tokio::spawn(async move {
        loop {
            let ledger = ledger_printer_pointer.clone();
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
    });

    let ledger_node_pointer = ledger.clone();
    let node_runtime = tokio::spawn(async move {
        loop {
            let ledger = ledger_node_pointer.clone();
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
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
            }
        }
    });

    rpc_runtime.await??;
    printer_runtime.await?;
    node_runtime.await?;

    Ok(())
}
