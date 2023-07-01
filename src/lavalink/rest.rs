use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkVoiceState {
    pub token: String,
    pub endpoint: String,
    pub session_id: String,

    #[serde(skip_serializing)]
    pub connected: bool,
    #[serde(skip_serializing)]
    pub ping: i32
}

impl LavalinkVoiceState {
    pub fn new(token: &str, endpoint: &str, session_id: &str) -> Self {
        Self {
            token: token.to_owned(),
            endpoint: endpoint.to_owned(),
            session_id: session_id.to_owned(),
            connected: Default::default(),
            ping: Default::default()
        }
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkUpdatePlayer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_track: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<LavalinkVoiceState>
}

impl LavalinkUpdatePlayer {
    pub fn new() -> Self {
        Self {
            encoded_track: None,
            identifier: None,
            end_time: None,
            paused: None,
            position: None,
            voice: None,
            volume: None
        }
    }
    
    pub fn identifier(mut self, identifier: &str) -> Self {
        if self.encoded_track == None {
            self.identifier = Some(identifier.to_owned());
        }

        self
    }

    pub fn voice_state(&mut self, voice_state: LavalinkVoiceState) {
        self.voice = Some(voice_state);
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkPlayer {
    pub guild_id: String,
    pub track: Option<LavalinkTrack>,
    pub volume: i32,
    pub paused: bool,
    pub voice: LavalinkVoiceState
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkTrack {
    pub encoded: String,
    pub track: String,
    pub info: LavalinkTrackInfo
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkTrackInfo {
    pub identifier: String,
    pub is_seekable: bool,
    pub author: String,
    pub length: i32,
    pub is_stream: bool,
    pub position: i32,
    pub title: String,
    pub uri: Option<String>,
    pub source_name: String
}