#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use avro_rs::{Schema, Error};
use crate::helpers::EtpMessageBody;

use crate::energistics::etp::v12::datatypes::uuid::Uuid;
use crate::energistics::etp::v12::datatypes::object::object_change_kind::ObjectChangeKind;
use crate::energistics::etp::v12::datatypes::object::object_part::ObjectPart;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct PartsChanged{

	#[serde(rename = "uri")]
    pub uri:String,


	#[serde(rename = "requestUuid")]
    pub request_uuid:Uuid,


	#[serde(rename = "changeKind")]
    pub change_kind:ObjectChangeKind,


	#[serde(rename = "changeTime")]
    pub change_time:i64,


	#[serde(rename = "format")]
    #[derivative(Default(value = r#"String::from("")"# ))]
    pub format:String,


	#[serde(rename = "parts")]
    pub parts:Vec<ObjectPart>,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.GrowingObjectNotification", "name": "PartsChanged", "protocol": "7", "messageType": "2", "senderRole": "store", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "uri", "type": "string"}, {"name": "requestUuid", "type": {"type": "fixed", "namespace": "Energistics.Etp.v12.Datatypes", "name": "Uuid", "size": 16, "fullName": "Energistics.Etp.v12.Datatypes.Uuid", "depends": []}}, {"name": "changeKind", "type": {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "ObjectChangeKind", "symbols": ["insert", "update", "authorized", "joined", "unjoined", "joinedSubscription", "unjoinedSubscription"], "fullName": "Energistics.Etp.v12.Datatypes.Object.ObjectChangeKind", "depends": []}}, {"name": "changeTime", "type": "long"}, {"name": "format", "type": "string", "default": ""}, {"name": "parts", "type": {"type": "array", "items": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "ObjectPart", "fields": [{"name": "uid", "type": "string"}, {"name": "data", "type": "bytes"}], "fullName": "Energistics.Etp.v12.Datatypes.Object.ObjectPart", "depends": []}}}], "fullName": "Energistics.Etp.v12.Protocol.GrowingObjectNotification.PartsChanged", "depends": ["Energistics.Etp.v12.Datatypes.Uuid", "Energistics.Etp.v12.Datatypes.Object.ObjectChangeKind", "Energistics.Etp.v12.Datatypes.Object.ObjectPart"]}"#;

impl EtpMessageBody for PartsChanged{
    fn avro_schema() -> Option<Schema>{
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) ->i32{
        7
    }
    fn message_type(&self) ->i32{
        2
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

