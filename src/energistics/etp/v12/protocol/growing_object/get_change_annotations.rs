#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use avro_rs::{Schema, Error};
use crate::helpers::EtpMessageBody;


#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct GetChangeAnnotations{

	#[serde(rename = "sinceChangeTime")]
    pub since_change_time:i64,


	#[serde(rename = "uris")]
    pub uris:HashMap<String, String,>,


	#[serde(rename = "latestOnly")]
    #[derivative(Default(value = "false" ))]
    pub latest_only:bool,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.GrowingObject", "name": "GetChangeAnnotations", "protocol": "6", "messageType": "19", "senderRole": "customer", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "sinceChangeTime", "type": "long"}, {"name": "uris", "type": {"type": "map", "values": "string"}}, {"name": "latestOnly", "type": "boolean", "default": false}], "fullName": "Energistics.Etp.v12.Protocol.GrowingObject.GetChangeAnnotations", "depends": []}"#;

impl EtpMessageBody for GetChangeAnnotations{
    fn avro_schema() -> Option<Schema>{
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) ->i32{
        6
    }
    fn message_type(&self) ->i32{
        19
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

