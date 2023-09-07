// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::ETPMetadata;
use crate::helpers::*;
use avro_rs::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct WMLS_AddToStore {
    #[serde(rename = "WMLtypeIn")]
    pub wmltype_in: String,

    #[serde(rename = "XMLin")]
    pub xmlin: String,

    #[serde(rename = "OptionsIn")]
    pub options_in: String,

    #[serde(rename = "CapabilitiesIn")]
    pub capabilities_in: String,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.PrivateProtocols.WitsmlSoap", "name": "WMLS_AddToStore", "protocol": "2100", "messageType": "1", "senderRole": "customer", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "WMLtypeIn", "type": "string"}, {"name": "XMLin", "type": "string"}, {"name": "OptionsIn", "type": "string"}, {"name": "CapabilitiesIn", "type": "string"}], "fullName": "Energistics.Etp.v12.PrivateProtocols.WitsmlSoap.WMLS_AddToStore", "depends": []}"#;

impl ETPMetadata for WMLS_AddToStore {
    fn avro_schema() -> Option<Schema> {
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) -> i32 {
        2100
    }
    fn message_type(&self) -> i32 {
        1
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

impl Default for WMLS_AddToStore {
    /* Protocol 2100, MessageType : 1 */
    fn default() -> WMLS_AddToStore {
        WMLS_AddToStore {
            wmltype_in: "".to_string(),
            xmlin: "".to_string(),
            options_in: "".to_string(),
            capabilities_in: "".to_string(),
        }
    }
}
