use serde::Deserialize;

use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTitlePayload {
    pub title: String,
    pub target: String,
    pub state: usize,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetStatePayload {
    pub state: usize,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetImagePayload {
    pub image: String,
    // pub target: String,
    pub state: usize,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "event")]
pub enum ClientRequest {
    RegisterPlugin {
        uuid: String,
    },
    SetTitle {
        context: String,
        payload: SetTitlePayload,
    },
    SetState {
        context: String,
        payload: SetStatePayload,
    },
    SetImage {
        context: String,
        payload: SetImagePayload,
    },
}
