// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::*;
use apache_avro::{from_avro_datum, from_value, AvroResult};
use apache_avro::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::io::Read;
use std::time::SystemTime;

use crate::helpers::ETPMetadata;
use crate::helpers::Schemable;
#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct PutDataspacesResponse {
    #[serde(rename = "success")]
    pub success: HashMap<String, String>,
}

impl Schemable for PutDataspacesResponse {
    fn avro_schema() -> Option<Schema> {
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn avro_schema_str() -> &'static str {
        AVRO_SCHEMA
    }
}

impl ETPMetadata for PutDataspacesResponse {
    fn protocol(&self) -> i32 {
        24
    }
    fn message_type(&self) -> i32 {
        6
    }
    fn sender_role(&self) -> Vec<Role> {
        vec![Role::Store]
    }
    fn protocol_roles(&self) -> Vec<Role> {
        vec![Role::Store, Role::Customer]
    }
    fn multipart_flag(&self) -> bool {
        true
    }

    fn avro_deserialize<R: Read>(input: &mut R) -> AvroResult<PutDataspacesResponse> {
        let record =
            from_avro_datum(&PutDataspacesResponse::avro_schema().unwrap(), input, None).unwrap();
        from_value::<PutDataspacesResponse>(&record)
    }
}

impl Default for PutDataspacesResponse {
    /* Protocol 24, MessageType : 6 */
    fn default() -> PutDataspacesResponse {
        PutDataspacesResponse {
            success: HashMap::new(),
        }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Protocol.Dataspace",
    "name": "PutDataspacesResponse",
    "protocol": "24",
    "messageType": "6",
    "senderRole": "store",
    "protocolRoles": "store,customer",
    "multipartFlag": true,
    "fields": [
        {
            "name": "success",
            "type": {
                "type": "map",
                "values": "string"
            }
        }
    ],
    "fullName": "Energistics.Etp.v12.Protocol.Dataspace.PutDataspacesResponse",
    "depends": []
}"#;
