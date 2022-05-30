use std::time::Duration;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use archive_gateway::ArchiveGateway;

mod logger;

#[derive(Parser, Debug)]
#[clap(about)]
struct Args {
    /// Database connection string
    #[clap(long)]
    database_url: String,

    /// Maximum number of connections supported by pool
    #[clap(long, default_value_t = 1)]
    database_max_connections: u32,

    /// EVM pallet support
    #[clap(long)]
    evm_support: bool,

    /// Сontracts pallet support
    #[clap(long)]
    contracts_support: bool,
}

#[tracing::instrument]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    logger::init();

    let pool = PgPoolOptions::new()
        .max_connections(args.database_max_connections)
        .connect_timeout(Duration::from_secs(5))
        .connect_lazy(&args.database_url)
        .unwrap();
    ArchiveGateway::new(pool, args.evm_support, args.contracts_support)
        .run()
        .await
}
