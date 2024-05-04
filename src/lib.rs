//! Command line tool for working with qBittorrent's fastresume data
//!
//! ## Features
//! With this tool you can:
//! - Currently do nothing!
//!
//! **More functionality to come!**
//!
//! The application will look for the default qBittorrent data directory
//! containing the torrents.db file. This behavior can be changed by passing
//! `--data_dir /some/path/to/db`
//!
//! ## Arguments
//! - `-p` / `--data_dir` - Path to the qB .torrent data directory (where torrents.db lives)
//! - `-d` / `--disable_backup` - Disables the automatic torrents.db backup
//! - `-v` / `--verbose` - Enables more verbose output
//!
//! ## Examples and Usage
//!
//! ## Notes

#![warn(missing_docs)]

pub mod config;
pub mod db;
pub mod fastresume_db;
