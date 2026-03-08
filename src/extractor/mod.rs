/// Error types for video extraction
pub mod error;

/// Regex patterns and domain lists for YouTube URL matching
pub mod patterns;

/// Core video ID extraction functionality
pub mod video_id;

pub use video_id::{extract_video_id, is_valid_youtube_url};
