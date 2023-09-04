#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use avro_rs::{Schema, Error};
use crate::helpers::EtpMessageBody;


#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionsStopped{

	#[serde(rename = "reason")]
    pub reason:String,


	#[serde(rename = "channelIds")]
    #[derivative(Default(value = "HashMap::new()" ))]
    pub channel_ids:HashMap<String, i64,>,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.ChannelSubscribe", "name": "SubscriptionsStopped", "protocol": "21", "messageType": "8", "senderRole": "store", "protocolRoles": "store,customer", "multipartFlag": true, "fields": [{"name": "reason", "type": "string"}, {"name": "channelIds", "type": {"type": "map", "values": "long"}, "default": {}}], "fullName": "Energistics.Etp.v12.Protocol.ChannelSubscribe.SubscriptionsStopped", "depends": []}"#;

impl EtpMessageBody for SubscriptionsStopped{
    fn avro_schema() -> Option<Schema>{
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) ->i32{
        21
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
        true
    }
}

