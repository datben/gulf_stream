use pg_embed::pg_enums::PgAuthMethod;
use pg_embed::pg_fetch::{PgFetchSettings, PG_V13};
use pg_embed::postgres::{PgEmbed, PgSettings};
use std::path::PathBuf;
use std::time::Duration;
use tokio_postgres::{Client, NoTls, Row};

use crate::ed25519::publickey::PublicKey;
use crate::ed25519::signature::Signature;
use crate::err::GulfStreamError;
use crate::state::transaction::Transaction;

pub struct DbClient {
    client: Client,
}

impl DbClient {
    pub async fn new(uri: String) -> Result<Self, GulfStreamError> {
        let (client, connection) = tokio_postgres::connect(uri.as_str(), NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        return Ok(Self { client });
    }

    pub async fn launch_pg_embed() -> Result<PgEmbed, GulfStreamError> {
        let pg_settings = PgSettings {
            // Where to store the postgresql database
            database_dir: PathBuf::from("data/db"),
            port: 5432,
            user: "postgres".to_string(),
            password: "password".to_string(),
            auth_method: PgAuthMethod::Plain,
            persistent: true,
            timeout: Some(Duration::from_secs(15)),
            migration_dir: None,
        };
        let fetch_settings = PgFetchSettings {
            version: PG_V13,
            ..Default::default()
        };
        let mut pg = PgEmbed::new(pg_settings, fetch_settings).await?;
        pg.setup().await?;
        pg.start_db().await?;
        Ok(pg)
    }

    pub async fn init_tables(&self) -> Result<(), GulfStreamError> {
        self.client.simple_query(
            "CREATE TABLE blocks ( blockheight int primary key, blockhash varchar(40) NOT NULL );",
        )
        .await?;
        self.client.simple_query(
        "CREATE TABLE txs ( signature varchar(92) primary key NOT NULL, blockheight int, gas int, payer varchar(44));").await?;
        Ok(())
    }

    pub async fn get_tx(&self, tx: &Signature) -> Result<Transaction, GulfStreamError> {
        let row = self
            .client
            .query_one("SELECT $1 FROM txs", &[&tx.into_string()])
            .await?;
        Self::map_row_to_tx(row)
    }

    pub async fn insert_tx(&self, tx: &Transaction) -> Result<(), GulfStreamError> {
        self.client
            .query(
                "INSERT INTO txs (signature,blockheight,gas,payer) VALUES ($1,$2,$3,$4)",
                &[
                    &tx.signature.into_string(),
                    &tx.blockheight.to_string(),
                    &tx.gas.to_string(),
                    &tx.payer.into_string(),
                ],
            )
            .await?;
        Ok(())
    }

    fn map_row_to_tx(row: Row) -> Result<Transaction, GulfStreamError> {
        let blockheight: &str = row.get(&"blockheight");
        let gas: &str = row.get(&"gas");
        let payer: &str = row.get(&"payer");
        let signature: &str = row.get(&"signature");
        Ok(Transaction {
            blockheight: blockheight.parse::<u64>()?,
            gas: gas.parse::<u64>()?,
            msg: Default::default(),
            payer: PublicKey::try_from_str(payer)?,
            signature: Signature::try_from_str(signature)?,
        })
    }
}
