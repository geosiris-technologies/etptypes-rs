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
pub struct DeleteDataObjects {
    #[serde(rename = "uris")]
    pub uris: HashMap<String, String>,

    #[serde(rename = "pruneContainedObjects")]
    #[derivative(Default(value = "false"))]
    pub prune_contained_objects: bool,
}

impl Schemable for DeleteDataObjects {
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

impl ETPMetadata for DeleteDataObjects {
    fn protocol(&self) -> i32 {
        4
    }
    fn message_type(&self) -> i32 {
        3
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

    fn avro_deserialize<R: Read>(input: &mut R) -> AvroResult<DeleteDataObjects> {
        let record =
            from_avro_datum(&DeleteDataObjects::avro_schema().unwrap(), input, None).unwrap();
        from_value::<DeleteDataObjects>(&record)
    }
}

impl Default for DeleteDataObjects {
    /* Protocol 4, MessageType : 3 */
    fn default() -> DeleteDataObjects {
        DeleteDataObjects {
            uris: HashMap::new(),
            prune_contained_objects: false,
        }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Protocol.Store",
    "name": "DeleteDataObjects",
    "protocol": "4",
    "messageType": "3",
    "senderRole": "customer",
    "protocolRoles": "store,customer",
    "multipartFlag": false,
    "fields": [
        {
            "name": "uris",
            "type": {
                "type": "map",
                "values": "string"
            }
        },
        {
            "name": "pruneContainedObjects",
            "type": "boolean",
            "default": false
        }
    ],
    "fullName": "Energistics.Etp.v12.Protocol.Store.DeleteDataObjects",
    "depends": []
}"#;
