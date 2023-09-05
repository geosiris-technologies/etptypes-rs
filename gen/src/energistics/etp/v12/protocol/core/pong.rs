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
pub struct Pong{

	#[serde(rename = "currentDateTime")]
    pub current_date_time:i64,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.Core", "name": "Pong", "protocol": "0", "messageType": "9", "senderRole": "client,server", "protocolRoles": "client, server", "multipartFlag": false, "fields": [{"name": "currentDateTime", "type": "long"}], "fullName": "Energistics.Etp.v12.Protocol.Core.Pong", "depends": []}"#;

impl EtpMessageBody for Pong{
    fn avro_schema() -> Option<Schema>{
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) ->i32{
        0
    }
    fn message_type(&self) ->i32{
        9
    }
    fn sender_role(&self) ->String{
        "client,server".to_string()
    }
    fn protocol_roles(&self) ->String{
        "client, server".to_string()
    }
    fn multipart_flag(&self) ->bool{
        false
    }
}

impl Default for Pong{
    /* Protocol 0, MessageType : 9 */
    fn default()
    -> Pong {
        Pong {
            current_date_time : time_to_etp(SystemTime::now()),
        }
    }
}

