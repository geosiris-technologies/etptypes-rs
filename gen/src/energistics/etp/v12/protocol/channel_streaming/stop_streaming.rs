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
pub struct StopStreaming {}

impl Schemable for StopStreaming {
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

impl ETPMetadata for StopStreaming {
    fn protocol(&self) -> i32 {
        1
    }
    fn message_type(&self) -> i32 {
        4
    }
    fn sender_role(&self) -> Vec<Role> {
        vec![Role::Consumer]
    }
    fn protocol_roles(&self) -> Vec<Role> {
        vec![Role::Producer, Role::Consumer]
    }
    fn multipart_flag(&self) -> bool {
        false
    }

    fn avro_deserialize<R: Read>(input: &mut R) -> AvroResult<StopStreaming> {
        let record = from_avro_datum(&StopStreaming::avro_schema().unwrap(), input, None).unwrap();
        from_value::<StopStreaming>(&record)
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Protocol.ChannelStreaming",
    "name": "StopStreaming",
    "protocol": "1",
    "messageType": "4",
    "senderRole": "consumer",
    "protocolRoles": "producer,consumer",
    "multipartFlag": false,
    "fields": [],
    "fullName": "Energistics.Etp.v12.Protocol.ChannelStreaming.StopStreaming",
    "depends": []
}"#;
