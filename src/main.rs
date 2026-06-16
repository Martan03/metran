use std::process::ExitCode;

use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart, State, multipart::Field},
    http::StatusCode,
    routing::{get, post},
};
use pareg::Pareg;
use termal::{eprintcln, printcln};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::{app_state::AppState, args::Args, error::Error};

pub mod app_state;
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

    let state = AppState::from_args(&args);
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/upload", post(accept_photos))
        .layer(DefaultBodyLimit::disable())
        .with_state(state);

    let addr = args.server_addr();
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("Server listening on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn accept_photos(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> StatusCode {
    if let Err(e) = fs::create_dir_all(&state.output_dir).await {
        eprintcln!("{'r}Error:{'_} {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    while let Ok(Some(mut field)) = multipart.next_field().await {
        // TODO: handle this better
        let filename = field.file_name().unwrap_or("unknown").to_string();

        let filepath = state.output_dir.join(&filename);
        match File::create(&filepath).await {
            Ok(mut file) => loop {
                match field.chunk().await {
                    Ok(Some(chunk)) => {
                        if let Err(e) = file.write_all(&chunk).await {
                            eprintcln!("{'r}Error:{'_} {}", e);
                            return StatusCode::INTERNAL_SERVER_ERROR;
                        }
                    }
                    Ok(None) => {
                        printcln!("{'g}Successfully saved:{'_} {}", filename);
                        break;
                    }
                    Err(e) => {
                        eprintcln!("{'r}Error:{'_} {}", e);
                        return StatusCode::INTERNAL_SERVER_ERROR;
                    }
                }
            },
            Err(e) => {
                eprintcln!("{'r}Error:{'_} {}", e);
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        }
    }
    StatusCode::OK
}
