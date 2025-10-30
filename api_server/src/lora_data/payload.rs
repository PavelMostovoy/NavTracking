use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub struct UplinkPayload {
    #[serde(rename = "deduplicationId", default)]
    deduplication_id: Option<String>,
    
    pub(crate) time: String,

    #[serde(rename = "deviceInfo")]
    pub(crate) device_info: DeviceInfo,

    #[serde(rename = "devAddr", default)]
    dev_addr: Option<String>,

    #[serde(default)]
    adr: Option<bool>,

    #[serde(default)]
    dr: Option<u32>,

    #[serde(rename = "fCnt", default)]
    pub(crate) f_cnt: Option<u32>,

    #[serde(rename = "fPort", default)]
    f_port: Option<u32>,

    #[serde(default)]
    confirmed: Option<bool>,
    
    pub(crate) data: String,

    #[serde(rename = "rxInfo", default)]
    rx_info: Option<Vec<RxInfo>>,

    #[serde(rename = "txInfo", default)]
    tx_info: Option<TxInfo>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct DeviceInfo {
    #[serde(rename = "tenantId", default)]
    tenant_id: Option<String>,

    #[serde(rename = "tenantName", default)]
    tenant_name: Option<String>,

    #[serde(rename = "applicationId", default)]
    application_id: Option<String>,

    #[serde(rename = "applicationName", default)]
    application_name: Option<String>,

    #[serde(rename = "deviceProfileId", default)]
    device_profile_id: Option<String>,

    #[serde(rename = "deviceProfileName", default)]
    device_profile_name: Option<String>,

    #[serde(rename = "deviceName")]
    pub(crate) device_name: String,

    #[serde(rename = "devEui")]
    pub(crate) dev_eui: String,

    #[serde(rename = "deviceClassEnabled", default)]
    device_class_enabled: Option<String>,

    #[serde(default)]
    tags: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RxInfo {
    #[serde(rename = "gatewayId", default)]
    gateway_id: Option<String>,

    #[serde(rename = "uplinkId", default)]
    uplink_id: Option<u64>,

    #[serde(rename = "nsTime", default)]
    ns_time: Option<String>,

    #[serde(default)]
    rssi: Option<i32>,

    #[serde(default)]
    snr: Option<f32>,

    #[serde(default)]
    channel: Option<u32>,

    #[serde(rename = "rfChain", default)]
    rf_chain: Option<u32>,

    #[serde(default)]
    context: Option<String>,

    #[serde(default)]
    metadata: Option<Metadata>,

    #[serde(rename = "crcStatus", default)]
    crc_status: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Metadata {
    #[serde(default)]
    region_config_id: Option<String>,

    #[serde(default)]
    region_common_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TxInfo {
    #[serde(default)]
    frequency: Option<u64>,

    #[serde(default)]
    modulation: Option<Modulation>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Modulation {
    #[serde(default)]
    lora: Option<Lora>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Lora {
    #[serde(default)]
    bandwidth: Option<u32>,

    #[serde(rename = "spreadingFactor", default)]
    spreading_factor: Option<u32>,

    #[serde(rename = "codeRate", default)]
    code_rate: Option<String>,
}
