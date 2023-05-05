use pg_embed::pg_enums::PgAuthMethod;
use pg_embed::pg_errors::PgEmbedError;
use pg_embed::pg_fetch::{PgFetchSettings, PG_V13};
use pg_embed::postgres::{PgEmbed, PgSettings};
use std::path::PathBuf;
use std::time::Duration;
use tokio_postgres::{Client, NoTls};

pub async fn run_db() -> Result<PgEmbed, PgEmbedError> {
    let pg_settings = PgSettings {
        // Where to store the postgresql database
        database_dir: PathBuf::from("data/db"),
        port: 5432,
        user: "postgres".to_string(),
        password: "password".to_string(),
        // authentication method
        auth_method: PgAuthMethod::Plain,
        // If persistent is false clean up files and directories on drop, otherwise keep them
        persistent: true,
        // duration to wait before terminating process execution
        // pg_ctl start/stop and initdb timeout
        // if set to None the process will not be terminated
        timeout: Some(Duration::from_secs(15)),
        // If migration sql scripts need to be run, the directory containing those scripts can be
        // specified here with `Some(PathBuf(path_to_dir)), otherwise `None` to run no migrations.
        // To enable migrations view the **Usage** section for details
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

pub async fn get_client(uri: String) -> Result<Client, Box<dyn std::error::Error>> {
    // Connect to the database.
    let (client, connection) = tokio_postgres::connect(uri.as_str(), NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    return Ok(client);
}
