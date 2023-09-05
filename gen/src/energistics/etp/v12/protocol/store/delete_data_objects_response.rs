// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::EtpMessageBody;
use avro_rs::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;

use crate::energistics::etp::v12::datatypes::array_of_string::ArrayOfString;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDataObjectsResponse {
    #[serde(rename = "deletedUris")]
    pub deleted_uris: HashMap<String, ArrayOfString>,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.Store", "name": "DeleteDataObjectsResponse", "protocol": "4", "messageType": "10", "senderRole": "store", "protocolRoles": "store,customer", "multipartFlag": true, "fields": [{"name": "deletedUris", "type": {"type": "map", "values": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfString", "fields": [{"name": "values", "type": {"type": "array", "items": "string"}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfString", "depends": []}}}], "fullName": "Energistics.Etp.v12.Protocol.Store.DeleteDataObjectsResponse", "depends": ["Energistics.Etp.v12.Datatypes.ArrayOfString"]}"#;

impl EtpMessageBody for DeleteDataObjectsResponse {
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
        10
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
