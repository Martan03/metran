use std::path::PathBuf;

use pareg::Pareg;
use termal::printcln;

use crate::error::Error;

/// Parses the CLI arguments and contains the parsed values.
#[derive(Debug, Default)]
pub struct Args {
    /// Server address.
    pub address: Option<String>,
    /// Server port.
    pub port: Option<u16>,
    /// Output directory
    pub output_dir: Option<PathBuf>,
    /// Whether program should quit after parsing arguments.
    pub should_quit: bool,
}

impl Args {
    pub const VERSION_NUMBER: &str = {
        let v = option_env!("CARGO_PKG_VERSION");
        if let Some(v) = v { v } else { "unknown" }
    };

    /// Parses the given arguments and checks containts.
    ///
    /// Returns the parsed arguments on success, else corresponding error.
    pub fn parse(mut args: Pareg) -> Result<Self, Error> {
        let mut parsed = Self::default();
        while let Some(arg) = args.next() {
            match arg {
                "-a" | "--address" => parsed.address = Some(args.next_arg()?),
                "-p" | "--port" => parsed.port = Some(args.next_arg()?),
                "-h" | "--help" => {
                    Self::help();
                    parsed.should_quit = true;
                }
                _ => return Err(args.err_unknown_argument().into()),
            }
        }

        Ok(parsed)
    }

    /// Gets the server address in format address:port.
    ///
    /// Uses 0.0.0.0 as default for address and 3000 for port.
    pub fn server_addr(&self) -> String {
        format!(
            "{}:{}",
            self.address
                .as_ref()
                .map(|v| v.as_str())
                .unwrap_or("0.0.0.0"),
            self.port.unwrap_or(8080)
        )
    }

    /// Gets the set output directory.
    ///
    /// Uses "./received" directory by default.
    pub fn output_dir(&self) -> PathBuf {
        self.output_dir
            .clone()
            .unwrap_or(PathBuf::from("./received"))
    }

    /// Prints help
    pub fn help() {
        printcln!(
            "Welcome to {'g}metran{'_} by {}{'_}
{'bl}Version {}{'_}

Media Transfer server written in Rust.

{'g}Usage{'_}:
  {'c}metran{'_}
    Starts the server.

  {'c}metran{'_} [{'y}flags{'_}]
    Starts the server according to the flags.

{'g}Flags{'_}:
  {'y}-h  --help{'_}
    Displays this help.

  {'y}-a  --address{'_}
    Sets the address of the server.

  {'y}-p  --port{'_}
    Sets the port number of the server.",
            termal::gradient("Martan03", (0, 220, 255), (175, 80, 255)),
            Self::VERSION_NUMBER
        );
    }
}
