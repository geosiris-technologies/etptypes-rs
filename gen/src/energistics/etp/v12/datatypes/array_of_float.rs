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
pub struct ArrayOfFloat {
    #[serde(rename = "values")]
    pub values: Vec<f32>,
}

impl Schemable for ArrayOfFloat {
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

    fn avro_deserialize<R: Read>(input: &mut R) -> AvroResult<ArrayOfFloat> {
        let record = from_avro_datum(&ArrayOfFloat::avro_schema().unwrap(), input, None).unwrap();
        from_value::<ArrayOfFloat>(&record)
    }
}

impl Default for ArrayOfFloat {
    /* Protocol , MessageType :  */
    fn default() -> ArrayOfFloat {
        ArrayOfFloat { values: vec![] }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Datatypes",
    "name": "ArrayOfFloat",
    "fields": [
        {
            "name": "values",
            "type": {
                "type": "array",
                "items": "float"
            }
        }
    ],
    "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfFloat",
    "depends": []
}"#;
