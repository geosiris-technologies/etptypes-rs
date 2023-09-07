// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::*;
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::SystemTime;

use crate::energistics::etp::v12::datatypes::index_value::IndexValue;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct ChannelSubscribeInfo {
    #[serde(rename = "channelId")]
    pub channel_id: i64,

    #[serde(rename = "startIndex")]
    pub start_index: IndexValue,

    #[serde(rename = "dataChanges")]
    #[derivative(Default(value = "true"))]
    pub data_changes: bool,

    #[serde(rename = "requestLatestIndexCount")]
    pub request_latest_index_count: Option<i32>,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "ChannelSubscribeInfo", "fields": [{"name": "channelId", "type": "long"}, {"name": "startIndex", "type": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "IndexValue", "fields": [{"name": "item", "type": ["null", "long", "double", {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "PassIndexedDepth", "fields": [{"name": "pass", "type": "long"}, {"name": "direction", "type": {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "PassDirection", "symbols": ["Up", "HoldingSteady", "Down"], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.PassDirection", "depends": []}}, {"name": "depth", "type": "double"}], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.PassIndexedDepth", "depends": ["Energistics.Etp.v12.Datatypes.ChannelData.PassDirection"]}]}], "fullName": "Energistics.Etp.v12.Datatypes.IndexValue", "depends": ["Energistics.Etp.v12.Datatypes.ChannelData.PassIndexedDepth"]}}, {"name": "dataChanges", "type": "boolean", "default": true}, {"name": "requestLatestIndexCount", "type": ["null", "int"]}], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.ChannelSubscribeInfo", "depends": ["Energistics.Etp.v12.Datatypes.IndexValue"]}"#;

impl ChannelSubscribeInfo {
    /* Protocol , MessageType :  */
    pub fn default_with_params(
        channel_id: i64,
        start_index: IndexValue,
        request_latest_index_count: Option<i32>,
    ) -> ChannelSubscribeInfo {
        ChannelSubscribeInfo {
            channel_id,
            start_index,
            data_changes: true,
            request_latest_index_count,
        }
    }
}
