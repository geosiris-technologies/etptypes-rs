#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use avro_rs::{Schema, Error};
use crate::helpers::EtpMessageBody;


#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct ChannelsClosed{

	#[serde(rename = "reason")]
    pub reason:String,


	#[serde(rename = "id")]
    pub id:HashMap<String, i64,>,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.ChannelDataLoad", "name": "ChannelsClosed", "protocol": "22", "messageType": "7", "senderRole": "store", "protocolRoles": "store,customer", "multipartFlag": true, "fields": [{"name": "reason", "type": "string"}, {"name": "id", "type": {"type": "map", "values": "long"}}], "fullName": "Energistics.Etp.v12.Protocol.ChannelDataLoad.ChannelsClosed", "depends": []}"#;

impl EtpMessageBody for ChannelsClosed{
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
        7
    }
    fn sender_role(&self) ->String{
        "store".to_string()
    }
    fn protocol_roles(&self) ->String{
        "store,customer".to_string()
    }
    fn multipart_flag(&self) ->bool{
        true
    }
}

