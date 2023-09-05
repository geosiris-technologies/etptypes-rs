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
pub struct StartStreaming{
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.ChannelStreaming", "name": "StartStreaming", "protocol": "1", "messageType": "3", "senderRole": "consumer", "protocolRoles": "producer,consumer", "multipartFlag": false, "fields": [], "fullName": "Energistics.Etp.v12.Protocol.ChannelStreaming.StartStreaming", "depends": []}"#;

impl EtpMessageBody for StartStreaming{
    fn avro_schema() -> Option<Schema>{
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) ->i32{
        1
    }
    fn message_type(&self) ->i32{
        3
    }
    fn sender_role(&self) ->String{
        "consumer".to_string()
    }
    fn protocol_roles(&self) ->String{
        "producer,consumer".to_string()
    }
    fn multipart_flag(&self) ->bool{
        false
    }
}

