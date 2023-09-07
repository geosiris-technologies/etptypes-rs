// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::ETPMetadata;
use crate::helpers::*;
use avro_rs::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::SystemTime;

use crate::energistics::etp::v12::datatypes::channel_data::index_metadata_record::IndexMetadataRecord;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct GetFrameResponseHeader {
    #[serde(rename = "channelUris")]
    pub channel_uris: Vec<String>,

    #[serde(rename = "indexes")]
    pub indexes: Vec<IndexMetadataRecord>,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.ChannelDataFrame", "name": "GetFrameResponseHeader", "protocol": "2", "messageType": "4", "senderRole": "store", "protocolRoles": "store,customer", "multipartFlag": true, "fields": [{"name": "channelUris", "type": {"type": "array", "items": "string"}}, {"name": "indexes", "type": {"type": "array", "items": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "IndexMetadataRecord", "fields": [{"name": "indexKind", "type": {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "ChannelIndexKind", "symbols": ["DateTime", "ElapsedTime", "MeasuredDepth", "TrueVerticalDepth", "PassIndexedDepth", "Pressure", "Temperature", "Scalar"], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.ChannelIndexKind", "depends": []}, "default": "DateTime"}, {"name": "interval", "type": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "IndexInterval", "fields": [{"name": "startIndex", "type": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "IndexValue", "fields": [{"name": "item", "type": ["null", "long", "double", {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "PassIndexedDepth", "fields": [{"name": "pass", "type": "long"}, {"name": "direction", "type": {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "PassDirection", "symbols": ["Up", "HoldingSteady", "Down"], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.PassDirection", "depends": []}}, {"name": "depth", "type": "double"}], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.PassIndexedDepth", "depends": ["Energistics.Etp.v12.Datatypes.ChannelData.PassDirection"]}]}], "fullName": "Energistics.Etp.v12.Datatypes.IndexValue", "depends": ["Energistics.Etp.v12.Datatypes.ChannelData.PassIndexedDepth"]}}, {"name": "endIndex", "type": "Energistics.Etp.v12.Datatypes.IndexValue"}, {"name": "uom", "type": "string"}, {"name": "depthDatum", "type": "string", "default": ""}], "fullName": "Energistics.Etp.v12.Datatypes.Object.IndexInterval", "depends": ["Energistics.Etp.v12.Datatypes.IndexValue"]}}, {"name": "direction", "type": {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "IndexDirection", "symbols": ["Increasing", "Decreasing", "Unordered"], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.IndexDirection", "depends": []}, "default": "Increasing"}, {"name": "name", "type": "string", "default": ""}, {"name": "uom", "type": "string"}, {"name": "depthDatum", "type": "string", "default": ""}, {"name": "indexPropertyKindUri", "type": "string"}, {"name": "filterable", "type": "boolean", "default": true}], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.IndexMetadataRecord", "depends": ["Energistics.Etp.v12.Datatypes.ChannelData.ChannelIndexKind", "Energistics.Etp.v12.Datatypes.Object.IndexInterval", "Energistics.Etp.v12.Datatypes.ChannelData.IndexDirection"]}}}], "fullName": "Energistics.Etp.v12.Protocol.ChannelDataFrame.GetFrameResponseHeader", "depends": ["Energistics.Etp.v12.Datatypes.ChannelData.IndexMetadataRecord"]}"#;

impl ETPMetadata for GetFrameResponseHeader {
    fn avro_schema() -> Option<Schema> {
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) -> i32 {
        2
    }
    fn message_type(&self) -> i32 {
        4
    }
    fn sender_role(&self) -> Vec<Role> {
        vec![Role::Store]
    }
    fn protocol_roles(&self) -> Vec<Role> {
        vec![Role::Store, Role::Customer]
    }
    fn multipart_flag(&self) -> bool {
        true
    }
}

impl Default for GetFrameResponseHeader {
    /* Protocol 2, MessageType : 4 */
    fn default() -> GetFrameResponseHeader {
        GetFrameResponseHeader {
            channel_uris: vec![],
            indexes: vec![],
        }
    }
}
