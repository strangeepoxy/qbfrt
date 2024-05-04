//! SQLite fastresume data module

use serde_derive::{Deserialize, Serialize};

/// qB SQLite data
#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseData {
    /// Database row id
    pub id: u8,
    /// Torrent id / info hash
    pub torrent_id: String,
    /// Torrent queue position
    pub queue_position: i8,
    /// Torrent name (unused?)
    pub name: Option<String>,
    /// Torrent category
    pub category: Option<String>,
    /// Comma-separated torrent tags
    pub tags: Option<String>,
    /// Save path for torrent contents
    pub target_save_path: Option<String>,
    /// Download path
    pub download_path: Option<String>,
    /// Torrent content layout
    pub content_layout: String,
    /// Ratio limit for seeding
    pub ratio_limit: i32,
    /// Seed time limit
    pub seeding_time_limit: i32,
    /// Inactivity time limit
    pub inactive_seeding_time_limit: i32,
    /// Prioritize outer pieces
    pub has_outer_pieces_priority: u8,
    /// Seed status
    pub has_seed_status: u8,
    /// Torrent management mode
    pub operating_mode: String,
    /// Torrent stopped status
    pub stopped: u8,
    /// Stop condition for torrents
    pub stop_condition: String,
    /// Binary blob containing fastresume data
    pub libtorrent_resume_data: Vec<u8>,
    /// Binary blob containing .torrent file metadata
    pub metadata: Vec<u8>,
}
