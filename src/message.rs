use std::option::Option;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    #[serde(rename = "system")]
    SystemMsg(SystemMsg),
    #[serde(rename = "smartlife.iot.smartbulb.lightingservice")]
    LightingServiceMsg(LightingServiceMsg),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SystemMsg {
    #[serde(rename = "get_sysinfo")]
    GetSysinfo(Option<SysInfo>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SysInfo {
    pub sw_ver: String,
    pub hw_ver: String,
    pub model: String,
    pub description: String,
    pub alias: String,
    pub mic_type: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "oemId")]
    pub oem_id: String,
    #[serde(rename = "hwId")]
    pub hw_id: String,
    pub is_factory: bool,
    pub disco_ver: String,
    pub ctrl_protocols: CtrlProtocols,
    pub light_state: LightState,
    pub is_dimmable: u8,
    pub is_color: u8,
    pub is_variable_color_temp: u8,
    pub preferred_state: Vec<LightState>,
    pub rssi: i8,
    pub active_mode: String,
    pub heapsize: u64,
    pub err_code: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CtrlProtocols {
    name: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LightState {
    on_off: Option<u8>,
    mode: Option<String>,
    hue: u64,
    saturation: u16,
    color_temp: u8,
    brightness: u8,
    error_code: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum LightingServiceMsg {
    GetLightDetails(Option<LightDetails>),
    TransitionLightState(TransitionLightState),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TransitionLightState {
    TransitionLightFull(TransitionLightFull),
    TransitionLightOnOff(TransitionLightOnOff),
    TransitionLightHsv(TransitionLightHsv),
    TransitionLightTemp(TransitionLightTemp),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransitionLightFull {
    pub on_off: u8,
    pub error_code: u8
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransitionLightOnOff {
    pub on_off: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransitionLightHsv {
    pub hue: u16,
    pub saturation: u8,
    pub brightness: u8,
    pub on_off: u8,
    pub color_temp: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransitionLightTemp {
    pub color_temp: u16,
    pub brightness: u8,
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
