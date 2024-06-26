use serde::Deserialize;

use super::rest::LavalinkException;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkReadyEvent {
    pub resumed: bool,
    pub session_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkTrackStartEvent {
    pub guild_id: String,
    pub encoded_track: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkTrackEndEvent {
    pub guild_id: String,
    pub encoded_track: String,
    pub reason: LavalinkTrackEndReason,
}

#[derive(Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LavalinkTrackEndReason {
    Finished,
    LoadFailed,
    Stopped,
    Replaced,
    Cleanup,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkTrackExceptionEvent {
    pub encoded_track: String,
    pub exception: LavalinkException,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkTrackStuckEvent {
    pub encoded_track: String,
    pub threshold_ms: i32,
}
