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
pub struct WMLS_GetCapResponse{

	#[serde(rename = "Result")]
    pub result:i32,


	#[serde(rename = "CapabilitiesOut")]
    pub capabilities_out:String,


	#[serde(rename = "SuppMsgOut")]
    pub supp_msg_out:String,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.PrivateProtocols.WitsmlSoap", "name": "WMLS_GetCapResponse", "protocol": "2100", "messageType": "8", "senderRole": "store", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "Result", "type": "int"}, {"name": "CapabilitiesOut", "type": "string"}, {"name": "SuppMsgOut", "type": "string"}], "fullName": "Energistics.Etp.v12.PrivateProtocols.WitsmlSoap.WMLS_GetCapResponse", "depends": []}"#;

impl EtpMessageBody for WMLS_GetCapResponse{
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
        8
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

impl WMLS_GetCapResponse{
    /* Protocol 2100, MessageType : 8 */
    pub fn default_with_params(_result: i32, )
    -> WMLS_GetCapResponse {
        WMLS_GetCapResponse {
            result : _result,
            capabilities_out : "".to_string(),
            supp_msg_out : "".to_string(),
        }
    }
}

