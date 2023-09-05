// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::EtpMessageBody;
use avro_rs::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;

use crate::energistics::etp::v12::datatypes::uuid::Uuid;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct PartsDeleted {
    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "requestUuid")]
    pub request_uuid: Uuid,

    #[serde(rename = "changeTime")]
    pub change_time: i64,

    #[serde(rename = "uids")]
    pub uids: Vec<String>,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.GrowingObjectNotification", "name": "PartsDeleted", "protocol": "7", "messageType": "3", "senderRole": "store", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "uri", "type": "string"}, {"name": "requestUuid", "type": {"type": "fixed", "namespace": "Energistics.Etp.v12.Datatypes", "name": "Uuid", "size": 16, "fullName": "Energistics.Etp.v12.Datatypes.Uuid", "depends": []}}, {"name": "changeTime", "type": "long"}, {"name": "uids", "type": {"type": "array", "items": "string"}}], "fullName": "Energistics.Etp.v12.Protocol.GrowingObjectNotification.PartsDeleted", "depends": ["Energistics.Etp.v12.Datatypes.Uuid"]}"#;

impl EtpMessageBody for PartsDeleted {
    fn avro_schema() -> Option<Schema> {
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) -> i32 {
        7
    }
    fn message_type(&self) -> i32 {
        3
    }
    fn sender_role(&self) -> String {
        "store".to_string()
    }
    fn protocol_roles(&self) -> String {
        "store,customer".to_string()
    }
    fn multipart_flag(&self) -> bool {
        false
    }
}
