#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use avro_rs::{Schema, Error};
use crate::helpers::EtpMessageBody;


#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct GetDataObjects{

	#[serde(rename = "uris")]
    pub uris:HashMap<String, String,>,


	#[serde(rename = "format")]
    #[derivative(Default(value = r#"String::from("xml")"# ))]
    pub format:String,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.Store", "name": "GetDataObjects", "protocol": "4", "messageType": "1", "senderRole": "customer", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "uris", "type": {"type": "map", "values": "string"}}, {"name": "format", "type": "string", "default": "xml"}], "fullName": "Energistics.Etp.v12.Protocol.Store.GetDataObjects", "depends": []}"#;

impl EtpMessageBody for GetDataObjects{
    fn avro_schema() -> Option<Schema>{
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) ->i32{
        4
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

