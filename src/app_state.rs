use std::path::PathBuf;

use crate::args::Args;

/// Contains the state of the app.
///
/// These fields are needed when handling requests.
#[derive(Debug, Clone)]
pub struct AppState {
    pub output_dir: PathBuf,
}

impl AppState {
    /// Creates app state from the parsed arguments.
    pub fn from_args(args: &Args) -> Self {
        Self {
            output_dir: args.output_dir(),
        }
    }
}
