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
pub struct GetFrameMetadata{

	#[serde(rename = "uri")]
    pub uri:String,


	#[serde(rename = "includeAllChannelSecondaryIndexes")]
    #[derivative(Default(value = "false" ))]
    pub include_all_channel_secondary_indexes:bool,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.ChannelDataFrame", "name": "GetFrameMetadata", "protocol": "2", "messageType": "1", "senderRole": "customer", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "uri", "type": "string"}, {"name": "includeAllChannelSecondaryIndexes", "type": "boolean", "default": false}], "fullName": "Energistics.Etp.v12.Protocol.ChannelDataFrame.GetFrameMetadata", "depends": []}"#;

impl EtpMessageBody for GetFrameMetadata{
    fn avro_schema() -> Option<Schema>{
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) ->i32{
        2
    }
    fn message_type(&self) ->i32{
        1
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

impl Default for GetFrameMetadata{
    /* Protocol 2, MessageType : 1 */
    fn default()
    -> GetFrameMetadata {
        GetFrameMetadata {
            uri : "".to_string(),
            include_all_channel_secondary_indexes : false,
        }
    }
}

