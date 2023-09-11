// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::*;
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::SystemTime;
#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct MessageHeader {
    #[serde(rename = "protocol")]
    pub protocol: i32,

    #[serde(rename = "messageType")]
    pub message_type: i32,

    #[serde(rename = "correlationId")]
    pub correlation_id: i64,

    #[serde(rename = "messageId")]
    pub message_id: i64,

    #[serde(rename = "messageFlags")]
    pub message_flags: i32,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "MessageHeader", "fields": [{"name": "protocol", "type": "int"}, {"name": "messageType", "type": "int"}, {"name": "correlationId", "type": "long"}, {"name": "messageId", "type": "long"}, {"name": "messageFlags", "type": "int"}], "fullName": "Energistics.Etp.v12.Datatypes.MessageHeader", "depends": []}"#;
