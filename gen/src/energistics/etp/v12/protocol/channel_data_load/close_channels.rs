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
pub struct CloseChannels {
    #[serde(rename = "id")]
    pub id: HashMap<String, i64>,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.ChannelDataLoad", "name": "CloseChannels", "protocol": "22", "messageType": "3", "senderRole": "customer", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "id", "type": {"type": "map", "values": "long"}}], "fullName": "Energistics.Etp.v12.Protocol.ChannelDataLoad.CloseChannels", "depends": []}"#;

impl EtpMessageBody for CloseChannels {
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