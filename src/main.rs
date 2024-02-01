use core::database::Database;

use crate::core::cli::Cli;

mod core;
mod utils;

#[tokio::main(worker_threads = 1)]
async fn main() {
    Database::init();
    Cli::new().run().await;
}
