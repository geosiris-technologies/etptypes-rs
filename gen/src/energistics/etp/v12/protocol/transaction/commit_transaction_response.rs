// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use avro_rs::{Schema, Error};
use crate::helpers::EtpMessageBody;

use crate::energistics::etp::v12::datatypes::uuid::Uuid;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct CommitTransactionResponse{

	#[serde(rename = "transactionUuid")]
    pub transaction_uuid:Uuid,


	#[serde(rename = "successful")]
    #[derivative(Default(value = "true" ))]
    pub successful:bool,


	#[serde(rename = "failureReason")]
    #[derivative(Default(value = r#"String::from("")"# ))]
    pub failure_reason:String,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.Transaction", "name": "CommitTransactionResponse", "protocol": "18", "messageType": "5", "senderRole": "store", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "transactionUuid", "type": {"type": "fixed", "namespace": "Energistics.Etp.v12.Datatypes", "name": "Uuid", "size": 16, "fullName": "Energistics.Etp.v12.Datatypes.Uuid", "depends": []}}, {"name": "successful", "type": "boolean", "default": true}, {"name": "failureReason", "type": "string", "default": ""}], "fullName": "Energistics.Etp.v12.Protocol.Transaction.CommitTransactionResponse", "depends": ["Energistics.Etp.v12.Datatypes.Uuid"]}"#;

impl EtpMessageBody for CommitTransactionResponse{
    fn avro_schema() -> Option<Schema>{
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) ->i32{
        18
    }
    fn message_type(&self) ->i32{
        5
    }
    fn sender_role(&self) ->String{
        "store".to_string()
    }
    fn protocol_roles(&self) ->String{
        "store,customer".to_string()
    }
    fn multipart_flag(&self) ->bool{
        false
    }
}

