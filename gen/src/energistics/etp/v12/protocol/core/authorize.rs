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
pub struct Authorize {
    #[serde(rename = "authorization")]
    pub authorization: String,

    #[serde(rename = "supplementalAuthorization")]
    pub supplemental_authorization: HashMap<String, String>,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.Core", "name": "Authorize", "protocol": "0", "messageType": "6", "senderRole": "client,server", "protocolRoles": "client, server", "multipartFlag": false, "fields": [{"name": "authorization", "type": "string"}, {"name": "supplementalAuthorization", "type": {"type": "map", "values": "string"}}], "fullName": "Energistics.Etp.v12.Protocol.Core.Authorize", "depends": []}"#;

impl EtpMessageBody for Authorize {
    fn avro_schema() -> Option<Schema> {
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) -> i32 {
        0
    }
    fn message_type(&self) -> i32 {
        6
    }
    fn sender_role(&self) -> String {
        "client,server".to_string()
    }
    fn protocol_roles(&self) -> String {
        "client, server".to_string()
    }
    fn multipart_flag(&self) -> bool {
        false
    }
}