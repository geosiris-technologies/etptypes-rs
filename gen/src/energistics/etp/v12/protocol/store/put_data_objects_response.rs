// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use avro_rs::{Schema, Error};
use crate::helpers::EtpMessageBody;

use crate::energistics::etp::v12::datatypes::object::put_response::PutResponse;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct PutDataObjectsResponse{

	#[serde(rename = "success")]
    pub success:HashMap<String, PutResponse,>,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.Store", "name": "PutDataObjectsResponse", "protocol": "4", "messageType": "9", "senderRole": "store", "protocolRoles": "store,customer", "multipartFlag": true, "fields": [{"name": "success", "type": {"type": "map", "values": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "PutResponse", "fields": [{"name": "createdContainedObjectUris", "type": {"type": "array", "items": "string"}, "default": []}, {"name": "deletedContainedObjectUris", "type": {"type": "array", "items": "string"}, "default": []}, {"name": "joinedContainedObjectUris", "type": {"type": "array", "items": "string"}, "default": []}, {"name": "unjoinedContainedObjectUris", "type": {"type": "array", "items": "string"}, "default": []}], "fullName": "Energistics.Etp.v12.Datatypes.Object.PutResponse", "depends": []}}}], "fullName": "Energistics.Etp.v12.Protocol.Store.PutDataObjectsResponse", "depends": ["Energistics.Etp.v12.Datatypes.Object.PutResponse"]}"#;

impl EtpMessageBody for PutDataObjectsResponse{
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
        9
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

