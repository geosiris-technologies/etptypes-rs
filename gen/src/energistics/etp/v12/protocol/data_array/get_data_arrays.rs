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

use crate::energistics::etp::v12::datatypes::data_array_types::data_array_identifier::DataArrayIdentifier;
use crate::helpers::ETPMetadata;
use crate::helpers::Schemable;
use crate::protocols::ProtocolMessage;
use apache_avro::{from_avro_datum, from_value, AvroResult};
use std::io::Read;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct GetDataArrays {
    #[serde(rename = "dataArrays")]
    pub data_arrays: HashMap<String, DataArrayIdentifier>,
}

impl Schemable for GetDataArrays {
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

    fn avro_deserialize<R: Read>(input: &mut R) -> AvroResult<GetDataArrays> {
        let record = from_avro_datum(&GetDataArrays::avro_schema().unwrap(), input, None).unwrap();
        from_value::<GetDataArrays>(&record)
    }
}

impl ETPMetadata for GetDataArrays {
    fn protocol(&self) -> i32 {
        9
    }
    fn message_type(&self) -> i32 {
        2
    }
    fn sender_role(&self) -> Vec<Role> {
        vec![Role::Customer]
    }
    fn protocol_roles(&self) -> Vec<Role> {
        vec![Role::Store, Role::Customer]
    }
    fn multipart_flag(&self) -> bool {
        false
    }
}

impl GetDataArrays {
    pub fn as_protocol_message(&self) -> ProtocolMessage {
        ProtocolMessage::DataArray_GetDataArrays(self.clone())
    }
}

impl Default for GetDataArrays {
    /* Protocol 9, MessageType : 2 */
    fn default() -> GetDataArrays {
        GetDataArrays {
            data_arrays: HashMap::new(),
        }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Protocol.DataArray",
    "name": "GetDataArrays",
    "protocol": "9",
    "messageType": "2",
    "senderRole": "customer",
    "protocolRoles": "store,customer",
    "multipartFlag": false,
    "fields": [
        {
            "name": "dataArrays",
            "type": {
                "type": "map",
                "values": {
                    "type": "record",
                    "namespace": "Energistics.Etp.v12.Datatypes.DataArrayTypes",
                    "name": "DataArrayIdentifier",
                    "fields": [
                        {
                            "name": "uri",
                            "type": "string"
                        },
                        {
                            "name": "pathInResource",
                            "type": "string"
                        }
                    ],
                    "fullName": "Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArrayIdentifier",
                    "depends": []
                }
            }
        }
    ],
    "fullName": "Energistics.Etp.v12.Protocol.DataArray.GetDataArrays",
    "depends": [
        "Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArrayIdentifier"
    ]
}"#;
