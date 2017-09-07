use std::option::Option;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    #[serde(rename = "smartlife.iot.smartbulb.lightingservice")]
    LightingServiceMsg(LightingServiceMsg),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LightingServiceMsg {
    #[serde(rename = "get_light_details")]
    GetLightDetails(Option<()>),
    #[serde(rename = "transition_light_state")]
    TransitionLightState(TransitionLightState),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransitionLightState {
    pub on_off: u8,
}
