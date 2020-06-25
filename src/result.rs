use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub(crate) struct JsonResult {
    #[serde(default, rename = "Result")]
    result: HashMap<String, String>,
    #[serde(default, rename = "GameTime")]
    game_time: u32,
    #[serde(default, rename = "GameTimeSeconds")]
    game_time_seconds: f64,
    #[serde(default, rename = "GameTimeFormatted")]
    game_time_formatted: String,
    #[serde(default, rename = "AverageFrameTime")]
    average_frame_time: HashMap<String, f32>,
    #[serde(default, rename = "Status")]
    status: String,
}
impl JsonResult {
    pub(crate) fn from(
        result: Option<HashMap<String, String>>,
        game_time: Option<u32>,
        game_time_seconds: Option<f64>,
        game_time_formatted: Option<String>,
        average_frame_time: Option<HashMap<String, f32>>,
        status: Option<String>,
    ) -> Self {
        Self {
            result: result.unwrap_or_default(),
            game_time: game_time.unwrap_or_default(),
            game_time_seconds: game_time_seconds.unwrap_or_default(),
            game_time_formatted: game_time_formatted.unwrap_or_default(),
            average_frame_time: average_frame_time.unwrap_or_default(),
            status: status.unwrap_or_default(),
        }
    }
    pub(crate) fn serialize(&self) -> String {
        serde_json::to_string(&self).expect("Could not serialize Result")
    }
}
