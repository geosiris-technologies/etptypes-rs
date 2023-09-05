// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use avro_rs::{Schema, Error};
use crate::helpers::EtpMessageBody;

use crate::energistics::etp::v12::datatypes::channel_data::truncate_info::TruncateInfo;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct TruncateChannels{

	#[serde(rename = "channels")]
    pub channels:Vec<TruncateInfo>,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.ChannelStreaming", "name": "TruncateChannels", "protocol": "1", "messageType": "5", "senderRole": "producer", "protocolRoles": "producer,consumer", "multipartFlag": false, "fields": [{"name": "channels", "type": {"type": "array", "items": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "TruncateInfo", "fields": [{"name": "channelId", "type": "long"}, {"name": "newEndIndex", "type": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "IndexValue", "fields": [{"name": "item", "type": ["null", "long", "double", {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "PassIndexedDepth", "fields": [{"name": "pass", "type": "long"}, {"name": "direction", "type": {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "PassDirection", "symbols": ["Up", "HoldingSteady", "Down"], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.PassDirection", "depends": []}}, {"name": "depth", "type": "double"}], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.PassIndexedDepth", "depends": ["Energistics.Etp.v12.Datatypes.ChannelData.PassDirection"]}]}], "fullName": "Energistics.Etp.v12.Datatypes.IndexValue", "depends": ["Energistics.Etp.v12.Datatypes.ChannelData.PassIndexedDepth"]}}], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.TruncateInfo", "depends": ["Energistics.Etp.v12.Datatypes.IndexValue"]}}}], "fullName": "Energistics.Etp.v12.Protocol.ChannelStreaming.TruncateChannels", "depends": ["Energistics.Etp.v12.Datatypes.ChannelData.TruncateInfo"]}"#;

impl EtpMessageBody for TruncateChannels{
    fn avro_schema() -> Option<Schema>{
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) ->i32{
        1
    }
    fn message_type(&self) ->i32{
        5
    }
    fn sender_role(&self) ->String{
        "producer".to_string()
    }
    fn protocol_roles(&self) ->String{
        "producer,consumer".to_string()
    }
    fn multipart_flag(&self) ->bool{
        false
    }
}

