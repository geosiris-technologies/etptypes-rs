// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::EtpMessageBody;
use avro_rs::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;

use crate::energistics::etp::v12::datatypes::object::object_part::ObjectPart;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct FindPartsResponse {
    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "serverSortOrder")]
    pub server_sort_order: String,

    #[serde(rename = "format")]
    #[derivative(Default(value = r#"String::from("xml")"#))]
    pub format: String,

    #[serde(rename = "parts")]
    #[derivative(Default(value = "Vec::new()"))]
    pub parts: Vec<ObjectPart>,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.GrowingObjectQuery", "name": "FindPartsResponse", "protocol": "16", "messageType": "2", "senderRole": "store", "protocolRoles": "store,customer", "multipartFlag": true, "fields": [{"name": "uri", "type": "string"}, {"name": "serverSortOrder", "type": "string"}, {"name": "format", "type": "string", "default": "xml"}, {"name": "parts", "type": {"type": "array", "items": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "ObjectPart", "fields": [{"name": "uid", "type": "string"}, {"name": "data", "type": "bytes"}], "fullName": "Energistics.Etp.v12.Datatypes.Object.ObjectPart", "depends": []}}, "default": []}], "fullName": "Energistics.Etp.v12.Protocol.GrowingObjectQuery.FindPartsResponse", "depends": ["Energistics.Etp.v12.Datatypes.Object.ObjectPart"]}"#;

impl EtpMessageBody for FindPartsResponse {
    fn avro_schema() -> Option<Schema> {
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) -> i32 {
        16
    }
    fn message_type(&self) -> i32 {
        2
    }
    fn sender_role(&self) -> String {
        "store".to_string()
    }
    fn protocol_roles(&self) -> String {
        "store,customer".to_string()
    }
    fn multipart_flag(&self) -> bool {
        true
    }
}
