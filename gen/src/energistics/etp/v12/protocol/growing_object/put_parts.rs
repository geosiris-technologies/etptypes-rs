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
pub struct PutParts {
    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "format")]
    #[derivative(Default(value = r#"String::from("xml")"#))]
    pub format: String,

    #[serde(rename = "parts")]
    pub parts: HashMap<String, ObjectPart>,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.GrowingObject", "name": "PutParts", "protocol": "6", "messageType": "5", "senderRole": "customer", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "uri", "type": "string"}, {"name": "format", "type": "string", "default": "xml"}, {"name": "parts", "type": {"type": "map", "values": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "ObjectPart", "fields": [{"name": "uid", "type": "string"}, {"name": "data", "type": "bytes"}], "fullName": "Energistics.Etp.v12.Datatypes.Object.ObjectPart", "depends": []}}}], "fullName": "Energistics.Etp.v12.Protocol.GrowingObject.PutParts", "depends": ["Energistics.Etp.v12.Datatypes.Object.ObjectPart"]}"#;

impl EtpMessageBody for PutParts {
    fn avro_schema() -> Option<Schema> {
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) -> i32 {
        6
    }
    fn message_type(&self) -> i32 {
        5
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
