use std::process::ExitCode;

use axum::{Router, routing::get};
use pareg::Pareg;
use termal::eprintcln;

use crate::{args::Args, error::Error};

pub mod args;
pub mod error;

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintcln!("{'r}Error:{'_} {}", e);
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), Error> {
    let args = Args::parse(Pareg::args())?;
    if args.should_quit {
        return Ok(());
    }

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let addr = args.server_addr();
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("Server listening on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}
