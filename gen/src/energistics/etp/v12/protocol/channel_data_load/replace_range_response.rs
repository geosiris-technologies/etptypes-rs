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
pub struct ReplaceRangeResponse{

	#[serde(rename = "channelChangeTime")]
    pub channel_change_time:i64,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.ChannelDataLoad", "name": "ReplaceRangeResponse", "protocol": "22", "messageType": "8", "senderRole": "store", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "channelChangeTime", "type": "long"}], "fullName": "Energistics.Etp.v12.Protocol.ChannelDataLoad.ReplaceRangeResponse", "depends": []}"#;

impl EtpMessageBody for ReplaceRangeResponse{
    fn avro_schema() -> Option<Schema>{
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) ->i32{
        22
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

impl Default for ReplaceRangeResponse{
    /* Protocol 22, MessageType : 8 */
    fn default()
    -> ReplaceRangeResponse {
        ReplaceRangeResponse {
            channel_change_time : time_to_etp(SystemTime::now()),
        }
    }
}

