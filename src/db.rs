//! qBittorrent SQLite database module
//!
//! Mediates interactions with the experimental qB torrents.db fastresume database
//!
//! ## Examples and Usage
//! ### Connecting to the database
//! ```rs
//! let conn = DB::connect(&config).unwrap_or_else(|err| {
//!     println!("Could not connect to database: {err}");
//!     process::exit(1);
//! });
//! ```
//! ### Backing up the database
//! ```rs
//! DB::backup(&config).unwrap_or_else(|err| {
//!     println!("Could not backup database: {err}");
//!     process::exit(1);
//! });
//!

use crate::config::Config;
use rusqlite::{Connection, OpenFlags, Result};
use std::error::Error;
use std::fs;
use std::path::Path;

/// qB torrents.db struct
pub struct DB {}

impl DB {
    /// Automatically creates a timestamped backup of the torrents.db before modification
    ///
    /// ## Examples
    /// ```rs
    /// DB::backup(&config).unwrap_or_else(|err| {
    ///     println!("Could not backup database: {err}");
    ///     process::exit(1);
    /// });
    ///```
    /// ## Disabling
    /// Automatic backups can be disabled by passing `--disable-backup` or `-d`
    /// ```bash
    /// qbfrt --disable-backup
    /// ```
    pub fn backup(config: &Config) -> Result<(), Box<dyn Error>> {
        if !config.disable_backup {
            println!("Creating database backup...");

            let datetime = chrono::offset::Local::now().format("%Y%m%d%H%M%S");
            let backup_file =
                Path::new(&config.qb_directory).join(format!("torrents.db-{datetime}.bak"));
            fs::copy(&config.db_file, &backup_file)?;

            if config.verbose {
                println!("Backup saved to: {:?}", backup_file);
            }
        } else if config.verbose {
            println!("Database backup disabled");
        }

        Ok(())
    }

    /// Opens the torrents.db SQLite database and returns the connection
    ///
    /// ## Example
    /// ```rs
    /// let conn = DB::connect(&config).unwrap_or_else(|err| {
    ///     println!("Could not connect to database: {err}");
    ///     process::exit(1);
    /// });
    /// ```
    pub fn connect(config: &Config) -> Result<Connection, Box<dyn Error>> {
        println!("Opening database...");

        Ok(Connection::open_with_flags(
            &config.db_file,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_URI
                | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?)
    }
}
