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

use crate::energistics::etp::v12::datatypes::uuid::{random_uuid, Uuid};
use crate::helpers::ETPMetadata;
use crate::helpers::Schemable;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectAccessRevoked {
    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "changeTime")]
    pub change_time: i64,

    #[serde(with = "serde_bytes")]
    #[serde(rename = "requestUuid")]
    pub request_uuid: Uuid,
}

impl Schemable for ObjectAccessRevoked {
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

impl ETPMetadata for ObjectAccessRevoked {
    fn protocol(&self) -> i32 {
        5
    }
    fn message_type(&self) -> i32 {
        5
    }
    fn sender_role(&self) -> Vec<Role> {
        vec![Role::Store]
    }
    fn protocol_roles(&self) -> Vec<Role> {
        vec![Role::Store, Role::Customer]
    }
    fn multipart_flag(&self) -> bool {
        false
    }

    fn avro_deserialize<R: Read>(input: &mut R) -> AvroResult<ObjectAccessRevoked> {
        let record =
            from_avro_datum(&ObjectAccessRevoked::avro_schema().unwrap(), input, None).unwrap();
        from_value::<ObjectAccessRevoked>(&record)
    }
}

impl Default for ObjectAccessRevoked {
    /* Protocol 5, MessageType : 5 */
    fn default() -> ObjectAccessRevoked {
        ObjectAccessRevoked {
            uri: "".to_string(),
            change_time: time_to_etp(SystemTime::now()),
            request_uuid: random_uuid(),
        }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Protocol.StoreNotification",
    "name": "ObjectAccessRevoked",
    "protocol": "5",
    "messageType": "5",
    "senderRole": "store",
    "protocolRoles": "store,customer",
    "multipartFlag": false,
    "fields": [
        {
            "name": "uri",
            "type": "string"
        },
        {
            "name": "changeTime",
            "type": "long"
        },
        {
            "name": "requestUuid",
            "type": {
                "type": "fixed",
                "namespace": "Energistics.Etp.v12.Datatypes",
                "name": "Uuid",
                "size": 16,
                "fullName": "Energistics.Etp.v12.Datatypes.Uuid",
                "depends": []
            }
        }
    ],
    "fullName": "Energistics.Etp.v12.Protocol.StoreNotification.ObjectAccessRevoked",
    "depends": [
        "Energistics.Etp.v12.Datatypes.Uuid"
    ]
}"#;
