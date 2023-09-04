#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;


#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct ChannelChangeRequestInfo{

	#[serde(rename = "sinceChangeTime")]
    pub since_change_time:i64,


	#[serde(rename = "channelIds")]
    pub channel_ids:Vec<i64>,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "ChannelChangeRequestInfo", "fields": [{"name": "sinceChangeTime", "type": "long"}, {"name": "channelIds", "type": {"type": "array", "items": "long"}}], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.ChannelChangeRequestInfo", "depends": []}"#;

