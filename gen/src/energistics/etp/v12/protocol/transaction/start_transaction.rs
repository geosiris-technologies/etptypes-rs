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
#[serde(rename_all = "camelCase")]
pub struct StartTransaction {
    #[serde(rename = "readOnly")]
    pub read_only: bool,

    #[serde(rename = "message")]
    #[derivative(Default(value = r#"String::from("")"#))]
    pub message: String,

    #[serde(rename = "dataspaceUris")]
    #[derivative(Default(value = "Vec::new()"))]
    pub dataspace_uris: Vec<String>,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.Transaction", "name": "StartTransaction", "protocol": "18", "messageType": "1", "senderRole": "customer", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "readOnly", "type": "boolean"}, {"name": "message", "type": "string", "default": ""}, {"name": "dataspaceUris", "type": {"type": "array", "items": "string"}, "default": [""]}], "fullName": "Energistics.Etp.v12.Protocol.Transaction.StartTransaction", "depends": []}"#;

impl ETPMetadata for StartTransaction {
    fn avro_schema() -> Option<Schema> {
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) -> i32 {
        18
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

impl Default for StartTransaction {
    /* Protocol 18, MessageType : 1 */
    fn default() -> StartTransaction {
        StartTransaction {
            read_only: true,
            message: "".to_string(),
            dataspace_uris: vec!["".to_string()],
        }
    }
}
