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

use crate::helpers::Schemable;
use apache_avro::{from_avro_datum, from_value, AvroResult};
use std::io::Read;
#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct DataArrayIdentifier {
    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "pathInResource")]
    pub path_in_resource: String,
}

impl Schemable for DataArrayIdentifier {
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

    fn avro_deserialize<R: Read>(input: &mut R) -> AvroResult<DataArrayIdentifier> {
        let record =
            from_avro_datum(&DataArrayIdentifier::avro_schema().unwrap(), input, None).unwrap();
        from_value::<DataArrayIdentifier>(&record)
    }
}

impl Default for DataArrayIdentifier {
    /* Protocol , MessageType :  */
    fn default() -> DataArrayIdentifier {
        DataArrayIdentifier {
            uri: "".to_string(),
            path_in_resource: "".to_string(),
        }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
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
}"#;
