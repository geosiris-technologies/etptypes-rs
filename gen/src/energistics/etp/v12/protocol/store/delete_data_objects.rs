// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::EtpMessageBody;
use avro_rs::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDataObjects {
    #[serde(rename = "uris")]
    pub uris: HashMap<String, String>,

    #[serde(rename = "pruneContainedObjects")]
    #[derivative(Default(value = "false"))]
    pub prune_contained_objects: bool,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.Store", "name": "DeleteDataObjects", "protocol": "4", "messageType": "3", "senderRole": "customer", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "uris", "type": {"type": "map", "values": "string"}}, {"name": "pruneContainedObjects", "type": "boolean", "default": false}], "fullName": "Energistics.Etp.v12.Protocol.Store.DeleteDataObjects", "depends": []}"#;

impl EtpMessageBody for DeleteDataObjects {
    fn avro_schema() -> Option<Schema> {
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) -> i32 {
        4
    }
    fn message_type(&self) -> i32 {
        3
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
