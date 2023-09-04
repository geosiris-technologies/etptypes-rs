#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use avro_rs::{Schema, Error};
use crate::helpers::EtpMessageBody;


#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct CloseSession{

	#[serde(rename = "reason")]
    #[derivative(Default(value = r#"String::from("")"# ))]
    pub reason:String,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.Core", "name": "CloseSession", "protocol": "0", "messageType": "5", "senderRole": "client,server", "protocolRoles": "client, server", "multipartFlag": false, "fields": [{"name": "reason", "type": "string", "default": ""}], "fullName": "Energistics.Etp.v12.Protocol.Core.CloseSession", "depends": []}"#;

impl EtpMessageBody for CloseSession{
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
        5
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

