// TIPOS
use crate::types;

// FUNCIONES
#[path = "initLogger.rs"]
mod internal_logger;

#[path = "validateProperty.rs"]
mod validate_property;

#[path = "checkForUpdates.rs"]
mod check_for_updates;

#[path = "logger.rs"]
mod logger;

#[path = "getVersion.rs"]
mod version;

#[path = "parseSemver.rs"]
mod parse_semver;

#[path = "getStats.rs"]
mod get_stats;

#[path = "decodeTrack.rs"]
mod decode_track;

#[path = "encodeTrack.rs"]
mod encode_track;

#[path = "parseClient.rs"]
mod parse_client;

#[path = "generateRandomLetters.rs"]
mod generate_random_letters;

#[path = "verifyDiscordID.rs"]
mod verify_discord_id;

#[path = "verifyMethod.rs"]
mod verify_method;

#[path = "makeRequest.rs"]
mod make_request;

#[path = "sendResponse.rs"]
pub mod send_response;

#[path = "sendErrorResponse.rs"]
mod send_error_response;

#[path = "http1MakeRequest.rs"]
pub mod http1_make_request;

#[path = "loadHLS.rs"]
pub mod load_hls;

#[path = "loadHLSPlaylist.rs"]
pub mod load_hls_playlist;

// DEFINICIÓN DE MODULOS
pub use check_for_updates::check_for_updates;
pub use decode_track::decode_track;
pub use encode_track::encode_track;
pub use generate_random_letters::generate_random_letters;
pub use get_stats::get_stats;
pub use http1_make_request::http1_make_request;
pub use internal_logger::init_logger;
pub use load_hls::load_hls;
pub use load_hls_playlist::load_hls_playlist;
pub use logger::logger;
pub use make_request::make_request;
pub use parse_client::parse_client;
pub use parse_semver::parse_semver;
pub use send_error_response::send_error_response;
pub use send_response::send_response;
pub use validate_property::validate_property;
pub use verify_discord_id::verify_discord_id;
pub use verify_method::verify_method;
pub use version::VersionResult;
pub use version::get_version;
