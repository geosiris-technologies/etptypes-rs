// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::energistics::etp::v12::datatypes::uuid::Uuid;
use crate::helpers::ETPMetadata;
use crate::helpers::*;
use avro_rs::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct Chunk {
    #[serde(rename = "blobId")]
    pub blob_id: Uuid,

    #[serde(rename = "data")]
    pub data: Vec<u8>,

    #[serde(rename = "final")]
    pub final_: bool,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.StoreNotification", "name": "Chunk", "protocol": "5", "messageType": "9", "senderRole": "store", "protocolRoles": "store,customer", "multipartFlag": true, "fields": [{"name": "blobId", "type": {"type": "fixed", "namespace": "Energistics.Etp.v12.Datatypes", "name": "Uuid", "size": 16, "fullName": "Energistics.Etp.v12.Datatypes.Uuid", "depends": []}}, {"name": "data", "type": "bytes"}, {"name": "final", "type": "boolean"}], "fullName": "Energistics.Etp.v12.Protocol.StoreNotification.Chunk", "depends": ["Energistics.Etp.v12.Datatypes.Uuid"]}"#;

impl ETPMetadata for Chunk {
    fn avro_schema() -> Option<Schema> {
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) -> i32 {
        5
    }
    fn message_type(&self) -> i32 {
        9
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
}

impl Chunk {
    /* Protocol 5, MessageType : 9 */
    pub fn default_with_params(data: Vec<u8>) -> Chunk {
        Chunk {
            blob_id: Uuid::new_v4(),
            data,
            final_: true,
        }
    }
}
