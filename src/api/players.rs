use crate::types::stats::RustlinkMock;
use crate::utils::{decode_track, logger, send_response};
use axum::{
    Json,
    extract::{OriginalUri, State},
    http::{HeaderMap, Method, StatusCode},
    response::Response,
};
use chrono::Utc;
use serde_json::json;
use std::sync::{Arc, RwLock};

pub async fn handler(
    State(rustlink): State<Arc<RwLock<RustlinkMock>>>,
    method: Method,
    headers: HeaderMap,
    OriginalUri(uri): OriginalUri,
    body: Option<Json<serde_json::Value>>,
) -> Response {
    let path = uri.path();
    let parts: Vec<&str> = path.split('/').collect();

    // JS: parts[3] refers to sessionId, parts[5] refers to guildId
    let session_id = parts.get(3).copied();
    let guild_id = parts.get(5).copied();

    let mut rustlink_write = rustlink.write().unwrap();

    let session_id = match session_id {
        Some(id) => id,
        None => {
            return send_error(
                &headers,
                "Session ID not found in path",
                StatusCode::BAD_REQUEST,
                path,
            );
        }
    };

    let session = match rustlink_write.sessions.get_mut(session_id) {
        Some(s) => s,
        None => {
            return send_response(
                &headers,
                Some(json!({
                    "timestamp": Utc::now().timestamp_millis(),
                    "status": 404,
                    "error": "Not Found",
                    "message": "The provided sessionId doesn't exist.",
                    "path": path
                })),
                StatusCode::NOT_FOUND,
                false,
            );
        }
    };

    if guild_id.is_none() && path.contains("/players") {
        if method == Method::GET {
            // JS: const players = await Promise.all(...)
            // Here we just return the players HashMap values
            let players: Vec<serde_json::Value> = session
                .players
                .players
                .values()
                .map(|p| {
                    // Mocking toJSON logic
                    json!({ "guildId": p.guild_id, "connection": p.connection })
                })
                .collect();

            return send_response(&headers, Some(json!(players)), StatusCode::OK, false);
        }
    }

    if let Some(guild_id) = guild_id {
        if method == Method::GET {
            // JS: await session.players.create(guildId)
            // Mocking create logic (upsert)
            if !session.players.players.contains_key(guild_id) {
                session.players.players.insert(
                    guild_id.to_string(),
                    crate::types::stats::Player {
                        guild_id: guild_id.to_string(),
                        connection: None,
                    },
                );
            }

            let player = session.players.players.get(guild_id).unwrap();
            let player_json =
                json!({ "guildId": player.guild_id, "connection": player.connection });
            return send_response(&headers, Some(player_json), StatusCode::OK, false);
        }

        if method == Method::DELETE {
            if session.players.players.remove(guild_id).is_some() {
                return send_response(&headers, None, StatusCode::NO_CONTENT, false);
            } else {
                return send_response(
                    &headers,
                    Some(json!({
                        "timestamp": Utc::now().timestamp_millis(),
                        "status": 404,
                        "error": "Not Found",
                        "message": "Player not found",
                        "path": path
                    })),
                    StatusCode::NOT_FOUND,
                    false,
                );
            }
        }

        if method == Method::PATCH {
            let payload = match body {
                Some(Json(v)) => v,
                None => return send_error(&headers, "Empty body", StatusCode::BAD_REQUEST, path),
            };

            logger(
                "debug",
                "PlayerUpdate",
                &format!("Received payload for guild {}:", guild_id),
                None,
            );

            // Ensure player exists
            if !session.players.players.contains_key(guild_id) {
                session.players.players.insert(
                    guild_id.to_string(),
                    crate::types::stats::Player {
                        guild_id: guild_id.to_string(),
                        connection: None,
                    },
                );
            }

            if let Some(voice) = payload.get("voice") {
                let endpoint = voice.get("endpoint").and_then(|v| v.as_str());
                let token = voice.get("token").and_then(|v| v.as_str());
                let voice_session_id = voice.get("sessionId").and_then(|v| v.as_str());

                if endpoint.is_none() || token.is_none() || voice_session_id.is_none() {
                    logger(
                        "warn",
                        "PlayerUpdate",
                        &format!("Received invalid voice object for guild {}:", guild_id),
                        Some(voice),
                    );
                    return send_error(
                        &headers,
                        "Invalid voice object. Endpoint, token, and sessionId are required.",
                        StatusCode::BAD_REQUEST,
                        path,
                    );
                }

                logger(
                    "debug",
                    "PlayerUpdate",
                    &format!("Updating voice for guild {}:", guild_id),
                    Some(voice),
                );
                // Mock updating voice
            }

            if let Some(encoded_track_deprecated) = payload.get("encodedTrack") {
                logger(
                    "warn",
                    "PlayerUpdate",
                    "The `encodedTrack` field is deprecated. Use `track.encoded` instead.",
                    None,
                );
            }

            if let Some(track) = payload.get("track") {
                if let Some(encoded_track) = track.get("encoded") {
                    if encoded_track.is_null() {
                        logger(
                            "debug",
                            "PlayerUpdate",
                            &format!("Stopping player for guild {}", guild_id),
                            None,
                        );
                        // Mock stop
                    } else if let Some(encoded_str) = encoded_track.as_str() {
                        let no_replace = uri
                            .query()
                            .map(|q| q.contains("noReplace=true"))
                            .unwrap_or(false);
                        match decode_track(encoded_str) {
                            Ok(decoded) => {
                                logger(
                                    "debug",
                                    "PlayerUpdate",
                                    &format!("Playing track for guild {}:", guild_id),
                                    Some(
                                        &json!({ "track": decoded.info, "noReplace": no_replace }),
                                    ),
                                );
                                // Mock play
                            }
                            Err(_) => {
                                logger(
                                    "warn",
                                    "PlayerUpdate",
                                    &format!(
                                        "Received invalid track for guild {}: {}",
                                        guild_id, encoded_str
                                    ),
                                    None,
                                );
                                return send_error(
                                    &headers,
                                    "The provided track is invalid.",
                                    StatusCode::BAD_REQUEST,
                                    path,
                                );
                            }
                        }
                    }
                }
            }

            if let Some(volume) = payload.get("volume").and_then(|v| v.as_u64()) {
                if volume > 1000 {
                    logger(
                        "warn",
                        "PlayerUpdate",
                        &format!(
                            "Received invalid volume for guild {}: {}. Expected 0-1000.",
                            guild_id, volume
                        ),
                        None,
                    );
                    return send_error(
                        &headers,
                        "The volume must be between 0 and 1000.",
                        StatusCode::BAD_REQUEST,
                        path,
                    );
                }
                logger(
                    "debug",
                    "PlayerUpdate",
                    &format!("Setting volume to {} for guild {}", volume, guild_id),
                    None,
                );
                // Mock volume
            }

            if let Some(paused) = payload.get("paused") {
                if !paused.is_boolean() {
                    logger(
                        "warn",
                        "PlayerUpdate",
                        &format!(
                            "Received invalid paused value for guild {}: {}. Expected boolean.",
                            guild_id, paused
                        ),
                        None,
                    );
                    return send_error(
                        &headers,
                        "The paused value must be a boolean.",
                        StatusCode::BAD_REQUEST,
                        path,
                    );
                }
                logger(
                    "debug",
                    "PlayerUpdate",
                    &format!("Setting paused to {} for guild {}", paused, guild_id),
                    None,
                );
                // Mock pause
            }

            if let Some(position) = payload.get("position") {
                if !position.is_number() {
                    logger(
                        "warn",
                        "PlayerUpdate",
                        &format!(
                            "Received invalid position for guild {}: {}. Expected number.",
                            guild_id, position
                        ),
                        None,
                    );
                    return send_error(
                        &headers,
                        "The position value must be a number.",
                        StatusCode::BAD_REQUEST,
                        path,
                    );
                }
                logger(
                    "debug",
                    "PlayerUpdate",
                    &format!("Seeking to {}ms for guild {}", position, guild_id),
                    None,
                );
                // Mock seek
            }

            if let Some(filters) = payload.get("filters") {
                if !filters.is_object() {
                    logger(
                        "warn",
                        "PlayerUpdate",
                        &format!(
                            "Received invalid filters value for guild {}: {}. Expected object.",
                            guild_id, filters
                        ),
                        None,
                    );
                    return send_error(
                        &headers,
                        "The filters value must be an object.",
                        StatusCode::BAD_REQUEST,
                        path,
                    );
                }
                logger(
                    "debug",
                    "PlayerUpdate",
                    &format!("Applying filters for guild {}:", guild_id),
                    Some(filters),
                );
                // Mock setFilters
            }

            let player = session.players.players.get(guild_id).unwrap();
            let player_json =
                json!({ "guildId": player.guild_id, "connection": player.connection });
            return send_response(&headers, Some(player_json), StatusCode::OK, false);
        }
    }

    send_error(
        &headers,
        "The requested player endpoint was not found.",
        StatusCode::NOT_FOUND,
        path,
    )
}

fn send_error(headers: &HeaderMap, message: &str, status: StatusCode, path: &str) -> Response {
    send_response(
        headers,
        Some(json!({
            "timestamp": Utc::now().timestamp_millis(),
            "status": status.as_u16(),
            "error": status.canonical_reason().unwrap_or("Unknown"),
            "message": message,
            "path": path
        })),
        status,
        false,
    )
}
