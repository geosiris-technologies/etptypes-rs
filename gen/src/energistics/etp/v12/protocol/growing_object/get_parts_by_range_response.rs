// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::*;
use apache_avro::{from_avro_datum, from_value, AvroResult};
use apache_avro::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::io::Read;
use std::time::SystemTime;

use crate::energistics::etp::v12::datatypes::object::object_part::ObjectPart;
use crate::helpers::ETPMetadata;
use crate::helpers::Schemable;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct GetPartsByRangeResponse {
    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "format")]
    #[derivative(Default(value = r#"String::from("xml")"#))]
    pub format: String,

    #[serde(rename = "parts")]
    pub parts: Vec<ObjectPart>,
}

impl Schemable for GetPartsByRangeResponse {
    fn avro_schema() -> Option<Schema> {
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn avro_schema_str() -> &'static str {
        AVRO_SCHEMA
    }
}

impl ETPMetadata for GetPartsByRangeResponse {
    fn protocol(&self) -> i32 {
        6
    }
    fn message_type(&self) -> i32 {
        10
    }
    fn sender_role(&self) -> Vec<Role> {
        vec![Role::Store]
    }
    fn protocol_roles(&self) -> Vec<Role> {
        vec![Role::Store, Role::Customer]
    }
    fn multipart_flag(&self) -> bool {
        true
    }

    fn avro_deserialize<R: Read>(input: &mut R) -> AvroResult<GetPartsByRangeResponse> {
        let record = from_avro_datum(
            &GetPartsByRangeResponse::avro_schema().unwrap(),
            input,
            None,
        )
        .unwrap();
        from_value::<GetPartsByRangeResponse>(&record)
    }
}

impl Default for GetPartsByRangeResponse {
    /* Protocol 6, MessageType : 10 */
    fn default() -> GetPartsByRangeResponse {
        GetPartsByRangeResponse {
            uri: "".to_string(),
            format: "xml".to_string(),
            parts: vec![],
        }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Protocol.GrowingObject",
    "name": "GetPartsByRangeResponse",
    "protocol": "6",
    "messageType": "10",
    "senderRole": "store",
    "protocolRoles": "store,customer",
    "multipartFlag": true,
    "fields": [
        {
            "name": "uri",
            "type": "string"
        },
        {
            "name": "format",
            "type": "string",
            "default": "xml"
        },
        {
            "name": "parts",
            "type": {
                "type": "array",
                "items": {
                    "type": "record",
                    "namespace": "Energistics.Etp.v12.Datatypes.Object",
                    "name": "ObjectPart",
                    "fields": [
                        {
                            "name": "uid",
                            "type": "string"
                        },
                        {
                            "name": "data",
                            "type": "bytes"
                        }
                    ],
                    "fullName": "Energistics.Etp.v12.Datatypes.Object.ObjectPart",
                    "depends": []
                }
            }
        }
    ],
    "fullName": "Energistics.Etp.v12.Protocol.GrowingObject.GetPartsByRangeResponse",
    "depends": [
        "Energistics.Etp.v12.Datatypes.Object.ObjectPart"
    ]
}"#;
