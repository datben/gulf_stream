use std::sync::Arc;

use gulf_stream_lib::{ledger::ledger::*, state::blockchain::Blockchain, store::db};
use tokio::sync::Mutex;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    port: u64,

    #[arg(long)]
    host_known: Option<u64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let addr = format!("0.0.0.0:{}", args.port).parse()?;

    let other_nodes = if let Some(host_known) = args.host_known {
        let host_known = format!("http://0.0.0.0:{}", host_known);
        Mutex::new(vec![host_known.try_into().unwrap()])
    } else {
        Mutex::new(vec![])
    };

    let pg_runtime = db::run_db().await?;
    pg_runtime.create_database("state").await?;

    let pg = db::get_client(pg_runtime.full_db_uri("state")).await?;

    let ledger = Arc::new(Ledger {
        state: Mutex::new(Blockchain::default()),
        other_nodes,
        mem_pool: Mutex::new(vec![]),
        db: Arc::new(pg),
    });

    let rpc_runtime = ledger.clone().run_rpc(addr);
    let node_runtime = ledger.clone().run_node();
    let logs = ledger.clone().run_logs();
    let db_runtime = ledger.clone().run_db();

    rpc_runtime.await??;
    logs.await?;
    node_runtime.await?;
    db_runtime.await??;

    Ok(())
}
