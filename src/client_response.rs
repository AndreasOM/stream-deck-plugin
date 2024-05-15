use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
//#[serde(rename_all = "camelCase")]
pub enum Controller {
    Keypad,
    Encoder,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinates {
    column: u8,
    pub row: u8,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
enum Event {
    DeviceDidConnect,
    KeyUp,
    KeyDown,
    WillAppear,
    TitleParametersDidChange,

    SetTitle,
    SetState,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientResponseWillAppearPayload {
    pub settings: serde_json::Value,
    pub coordinates: Coordinates,
    pub controller: Controller,
    pub state: u8,
    pub is_in_multi_action: bool,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientResponseKeyUpPayload {
    pub settings: serde_json::Value,
    pub coordinates: Coordinates,
    pub state: usize,
    pub user_desired_state: Option<u8>,
    pub is_in_multi_action: bool,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "event")]
pub enum ClientResponse {
    KeyUp {
        action: String,
        //event: Event,
        context: String,
        device: String,
        payload: ClientResponseKeyUpPayload,
    },
    WillAppear {
        action: String,
        //event: Event,
        context: String,
        device: String,
        payload: ClientResponseWillAppearPayload,
    },
}
