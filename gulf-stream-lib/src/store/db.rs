use pg_embed::pg_enums::PgAuthMethod;
use pg_embed::pg_fetch::{PgFetchSettings, PG_V13};
use pg_embed::postgres::{PgEmbed, PgSettings};
use sqlx::{Pool, Postgres, Row};
use std::path::PathBuf;
use std::time::Duration;

use crate::ed25519::publickey::PublicKey;
use crate::ed25519::signature::Signature;
use crate::err::GulfStreamError;
use crate::state::transaction::Transaction;
use sqlx::postgres::{PgPoolOptions, PgRow};

pub struct DbClient {
    client: Pool<Postgres>,
}

impl DbClient {
    pub async fn new(uri: String) -> Result<Self, GulfStreamError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(uri.as_str())
            .await?;
        return Ok(Self { client: pool });
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
        sqlx::query(
            "CREATE TABLE blocks ( blockheight bigint primary key, blockhash varchar(40) NOT NULL );"
        ).execute(&self.client)
        .await?;
        sqlx::query(
            "CREATE TABLE txs ( signature varchar(92) primary key NOT NULL, blockheight bigint, gas bigint, payer varchar(44));",
        ).execute(&self.client)
        .await?;
        Ok(())
    }

    pub async fn get_tx(&self, tx: &Signature) -> Result<Transaction, GulfStreamError> {
        let row = sqlx::query("SELECT * FROM txs WHERE signature = $1 ;")
            .bind(tx.into_string())
            .fetch_one(&self.client)
            .await?;
        Self::map_row_to_tx(row)
    }

    pub async fn insert_tx(&self, tx: &Transaction) -> Result<(), GulfStreamError> {
        sqlx::query("INSERT INTO txs (signature,blockheight,gas,payer) VALUES ($1,$2,$3,$4);")
            .bind(tx.signature.into_string())
            .bind(tx.blockheight as i64)
            .bind(tx.gas as i64)
            .bind(tx.payer.into_string())
            .execute(&self.client)
            .await?;
        Ok(())
    }

    fn map_row_to_tx(row: PgRow) -> Result<Transaction, GulfStreamError> {
        let blockheight: i64 = row.get(&"blockheight");
        let gas: i64 = row.get(&"gas");
        let payer: &str = row.get(&"payer");
        let signature: &str = row.get(&"signature");
        Ok(Transaction {
            blockheight: blockheight as u64,
            gas: gas as u64,
            msg: Default::default(),
            payer: PublicKey::try_from_str(payer)?,
            signature: Signature::try_from_str(signature)?,
        })
    }
}
