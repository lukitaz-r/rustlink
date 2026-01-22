use serde_json::{json, Value};
use regex::Regex;
use once_cell::sync::Lazy;
use crate::types::tracks::{TrackInfo, DecodedTrack};
use crate::utils::{encode_track, logger};

pub enum YoutubeUrlType {
    Video,
    Playlist,
    Shorts,
    Unknown,
}

pub fn check_url_type(url: &str, is_yt_music: bool) -> YoutubeUrlType {
    static VIDEO_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^https?://(?:www\.)?youtube\.com/watch\?v=[\w-]+").unwrap());
    static MUSIC_VIDEO_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^https?://music\.youtube\.com/watch\?v=[\w-]+").unwrap());
    static PLAYLIST_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^https?://(?:www\.)?youtube\.com/playlist\?list=[\w-]+").unwrap());
    static MUSIC_PLAYLIST_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^https?://music\.youtube\.com/playlist\?list=[\w-]+").unwrap());
    static SHORT_URL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^https?://youtu\.be/[\w-]+").unwrap());
    static SHORTS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^https?://(?:www\.)?youtube\.com/shorts/[\w-]+").unwrap());

    let (v_reg, p_reg) = if is_yt_music { (&*MUSIC_VIDEO_REGEX, &*MUSIC_PLAYLIST_REGEX) } else { (&*VIDEO_REGEX, &*PLAYLIST_REGEX) };

    if p_reg.is_match(url) || (v_reg.is_match(url) && url.contains("&list=")) {
        return YoutubeUrlType::Playlist;
    }
    if v_reg.is_match(url) {
        return YoutubeUrlType::Video;
    }
    if !is_yt_music {
        if SHORTS_REGEX.is_match(url) { return YoutubeUrlType::Shorts; }
        if SHORT_URL_REGEX.is_match(url) { return YoutubeUrlType::Video; }
    }
    YoutubeUrlType::Unknown
}

pub fn build_track(
    item_data: &Value,
    source_name: &str,
    full_api_response: Option<&Value>,
) -> Option<DecodedTrack> {
    let mut video_id = None;
    let mut title = String::from("Unknown Title");
    let mut author = String::from("Unknown Author");
    let mut length_ms = 0;
    let mut is_stream = true;
    let mut artwork_url = None;
    let mut uri = String::new();

    let is_yt_music = source_name == "ytmusic";

    if is_yt_music {
        let renderer = item_data.get("musicResponsiveListItemRenderer")
            .or_else(|| item_data.get("playlistPanelVideoRenderer"))
            .or_else(|| item_data.get("musicTwoColumnItemRenderer"));
        
        if let Some(r) = renderer {
            video_id = r.get("videoId").and_then(|v| v.as_str())
                .or_else(|| r.get("playlistItemData").and_then(|v| v.get("videoId")).and_then(|v| v.as_str()))
                .or_else(|| r.get("navigationEndpoint").and_then(|v| v.get("watchEndpoint")).and_then(|v| v.get("videoId")).and_then(|v| v.as_str()))
                .map(|s| s.to_string());
            
            // Logic for title, author, length, etc. simplified for now
            // ...
        }
    } else {
        let renderer = item_data.get("videoRenderer")
            .or_else(|| item_data.get("compactVideoRenderer"))
            .or_else(|| item_data.get("playlistPanelVideoRenderer"))
            .or_else(|| item_data.get("gridVideoRenderer"))
            .unwrap_or(item_data);
        
        video_id = renderer.get("videoId").and_then(|v| v.as_str()).map(|s| s.to_string());
        // Simplified mapping...
    }

    let vid = video_id?;
    uri = if is_yt_music { format!("https://music.youtube.com/watch?v={}", vid) } else { format!("https://www.youtube.com/watch?v={}", vid) };

    let track_info = TrackInfo {
        identifier: vid,
        is_seekable: !is_stream,
        author,
        length: length_ms,
        is_stream,
        position: 0,
        title,
        uri: Some(uri),
        artwork_url,
        isrc: None,
        source_name: source_name.to_string(),
    };

    let encoded = match encode_track(&track_info) {
        Ok(e) => e,
        Err(_) => return None,
    };

    Some(DecodedTrack {
        encoded,
        info: track_info,
    })
}
