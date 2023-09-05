// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use avro_rs::{Schema, Error};
use crate::helpers::EtpMessageBody;

use crate::energistics::etp::v12::datatypes::object::supported_type::SupportedType;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct GetSupportedTypesResponse{

	#[serde(rename = "supportedTypes")]
    #[derivative(Default(value = "Vec::new()" ))]
    pub supported_types:Vec<SupportedType>,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.SupportedTypes", "name": "GetSupportedTypesResponse", "protocol": "25", "messageType": "2", "senderRole": "store", "protocolRoles": "store,customer", "multipartFlag": true, "fields": [{"name": "supportedTypes", "type": {"type": "array", "items": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "SupportedType", "fields": [{"name": "dataObjectType", "type": "string"}, {"name": "objectCount", "type": ["null", "int"]}, {"name": "relationshipKind", "type": {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "RelationshipKind", "symbols": ["Primary", "Secondary", "Both"], "fullName": "Energistics.Etp.v12.Datatypes.Object.RelationshipKind", "depends": []}}], "fullName": "Energistics.Etp.v12.Datatypes.Object.SupportedType", "depends": ["Energistics.Etp.v12.Datatypes.Object.RelationshipKind"]}}, "default": []}], "fullName": "Energistics.Etp.v12.Protocol.SupportedTypes.GetSupportedTypesResponse", "depends": ["Energistics.Etp.v12.Datatypes.Object.SupportedType"]}"#;

impl EtpMessageBody for GetSupportedTypesResponse{
    fn avro_schema() -> Option<Schema>{
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) ->i32{
        25
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
        true
    }
}

