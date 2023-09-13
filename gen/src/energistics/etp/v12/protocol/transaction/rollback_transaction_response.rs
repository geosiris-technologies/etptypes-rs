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
pub struct RollbackTransactionResponse {
    #[serde(with = "serde_bytes")]
    #[serde(rename = "transactionUuid")]
    pub transaction_uuid: Uuid,

    #[serde(rename = "successful")]
    #[derivative(Default(value = "true"))]
    pub successful: bool,

    #[serde(rename = "failureReason")]
    #[derivative(Default(value = r#"String::from("")"#))]
    pub failure_reason: String,
}

impl Schemable for RollbackTransactionResponse {
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

impl ETPMetadata for RollbackTransactionResponse {
    fn protocol(&self) -> i32 {
        18
    }
    fn message_type(&self) -> i32 {
        6
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

    fn avro_deserialize<R: Read>(input: &mut R) -> AvroResult<RollbackTransactionResponse> {
        let record = from_avro_datum(
            &RollbackTransactionResponse::avro_schema().unwrap(),
            input,
            None,
        )
        .unwrap();
        from_value::<RollbackTransactionResponse>(&record)
    }
}

impl Default for RollbackTransactionResponse {
    /* Protocol 18, MessageType : 6 */
    fn default() -> RollbackTransactionResponse {
        RollbackTransactionResponse {
            transaction_uuid: random_uuid(),
            successful: true,
            failure_reason: "".to_string(),
        }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Protocol.Transaction",
    "name": "RollbackTransactionResponse",
    "protocol": "18",
    "messageType": "6",
    "senderRole": "store",
    "protocolRoles": "store,customer",
    "multipartFlag": false,
    "fields": [
        {
            "name": "transactionUuid",
            "type": {
                "type": "fixed",
                "namespace": "Energistics.Etp.v12.Datatypes",
                "name": "Uuid",
                "size": 16,
                "fullName": "Energistics.Etp.v12.Datatypes.Uuid",
                "depends": []
            }
        },
        {
            "name": "successful",
            "type": "boolean",
            "default": true
        },
        {
            "name": "failureReason",
            "type": "string",
            "default": ""
        }
    ],
    "fullName": "Energistics.Etp.v12.Protocol.Transaction.RollbackTransactionResponse",
    "depends": [
        "Energistics.Etp.v12.Datatypes.Uuid"
    ]
}"#;
