#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use avro_rs::{Schema, Error};
use crate::helpers::EtpMessageBody;

use crate::energistics::etp::v12::datatypes::object::change_response_info::ChangeResponseInfo;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct GetChangeAnnotationsResponse{

	#[serde(rename = "changes")]
    pub changes:HashMap<String, ChangeResponseInfo,>,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.ChannelSubscribe", "name": "GetChangeAnnotationsResponse", "protocol": "21", "messageType": "15", "senderRole": "store", "protocolRoles": "store,customer", "multipartFlag": true, "fields": [{"name": "changes", "type": {"type": "map", "values": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "ChangeResponseInfo", "fields": [{"name": "responseTimestamp", "type": "long"}, {"name": "changes", "type": {"type": "map", "values": {"type": "array", "items": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "ChangeAnnotation", "fields": [{"name": "changeTime", "type": "long"}, {"name": "interval", "type": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "IndexInterval", "fields": [{"name": "startIndex", "type": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "IndexValue", "fields": [{"name": "item", "type": ["null", "long", "double", {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "PassIndexedDepth", "fields": [{"name": "pass", "type": "long"}, {"name": "direction", "type": {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "PassDirection", "symbols": ["Up", "HoldingSteady", "Down"], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.PassDirection", "depends": []}}, {"name": "depth", "type": "double"}], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.PassIndexedDepth", "depends": ["Energistics.Etp.v12.Datatypes.ChannelData.PassDirection"]}]}], "fullName": "Energistics.Etp.v12.Datatypes.IndexValue", "depends": ["Energistics.Etp.v12.Datatypes.ChannelData.PassIndexedDepth"]}}, {"name": "endIndex", "type": "Energistics.Etp.v12.Datatypes.IndexValue"}, {"name": "uom", "type": "string"}, {"name": "depthDatum", "type": "string", "default": ""}], "fullName": "Energistics.Etp.v12.Datatypes.Object.IndexInterval", "depends": ["Energistics.Etp.v12.Datatypes.IndexValue"]}}], "fullName": "Energistics.Etp.v12.Datatypes.Object.ChangeAnnotation", "depends": ["Energistics.Etp.v12.Datatypes.Object.IndexInterval"]}}}}], "fullName": "Energistics.Etp.v12.Datatypes.Object.ChangeResponseInfo", "depends": ["Energistics.Etp.v12.Datatypes.Object.ChangeAnnotation"]}}}], "fullName": "Energistics.Etp.v12.Protocol.ChannelSubscribe.GetChangeAnnotationsResponse", "depends": ["Energistics.Etp.v12.Datatypes.Object.ChangeResponseInfo"]}"#;

impl EtpMessageBody for GetChangeAnnotationsResponse{
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
        15
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

