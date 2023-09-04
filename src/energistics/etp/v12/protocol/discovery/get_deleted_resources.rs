#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use avro_rs::{Schema, Error};
use crate::helpers::EtpMessageBody;


#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct GetDeletedResources{

	#[serde(rename = "dataspaceUri")]
    pub dataspace_uri:String,


	#[serde(rename = "deleteTimeFilter")]
    pub delete_time_filter:Option<
        i64>,


	#[serde(rename = "dataObjectTypes")]
    #[derivative(Default(value = "Vec::new()" ))]
    pub data_object_types:Vec<String>,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.Discovery", "name": "GetDeletedResources", "protocol": "3", "messageType": "5", "senderRole": "customer", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "dataspaceUri", "type": "string"}, {"name": "deleteTimeFilter", "type": ["null", "long"]}, {"name": "dataObjectTypes", "type": {"type": "array", "items": "string"}, "default": []}], "fullName": "Energistics.Etp.v12.Protocol.Discovery.GetDeletedResources", "depends": []}"#;

impl EtpMessageBody for GetDeletedResources{
    fn avro_schema() -> Option<Schema>{
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) ->i32{
        3
    }
    fn message_type(&self) ->i32{
        5
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

