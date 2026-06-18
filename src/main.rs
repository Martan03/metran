use std::process::ExitCode;

use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart, State},
    http::StatusCode,
    response::IntoResponse,
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
        .route("/upload", post(handle_upload))
        .layer(DefaultBodyLimit::disable())
        .with_state(state);

    let addr = args.server_addr();
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("Server listening on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handle_upload(
    State(state): State<AppState>,
    multipart: Multipart,
) -> impl IntoResponse {
    match accept_photos(state, multipart).await {
        Ok(true) => (StatusCode::OK, "SUCCESS").into_response(),
        Ok(false) => {
            eprintcln!("{'y}Warning:{'_} Received empty payload. Rejecting.");
            (StatusCode::BAD_REQUEST, "ERROR_EMPTY").into_response()
        }
        Err(e) => {
            eprintcln!("{'r}Error:{'_} {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "ERROR_INTERNAL")
                .into_response()
        }
    }
}

async fn accept_photos(
    state: AppState,
    mut multipart: Multipart,
) -> Result<bool, Error> {
    fs::create_dir_all(&state.output_dir).await?;

    let mut any_file = false;
    while let Some(mut field) = multipart.next_field().await? {
        // TODO: handle this better
        let filename = field.file_name().unwrap_or("unknown").to_string();
        let filepath = state.output_dir.join(&filename);

        let mut file = File::create(&filepath).await?;
        while let Some(chunk) = field.chunk().await? {
            file.write_all(&chunk).await?;
        }

        printcln!("{'g}Successfully saved:{'_} {}", filename);
        any_file = true;
    }
    Ok(any_file)
}
