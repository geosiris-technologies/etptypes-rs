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
pub struct GetChangeAnnotations {
    #[serde(rename = "sinceChangeTime")]
    pub since_change_time: i64,

    #[serde(rename = "uris")]
    pub uris: HashMap<String, String>,

    #[serde(rename = "latestOnly")]
    #[derivative(Default(value = "false"))]
    pub latest_only: bool,
}

impl Schemable for GetChangeAnnotations {
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

impl ETPMetadata for GetChangeAnnotations {
    fn protocol(&self) -> i32 {
        6
    }
    fn message_type(&self) -> i32 {
        19
    }
    fn sender_role(&self) -> Vec<Role> {
        vec![Role::Customer]
    }
    fn protocol_roles(&self) -> Vec<Role> {
        vec![Role::Store, Role::Customer]
    }
    fn multipart_flag(&self) -> bool {
        false
    }

    fn avro_deserialize<R: Read>(input: &mut R) -> AvroResult<GetChangeAnnotations> {
        let record =
            from_avro_datum(&GetChangeAnnotations::avro_schema().unwrap(), input, None).unwrap();
        from_value::<GetChangeAnnotations>(&record)
    }
}

impl Default for GetChangeAnnotations {
    /* Protocol 6, MessageType : 19 */
    fn default() -> GetChangeAnnotations {
        GetChangeAnnotations {
            since_change_time: time_to_etp(SystemTime::now()),
            uris: HashMap::new(),
            latest_only: false,
        }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Protocol.GrowingObject",
    "name": "GetChangeAnnotations",
    "protocol": "6",
    "messageType": "19",
    "senderRole": "customer",
    "protocolRoles": "store,customer",
    "multipartFlag": false,
    "fields": [
        {
            "name": "sinceChangeTime",
            "type": "long"
        },
        {
            "name": "uris",
            "type": {
                "type": "map",
                "values": "string"
            }
        },
        {
            "name": "latestOnly",
            "type": "boolean",
            "default": false
        }
    ],
    "fullName": "Energistics.Etp.v12.Protocol.GrowingObject.GetChangeAnnotations",
    "depends": []
}"#;
