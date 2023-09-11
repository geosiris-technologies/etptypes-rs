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
pub struct GetFrameMetadata {
    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "includeAllChannelSecondaryIndexes")]
    #[derivative(Default(value = "false"))]
    pub include_all_channel_secondary_indexes: bool,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.ChannelDataFrame", "name": "GetFrameMetadata", "protocol": "2", "messageType": "1", "senderRole": "customer", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "uri", "type": "string"}, {"name": "includeAllChannelSecondaryIndexes", "type": "boolean", "default": false}], "fullName": "Energistics.Etp.v12.Protocol.ChannelDataFrame.GetFrameMetadata", "depends": []}"#;

impl ETPMetadata for GetFrameMetadata {
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
        1
    }
    fn sender_role(&self) -> Vec<Role> {
        vec![Role::Customer]
    }
    fn protocol_roles(&self) -> Vec<Role> {
        vec![Role::Store, Role::Customer]
    }
    fn multipart_flag(&self) -> bool {
        false
    }
}

impl Default for GetFrameMetadata {
    /* Protocol 2, MessageType : 1 */
    fn default() -> GetFrameMetadata {
        GetFrameMetadata {
            uri: "".to_string(),
            include_all_channel_secondary_indexes: false,
        }
    }
}
