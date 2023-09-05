// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::{SystemTime};
use crate::helpers::*;
use avro_rs::{Schema, Error};
use crate::helpers::EtpMessageBody;


#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct WMLS_DeleteFromStoreResponse{

	#[serde(rename = "Result")]
    pub result:i32,


	#[serde(rename = "SuppMsgOut")]
    pub supp_msg_out:String,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.PrivateProtocols.WitsmlSoap", "name": "WMLS_DeleteFromStoreResponse", "protocol": "2100", "messageType": "4", "senderRole": "store", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "Result", "type": "int"}, {"name": "SuppMsgOut", "type": "string"}], "fullName": "Energistics.Etp.v12.PrivateProtocols.WitsmlSoap.WMLS_DeleteFromStoreResponse", "depends": []}"#;

impl EtpMessageBody for WMLS_DeleteFromStoreResponse{
    fn avro_schema() -> Option<Schema>{
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) ->i32{
        2100
    }
    fn message_type(&self) ->i32{
        4
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

impl WMLS_DeleteFromStoreResponse{
    /* Protocol 2100, MessageType : 4 */
    pub fn default_with_params(_result: i32, )
    -> WMLS_DeleteFromStoreResponse {
        WMLS_DeleteFromStoreResponse {
            result : _result,
            supp_msg_out : "".to_string(),
        }
    }
}

