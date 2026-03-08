use crate::extractor::error::ExtractionError;
use crate::extractor::patterns::{try_extract_id, YOUTUBE_VIDEO_REGEX};

/// Extracts a YouTube video ID from a URL or naked video ID.
///
/// This function accepts various YouTube URL formats including:
/// - Standard: `https://www.youtube.com/watch?v=VIDEO_ID`
/// - Short: `https://youtu.be/VIDEO_ID`
/// - Embed: `https://www.youtube.com/embed/VIDEO_ID`
/// - Direct paths: `https://www.youtube.com/v/VIDEO_ID`, `https://www.youtube.com/e/VIDEO_ID`
/// - Alternative domains: hooktube.com, deturl.com, etc.
/// - 60+ Invidious mirror sites (yewtu.be, invidious.pussthecat.org, etc.)
/// - Protocol-independent: `//youtube.com/watch?v=VIDEO_ID`
/// - Naked ID: just `VIDEO_ID`
///
/// # Arguments
///
/// * `url` - A string slice containing the YouTube URL or naked video ID
///
/// # Returns
///
/// * `Ok(String)` - The 11-character video ID if extraction is successful
/// * `Err(ExtractionError)` - If the URL is invalid or no video ID is found
///
/// # Examples
///
/// ```
/// use yt_playlist_bot::extract_video_id;
///
/// // Standard YouTube URL
/// let id = extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ").unwrap();
/// assert_eq!(id, "dQw4w9WgXcQ");
///
/// // Short URL
/// let id = extract_video_id("https://youtu.be/dQw4w9WgXcQ").unwrap();
/// assert_eq!(id, "dQw4w9WgXcQ");
///
/// // Naked ID
/// let id = extract_video_id("dQw4w9WgXcQ").unwrap();
/// assert_eq!(id, "dQw4w9WgXcQ");
/// ```
pub fn extract_video_id(url: &str) -> Result<String, ExtractionError> {
    // Try multiple regex patterns to extract the video ID
    let video_id = try_extract_id(url)
        .ok_or_else(|| ExtractionError::InvalidUrl(url.to_string()))?;

    // Defensive check: validate that the ID is exactly 11 characters
    // (the regex should already enforce this, but we verify it)
    if video_id.len() != 11 {
        return Err(ExtractionError::InvalidVideoIdLength {
            expected: 11,
            got: video_id.len(),
        });
    }

    // Return the validated video ID
    Ok(video_id)
}

/// Checks if a string is a valid YouTube URL without extracting the ID.
///
/// This is faster than `extract_video_id` when you only need to validate
/// the URL format without needing the actual video ID.
///
/// # Arguments
///
/// * `url` - A string slice containing the potential YouTube URL
///
/// # Returns
///
/// `true` if the URL matches a valid YouTube URL pattern, `false` otherwise
///
/// # Examples
///
/// ```
/// use yt_playlist_bot::is_valid_youtube_url;
///
/// assert!(is_valid_youtube_url("https://www.youtube.com/watch?v=dQw4w9WgXcQ"));
/// assert!(is_valid_youtube_url("https://youtu.be/dQw4w9WgXcQ"));
/// assert!(!is_valid_youtube_url("https://www.google.com"));
/// ```
pub fn is_valid_youtube_url(url: &str) -> bool {
    YOUTUBE_VIDEO_REGEX.is_match(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_standard_url() {
        let result = extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "dQw4w9WgXcQ");
    }

    #[test]
    fn test_extract_short_url() {
        let result = extract_video_id("https://youtu.be/dQw4w9WgXcQ");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "dQw4w9WgXcQ");
    }

    #[test]
    fn test_extract_embed_url() {
        let result = extract_video_id("https://www.youtube.com/embed/dQw4w9WgXcQ");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "dQw4w9WgXcQ");
    }

    #[test]
    fn test_extract_naked_id() {
        let result = extract_video_id("dQw4w9WgXcQ");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "dQw4w9WgXcQ");
    }

    #[test]
    fn test_invalid_url() {
        let result = extract_video_id("https://www.google.com");
        assert!(result.is_err());
        match result {
            Err(ExtractionError::InvalidUrl(_)) => {},
            _ => panic!("Expected InvalidUrl error"),
        }
    }

    #[test]
    fn test_empty_string() {
        let result = extract_video_id("");
        assert!(result.is_err());
    }

    #[test]
    fn test_is_valid_youtube_url() {
        assert!(is_valid_youtube_url("https://www.youtube.com/watch?v=dQw4w9WgXcQ"));
        assert!(is_valid_youtube_url("https://youtu.be/dQw4w9WgXcQ"));
        assert!(!is_valid_youtube_url("https://www.google.com"));
        assert!(!is_valid_youtube_url(""));
    }
}
