// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::energistics::etp::v12::datatypes::error_info::ErrorInfo;
use crate::helpers::ETPMetadata;
use crate::helpers::*;
use avro_rs::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct ProtocolException {
    #[serde(rename = "error")]
    pub error: Option<ErrorInfo>,

    #[serde(rename = "errors")]
    #[derivative(Default(value = "HashMap::new()"))]
    pub errors: HashMap<String, ErrorInfo>,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.Core", "name": "ProtocolException", "protocol": "0", "messageType": "1000", "senderRole": "*", "protocolRoles": "client, server", "multipartFlag": true, "fields": [{"name": "error", "type": ["null", {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ErrorInfo", "fields": [{"name": "message", "type": "string"}, {"name": "code", "type": "int"}], "fullName": "Energistics.Etp.v12.Datatypes.ErrorInfo", "depends": []}]}, {"name": "errors", "type": {"type": "map", "values": "Energistics.Etp.v12.Datatypes.ErrorInfo"}, "default": {}}], "fullName": "Energistics.Etp.v12.Protocol.Core.ProtocolException", "depends": ["Energistics.Etp.v12.Datatypes.ErrorInfo"]}"#;

impl ETPMetadata for ProtocolException {
    fn avro_schema() -> Option<Schema> {
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) -> i32 {
        0
    }
    fn message_type(&self) -> i32 {
        1000
    }
    fn sender_role(&self) -> Vec<Role> {
        vec![Role::All]
    }
    fn protocol_roles(&self) -> Vec<Role> {
        vec![Role::Client, Role::Server]
    }
    fn multipart_flag(&self) -> bool {
        true
    }
}

impl ProtocolException {
    /* Protocol 0, MessageType : 1000 */
    pub fn default_with_params(error: Option<ErrorInfo>) -> ProtocolException {
        ProtocolException {
            error,
            errors: HashMap::new(),
        }
    }
}
