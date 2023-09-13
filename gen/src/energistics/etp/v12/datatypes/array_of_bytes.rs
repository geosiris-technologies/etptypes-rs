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
pub struct ArrayOfBytes {
    #[serde(rename = "values")]
    pub values: Vec<Vec<u8>>,
}

impl Schemable for ArrayOfBytes {
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

    fn avro_deserialize<R: Read>(input: &mut R) -> AvroResult<ArrayOfBytes> {
        let record = from_avro_datum(&ArrayOfBytes::avro_schema().unwrap(), input, None).unwrap();
        from_value::<ArrayOfBytes>(&record)
    }
}

impl Default for ArrayOfBytes {
    /* Protocol , MessageType :  */
    fn default() -> ArrayOfBytes {
        ArrayOfBytes { values: vec![] }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Datatypes",
    "name": "ArrayOfBytes",
    "fields": [
        {
            "name": "values",
            "type": {
                "type": "array",
                "items": "bytes"
            }
        }
    ],
    "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfBytes",
    "depends": []
}"#;
