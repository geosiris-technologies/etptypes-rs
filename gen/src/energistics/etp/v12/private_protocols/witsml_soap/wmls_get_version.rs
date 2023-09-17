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

use crate::helpers::ETPMetadata;
use crate::helpers::Schemable;
use crate::protocols::ProtocolMessage;
use apache_avro::{from_avro_datum, from_value, AvroResult};
use std::io::Read;
#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct WMLS_GetVersion {}

impl Schemable for WMLS_GetVersion {
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

    fn avro_deserialize<R: Read>(input: &mut R) -> AvroResult<WMLS_GetVersion> {
        let record =
            from_avro_datum(&WMLS_GetVersion::avro_schema().unwrap(), input, None).unwrap();
        from_value::<WMLS_GetVersion>(&record)
    }
}

impl ETPMetadata for WMLS_GetVersion {
    fn protocol(&self) -> i32 {
        2100
    }
    fn message_type(&self) -> i32 {
        11
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
}

impl WMLS_GetVersion {
    pub fn as_protocol_message(&self) -> ProtocolMessage {
        ProtocolMessage::WitsmlSoap_WMLS_GetVersion(self.clone())
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.PrivateProtocols.WitsmlSoap",
    "name": "WMLS_GetVersion",
    "protocol": "2100",
    "messageType": "11",
    "senderRole": "customer",
    "protocolRoles": "store,customer",
    "multipartFlag": false,
    "fields": [],
    "fullName": "Energistics.Etp.v12.PrivateProtocols.WitsmlSoap.WMLS_GetVersion",
    "depends": []
}"#;
