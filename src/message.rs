use std::option::Option;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    #[serde(rename = "smartlife.iot.smartbulb.lightingservice")]
    LightingServiceMsg(LightingServiceMsg),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LightingServiceMsg {
    #[serde(rename = "get_light_details")]
    GetLightDetails(Option<LightDetails>),
    #[serde(rename = "transition_light_state")]
    TransitionLightOnOff(TransitionLightOnOff),
    #[serde(rename = "transition_light_state")]
    TransitionLightState(TransitionLightState),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransitionLightState {
    pub on_off: u8,
    pub error_code: u8
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransitionLightOnOff {
    pub on_off: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LightDetails {
    pub lamp_beam_angle: u16,
    pub min_voltage: u16,
    pub max_voltage: u16,
    pub wattage: u16,
    pub incandescent_equivalent: u16,
    pub max_lumens: u16,
    pub color_rendering_index: u8,
    pub err_code: u64
}
