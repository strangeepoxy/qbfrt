//! qbfrt configuration module
//!
//! Configures the application based on the passed command line arguments

use argh::FromArgs;
use directories::BaseDirs;
use std::path::{Path, PathBuf};

/// Returns the OS-specific qB .torrent data directory
fn get_qb_dir() -> PathBuf {
    let base_dirs = BaseDirs::new().unwrap();
    let local_appdata_dir = base_dirs.data_local_dir();
    local_appdata_dir.join("qBittorrent")
}

/// CLI argument options
#[derive(FromArgs)]
struct CLIOpts {
    /// path to qB local config directory (where torrents.db lives)
    #[argh(option, short = 'p')]
    config_dir: Option<String>,
    /// disable automatic torrents.db backup
    #[argh(switch, short = 'd')]
    disable_backup: bool,
    /// enable verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

/// Application configuration generated from CLI arguments
#[derive(Debug)]
pub struct Config {
    /// Path to qB local config directory (where torrents.db lives)
    pub qb_directory: PathBuf,
    /// The full path to torrents.db
    pub db_file: PathBuf,
    /// Toggles the automatic torrents.db backup
    pub disable_backup: bool,
    /// Toggles verbose output
    pub verbose: bool,
}

impl Config {
    /// Builds application configuration using command line arguments
    ///
    /// ## Example
    /// ```rs
    /// let config = Config::build().unwrap_or_else(|err| {
    ///     println!("Problem parsing arguments: {err}");
    ///     process::exit(1);
    /// });
    /// ```
    pub fn build() -> Result<Config, String> {
        let args: CLIOpts = argh::from_env();
        let qb_dir = get_qb_dir();

        let qb_directory = match args.config_dir {
            Some(dir) => Path::new("").join(dir),
            _ => qb_dir,
        };

        let db_file = Path::new(&qb_directory).join("torrents.db");

        let config = Config {
            qb_directory,
            db_file,
            disable_backup: args.disable_backup,
            verbose: args.verbose,
        };

        if config.verbose {
            println!("Verbose output enabled");
            println!("Using {:?} as qB directory", config.qb_directory);
            println!("{:?}", config);
        }

        Ok(config)
    }
}
