// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::EtpMessageBody;
use avro_rs::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;

use crate::energistics::etp::v12::datatypes::channel_data::channel_range_info::ChannelRangeInfo;
use crate::energistics::etp::v12::datatypes::uuid::Uuid;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct GetRanges {
    #[serde(rename = "requestUuid")]
    pub request_uuid: Uuid,

    #[serde(rename = "channelRanges")]
    pub channel_ranges: Vec<ChannelRangeInfo>,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.ChannelSubscribe", "name": "GetRanges", "protocol": "21", "messageType": "9", "senderRole": "customer", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "requestUuid", "type": {"type": "fixed", "namespace": "Energistics.Etp.v12.Datatypes", "name": "Uuid", "size": 16, "fullName": "Energistics.Etp.v12.Datatypes.Uuid", "depends": []}}, {"name": "channelRanges", "type": {"type": "array", "items": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "ChannelRangeInfo", "fields": [{"name": "channelIds", "type": {"type": "array", "items": "long"}}, {"name": "interval", "type": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "IndexInterval", "fields": [{"name": "startIndex", "type": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "IndexValue", "fields": [{"name": "item", "type": ["null", "long", "double", {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "PassIndexedDepth", "fields": [{"name": "pass", "type": "long"}, {"name": "direction", "type": {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "PassDirection", "symbols": ["Up", "HoldingSteady", "Down"], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.PassDirection", "depends": []}}, {"name": "depth", "type": "double"}], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.PassIndexedDepth", "depends": ["Energistics.Etp.v12.Datatypes.ChannelData.PassDirection"]}]}], "fullName": "Energistics.Etp.v12.Datatypes.IndexValue", "depends": ["Energistics.Etp.v12.Datatypes.ChannelData.PassIndexedDepth"]}}, {"name": "endIndex", "type": "Energistics.Etp.v12.Datatypes.IndexValue"}, {"name": "uom", "type": "string"}, {"name": "depthDatum", "type": "string", "default": ""}], "fullName": "Energistics.Etp.v12.Datatypes.Object.IndexInterval", "depends": ["Energistics.Etp.v12.Datatypes.IndexValue"]}}, {"name": "secondaryIntervals", "type": {"type": "array", "items": "Energistics.Etp.v12.Datatypes.Object.IndexInterval"}, "default": []}], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.ChannelRangeInfo", "depends": ["Energistics.Etp.v12.Datatypes.Object.IndexInterval"]}}}], "fullName": "Energistics.Etp.v12.Protocol.ChannelSubscribe.GetRanges", "depends": ["Energistics.Etp.v12.Datatypes.Uuid", "Energistics.Etp.v12.Datatypes.ChannelData.ChannelRangeInfo"]}"#;

impl EtpMessageBody for GetRanges {
    fn avro_schema() -> Option<Schema> {
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) -> i32 {
        21
    }
    fn message_type(&self) -> i32 {
        9
    }
    fn sender_role(&self) -> String {
        "customer".to_string()
    }
    fn protocol_roles(&self) -> String {
        "store,customer".to_string()
    }
    fn multipart_flag(&self) -> bool {
        false
    }
}
