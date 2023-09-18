// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::*;
use apache_avro::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::SystemTime;

use crate::energistics::etp::v12::datatypes::uuid::{random_uuid, Uuid};
use crate::helpers::ETPMetadata;
use crate::helpers::Schemable;
use crate::protocols::ProtocolMessage;
use apache_avro::{from_avro_datum, from_value, AvroResult};
use std::io::Read;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectDeleted {
    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "changeTime")]
    pub change_time: i64,

    #[serde(with = "serde_bytes")]
    #[serde(rename = "requestUuid")]
    pub request_uuid: Uuid,
}

fn objectdeleted_avro_schema() -> Option<Schema> {
    match Schema::parse_str(AVRO_SCHEMA) {
        Ok(result) => Some(result),
        Err(e) => {
            panic!("{:?}", e);
        }
    }
}

impl Schemable for ObjectDeleted {
    fn avro_schema(&self) -> Option<Schema> {
        objectdeleted_avro_schema()
    }
    fn avro_schema_str(&self) -> &'static str {
        AVRO_SCHEMA
    }
}

impl AvroSerializable for ObjectDeleted {}

impl AvroDeserializable for ObjectDeleted {
    fn avro_deserialize<R: Read>(input: &mut R) -> AvroResult<ObjectDeleted> {
        let record = from_avro_datum(&objectdeleted_avro_schema().unwrap(), input, None).unwrap();
        from_value::<ObjectDeleted>(&record)
    }
}

impl ETPMetadata for ObjectDeleted {
    fn protocol(&self) -> i32 {
        5
    }
    fn message_type(&self) -> i32 {
        3
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
}

impl ObjectDeleted {
    pub fn as_protocol_message(&self) -> ProtocolMessage {
        ProtocolMessage::StoreNotification_ObjectDeleted(self.clone())
    }
}

impl Default for ObjectDeleted {
    /* Protocol 5, MessageType : 3 */
    fn default() -> ObjectDeleted {
        ObjectDeleted {
            uri: "".to_string(),
            change_time: time_to_etp(SystemTime::now()),
            request_uuid: random_uuid(),
        }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Protocol.StoreNotification",
    "name": "ObjectDeleted",
    "protocol": "5",
    "messageType": "3",
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
    "fullName": "Energistics.Etp.v12.Protocol.StoreNotification.ObjectDeleted",
    "depends": [
        "Energistics.Etp.v12.Datatypes.Uuid"
    ]
}"#;
