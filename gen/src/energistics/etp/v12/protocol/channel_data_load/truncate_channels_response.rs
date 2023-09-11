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

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct TruncateChannelsResponse {
    #[serde(rename = "channelsTruncatedTime")]
    pub channels_truncated_time: HashMap<String, i64>,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.ChannelDataLoad", "name": "TruncateChannelsResponse", "protocol": "22", "messageType": "10", "senderRole": "store", "protocolRoles": "store,customer", "multipartFlag": true, "fields": [{"name": "channelsTruncatedTime", "type": {"type": "map", "values": "long"}}], "fullName": "Energistics.Etp.v12.Protocol.ChannelDataLoad.TruncateChannelsResponse", "depends": []}"#;

impl ETPMetadata for TruncateChannelsResponse {
    fn avro_schema() -> Option<Schema> {
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) -> i32 {
        22
    }
    fn message_type(&self) -> i32 {
        10
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

impl Default for TruncateChannelsResponse {
    /* Protocol 22, MessageType : 10 */
    fn default() -> TruncateChannelsResponse {
        TruncateChannelsResponse {
            channels_truncated_time: HashMap::new(),
        }
    }
}
