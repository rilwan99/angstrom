use clap::Parser;

use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Run {
    #[arg(long, value_name = "FILE")]
    pub staking_secret_key: PathBuf,
    #[arg(long, value_name = "FILE")]
    pub bundle_key: PathBuf,
    
}

#[tokio::main]
async fn main() {}
