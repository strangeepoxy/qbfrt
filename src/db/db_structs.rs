//! Structs for qBittorrent SQLite data

use serde_derive::{Deserialize, Serialize};

/// qB SQLite data
///
/// Each field here corresponds to a column in the "torrents" table in the SQLite database.
#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseData {
    /// Torrent database row id
    pub id: u64,
    /// Torrent hash id
    pub torrent_id: String,
    /// Queue position for torrent
    pub queue_position: i64,
    /// Torrent name, only used on re-named torrents
    pub name: Option<String>,
    /// Torrent category
    pub category: Option<String>,
    /// Comma-separated torrent tags
    pub tags: Option<String>,
    /// Save path for torrent content
    pub target_save_path: Option<String>,
    /// Download path, used for incomplete download directory
    pub download_path: Option<String>,
    /// Torrent content layout, whether sub-folders are created or not
    pub content_layout: String,
    /// Ratio limit for seeding
    pub ratio_limit: i64,
    /// Time limit for seeding
    pub seeding_time_limit: i64,
    /// Inactivity time limit for seeding
    pub inactive_seeding_time_limit: i64,
    /// Prioritize outer (first and last) pieces
    pub has_outer_pieces_priority: i64,
    /// Torrent is seeding
    pub has_seed_status: i64,
    /// Torrent management mode
    pub operating_mode: String,
    /// Torrent is stopped
    pub stopped: i64,
    /// Stop condition for torrents
    pub stop_condition: String,
    /// Binary blob containing libtorrent fastresume data
    ///
    /// See "FastresumeData" for deserialized contents
    pub libtorrent_resume_data: Vec<u8>,
    /// Binary blob containing metadata
    pub metadata: Vec<u8>,
}

/// qB libtorrent fastresume data
///
/// This data comes from the libtorrent_resume_data column in the "torrents" table of the SQLite database.
/// In the database, it is essentially a binary blob of the actual .fastresume file qB creates by default.
#[derive(Serialize, Deserialize, Debug)]
pub struct FastresumeData {
    /// Amount of time torrent has been active
    pub active_time: i64,
    /// Date torrent was added
    pub added_time: i64,
    /// Allocation
    pub allocation: String,
    /// Enable IP filter for torrent
    pub apply_ip_filter: i64,
    /// Enable auto manage for torrent
    pub auto_managed: i64,
    /// Date torrent was completed
    pub completed_time: i64,
    /// Enable DHT for torrent
    pub disable_dht: i64,
    /// Enable LSD for torrent
    pub disable_lsd: i64,
    /// Enable PEX for torrent
    pub disable_pex: i64,
    /// Set download rate limit for torrent
    pub download_rate_limit: i64,
    /// File format, "libtorrent resume file"
    #[serde(rename = "file-format")]
    pub file_format: String,
    /// Torrent file version
    #[serde(rename = "file-version")]
    pub file_version: i64,
    /// Priority level for each file
    pub file_priority: Option<Vec<u8>>,
    /// Date torrent was finish
    pub finished_time: i64,
    /// httpseed URLs used by torrent
    pub httpseeds: Vec<u8>,
    // Parsing this is annoying right now, skipping it since it's not important
    // /// i2p
    // pub i2p: i64,
    /// Info hash for torrent
    #[serde(rename = "info-hash")]
    #[serde(with = "serde_bytes")]
    pub info_hash: Vec<u8>,
    /// V2 info hash for torrent
    #[serde(rename = "info-hash2")]
    pub info_hash2: String,
    /// Date last downloaded
    pub last_download: i64,
    /// Date last seen complete
    pub last_seen_complete: i64,
    /// Date last uploaded
    pub last_upload: i64,
    /// libtorrent version
    #[serde(rename = "libtorrent-version")]
    pub libtorrent_version: String,
    /// Max number of active connections
    pub max_connections: i64,
    /// Max number of upload slots
    pub max_uploads: i64,
    /// Torrent name
    pub name: String,
    /// Number of completed pieces
    pub num_complete: u64,
    /// Number of downloaded pieces
    pub num_downloaded: u64,
    /// Number of incomplete pieces
    pub num_incomplete: u64,
    /// Torrent pause state
    pub paused: u64,
    // Parsing this is annoying right now, skipping it since it's not important
    /// IPv4 peers
    // #[serde(with = "serde_bytes")]
    // pub peers: Option<Vec<u8>>,
    // Parsing this is annoying right now, skipping it since it's not important
    /// IPv6 peers
    // #[serde(with = "serde_bytes")]
    // pub peers6: Option<Vec<u8>>,
    /// Piece priority
    pub piece_priority: Option<String>,
    /// Torrent pieces, there should be no reason to manipulate this
    pub pieces: String,
    /// Save path for torrent content
    pub save_path: String,
    /// Seed mode
    pub seed_mode: i64,
    /// Amount of time torrent has been seeding
    pub seeding_time: u64,
    /// Enable sequential download for torrent
    pub sequential_download: i64,
    /// Toggle share mode for torrent
    pub share_mode: i64,
    /// Stop when a condition is met
    pub stop_when_ready: i64,
    /// Toggle super seeding for torrent
    pub super_seeding: i64,
    /// Total downloaded amount for torrent
    pub total_downloaded: u64,
    /// Total uploaded amount for torrent
    pub total_uploaded: u64,
    /// Trackers list for torrent
    pub trackers: Vec<Vec<String>>,
    /// Toggle upload mode for torrent
    pub upload_mode: i64,
    /// Upload rate limit set on torrent
    pub upload_rate_limit: i64,
    /// List of url-seed URLs for torrent
    #[serde(rename = "url-list")]
    pub url_list: Vec<String>,
}

/// A subset of database columns needed for save path operations
#[derive(Serialize, Deserialize, Debug)]
pub struct FetchedPathData {
    /// Torrent database row id
    pub id: u64,
    /// Torrent hash id
    pub torrent_id: String,
    /// Save path for torrent content
    pub target_save_path: Option<String>,
    /// Binary blob containing fastresume data
    pub libtorrent_resume_data: Vec<u8>,
}

/// A subset of database columns needed for libtorrent_resume_data operations
#[derive(Serialize, Deserialize, Debug)]
pub struct FetchedLibtorrentResumeData {
    /// Torrent database row id
    pub id: u64,
    /// Torrent hash id
    pub torrent_id: String,
    ///Binary blob containing fastresume data
    pub libtorrent_resume_data: Vec<u8>,
}
