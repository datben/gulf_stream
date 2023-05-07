use std::{path::PathBuf, sync::Arc};

use gulf_stream_lib::{
    ledger::ledger::*,
    state::{blockchain::Blockchain, transaction::Transaction},
    store::db::DbClient,
};
use tokio::sync::Mutex;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    port: u64,

    #[arg(long)]
    host_known: Option<u64>,

    #[arg(short, long, default_value_t = false)]
    reset: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let addr = format!("0.0.0.0:{}", args.port).parse()?;

    let other_nodes = if let Some(host_known) = args.host_known {
        let host_known = format!("http://0.0.0.0:{}", host_known);
        Mutex::new(vec![host_known.try_into().unwrap()])
    } else {
        Mutex::new(vec![])
    };

    if args.reset {
        std::fs::remove_dir_all(PathBuf::from("./data"))?;
    }

    let pg_runtime = DbClient::launch_pg_embed().await?;

    if args.reset {
        pg_runtime.create_database("state").await?;
    }

    let client = DbClient::new(pg_runtime.full_db_uri("state")).await?;

    println!("DB uri {}", pg_runtime.full_db_uri("state"));

    if args.reset {
        client.init_tables().await?;
    }

    client
        .insert_tx(&Transaction {
            blockheight: 5,
            gas: 62,
            msg: Default::default(),
            payer: Default::default(),
            signature: Default::default(),
        })
        .await?;

    let ledger = Arc::new(Ledger {
        state: Mutex::new(Blockchain::default()),
        other_nodes,
        mem_pool: Mutex::new(vec![]),
        db: Arc::new(client),
    });

    let rpc_runtime = ledger.clone().run_rpc(addr);
    let node_runtime = ledger.clone().run_node();
    let logs = ledger.clone().run_logs();
    let db_runtime = ledger.clone().run_db();

    rpc_runtime.await??;
    logs.await?;
    node_runtime.await??;
    db_runtime.await??;

    Ok(())
}
