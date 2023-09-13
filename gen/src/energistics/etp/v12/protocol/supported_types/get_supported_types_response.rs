// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::*;
use apache_avro::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::SystemTime;

use crate::energistics::etp::v12::datatypes::object::supported_type::SupportedType;
use crate::helpers::ETPMetadata;
use crate::helpers::Schemable;
use apache_avro::{from_avro_datum, from_value, AvroResult};
use std::io::Read;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct GetSupportedTypesResponse {
    #[serde(rename = "supportedTypes")]
    #[derivative(Default(value = "Vec::new()"))]
    pub supported_types: Vec<SupportedType>,
}

impl Schemable for GetSupportedTypesResponse {
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

    fn avro_deserialize<R: Read>(input: &mut R) -> AvroResult<GetSupportedTypesResponse> {
        let record = from_avro_datum(
            &GetSupportedTypesResponse::avro_schema().unwrap(),
            input,
            None,
        )
        .unwrap();
        from_value::<GetSupportedTypesResponse>(&record)
    }
}

impl ETPMetadata for GetSupportedTypesResponse {
    fn protocol(&self) -> i32 {
        25
    }
    fn message_type(&self) -> i32 {
        2
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
}

impl Default for GetSupportedTypesResponse {
    /* Protocol 25, MessageType : 2 */
    fn default() -> GetSupportedTypesResponse {
        GetSupportedTypesResponse {
            supported_types: vec![],
        }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Protocol.SupportedTypes",
    "name": "GetSupportedTypesResponse",
    "protocol": "25",
    "messageType": "2",
    "senderRole": "store",
    "protocolRoles": "store,customer",
    "multipartFlag": true,
    "fields": [
        {
            "name": "supportedTypes",
            "type": {
                "type": "array",
                "items": {
                    "type": "record",
                    "namespace": "Energistics.Etp.v12.Datatypes.Object",
                    "name": "SupportedType",
                    "fields": [
                        {
                            "name": "dataObjectType",
                            "type": "string"
                        },
                        {
                            "name": "objectCount",
                            "type": [
                                "null",
                                "int"
                            ]
                        },
                        {
                            "name": "relationshipKind",
                            "type": {
                                "type": "enum",
                                "namespace": "Energistics.Etp.v12.Datatypes.Object",
                                "name": "RelationshipKind",
                                "symbols": [
                                    "Primary",
                                    "Secondary",
                                    "Both"
                                ],
                                "fullName": "Energistics.Etp.v12.Datatypes.Object.RelationshipKind",
                                "depends": []
                            }
                        }
                    ],
                    "fullName": "Energistics.Etp.v12.Datatypes.Object.SupportedType",
                    "depends": [
                        "Energistics.Etp.v12.Datatypes.Object.RelationshipKind"
                    ]
                }
            },
            "default": []
        }
    ],
    "fullName": "Energistics.Etp.v12.Protocol.SupportedTypes.GetSupportedTypesResponse",
    "depends": [
        "Energistics.Etp.v12.Datatypes.Object.SupportedType"
    ]
}"#;
