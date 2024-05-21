//! # qbfrt (qBittorrent fastresume tool)
//! Command line tool for working with qBittorrent's fastresume data. Supports the
//! experimental SQLite database and (soon) traditional .fastresume files.
//!
//! ## Features
//! With this tool you can:
//! - Mass update the save paths for torrents in the SQLite database
//!     - Change files to a new drive or directory without having to move torrents in qBittorrent or recheck all of the torrent data
//!     - Migrate from qBittorrent on Windows to Linux without having to recheck the torrent data
//!
//! **More functionality to come!**

#![warn(missing_docs)]

pub mod config;
pub mod db;
