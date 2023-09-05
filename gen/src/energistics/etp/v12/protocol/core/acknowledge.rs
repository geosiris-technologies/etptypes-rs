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
pub struct Acknowledge {}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.Core", "name": "Acknowledge", "protocol": "0", "messageType": "1001", "senderRole": "*", "protocolRoles": "client, server", "multipartFlag": false, "fields": [], "fullName": "Energistics.Etp.v12.Protocol.Core.Acknowledge", "depends": []}"#;

impl EtpMessageBody for Acknowledge {
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
        1001
    }
    fn sender_role(&self) -> String {
        "*".to_string()
    }
    fn protocol_roles(&self) -> String {
        "client, server".to_string()
    }
    fn multipart_flag(&self) -> bool {
        false
    }
}
