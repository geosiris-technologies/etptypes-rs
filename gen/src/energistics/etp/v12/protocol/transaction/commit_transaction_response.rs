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

use crate::energistics::etp::v12::datatypes::uuid::Uuid;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct CommitTransactionResponse {
    #[serde(rename = "transactionUuid")]
    pub transaction_uuid: Uuid,

    #[serde(rename = "successful")]
    #[derivative(Default(value = "true"))]
    pub successful: bool,

    #[serde(rename = "failureReason")]
    #[derivative(Default(value = r#"String::from("")"#))]
    pub failure_reason: String,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.Transaction", "name": "CommitTransactionResponse", "protocol": "18", "messageType": "5", "senderRole": "store", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "transactionUuid", "type": {"type": "fixed", "namespace": "Energistics.Etp.v12.Datatypes", "name": "Uuid", "size": 16, "fullName": "Energistics.Etp.v12.Datatypes.Uuid", "depends": []}}, {"name": "successful", "type": "boolean", "default": true}, {"name": "failureReason", "type": "string", "default": ""}], "fullName": "Energistics.Etp.v12.Protocol.Transaction.CommitTransactionResponse", "depends": ["Energistics.Etp.v12.Datatypes.Uuid"]}"#;

impl ETPMetadata for CommitTransactionResponse {
    fn avro_schema() -> Option<Schema> {
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) -> i32 {
        18
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
}

impl Default for CommitTransactionResponse {
    /* Protocol 18, MessageType : 5 */
    fn default() -> CommitTransactionResponse {
        CommitTransactionResponse {
            transaction_uuid: Uuid::new_v4(),
            successful: true,
            failure_reason: "".to_string(),
        }
    }
}
