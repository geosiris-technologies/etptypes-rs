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
pub struct WMLS_UpdateInStore{

	#[serde(rename = "WMLtypeIn")]
    pub wmltype_in:String,


	#[serde(rename = "XMLin")]
    pub xmlin:String,


	#[serde(rename = "OptionsIn")]
    pub options_in:String,


	#[serde(rename = "CapabilitiesIn")]
    pub capabilities_in:String,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.PrivateProtocols.WitsmlSoap", "name": "WMLS_UpdateInStore", "protocol": "2100", "messageType": "13", "senderRole": "customer", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "WMLtypeIn", "type": "string"}, {"name": "XMLin", "type": "string"}, {"name": "OptionsIn", "type": "string"}, {"name": "CapabilitiesIn", "type": "string"}], "fullName": "Energistics.Etp.v12.PrivateProtocols.WitsmlSoap.WMLS_UpdateInStore", "depends": []}"#;

impl EtpMessageBody for WMLS_UpdateInStore{
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
        13
    }
    fn sender_role(&self) ->String{
        "customer".to_string()
    }
    fn protocol_roles(&self) ->String{
        "store,customer".to_string()
    }
    fn multipart_flag(&self) ->bool{
        false
    }
}

impl Default for WMLS_UpdateInStore{
    /* Protocol 2100, MessageType : 13 */
    fn default()
    -> WMLS_UpdateInStore {
        WMLS_UpdateInStore {
            wmltype_in : "".to_string(),
            xmlin : "".to_string(),
            options_in : "".to_string(),
            capabilities_in : "".to_string(),
        }
    }
}

