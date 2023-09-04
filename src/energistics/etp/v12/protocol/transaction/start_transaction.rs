#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use avro_rs::{Schema, Error};
use crate::helpers::EtpMessageBody;


#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct StartTransaction{

	#[serde(rename = "readOnly")]
    pub read_only:bool,


	#[serde(rename = "message")]
    #[derivative(Default(value = r#"String::from("")"# ))]
    pub message:String,


	#[serde(rename = "dataspaceUris")]
    #[derivative(Default(value = "Vec::new()" ))]
    pub dataspace_uris:Vec<String>,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.Transaction", "name": "StartTransaction", "protocol": "18", "messageType": "1", "senderRole": "customer", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "readOnly", "type": "boolean"}, {"name": "message", "type": "string", "default": ""}, {"name": "dataspaceUris", "type": {"type": "array", "items": "string"}, "default": [""]}], "fullName": "Energistics.Etp.v12.Protocol.Transaction.StartTransaction", "depends": []}"#;

impl EtpMessageBody for StartTransaction{
    fn avro_schema() -> Option<Schema>{
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) ->i32{
        18
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

