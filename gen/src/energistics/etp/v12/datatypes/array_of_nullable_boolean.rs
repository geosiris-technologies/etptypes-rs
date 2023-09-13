// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::Schemable;
use crate::helpers::*;
use apache_avro::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::SystemTime; // ['bool']
#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct ArrayOfNullableBoolean {
    #[serde(rename = "values")]
    pub values: Vec<Option<bool>>,
}

impl Schemable for ArrayOfNullableBoolean {
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

impl Default for ArrayOfNullableBoolean {
    /* Protocol , MessageType :  */
    fn default() -> ArrayOfNullableBoolean {
        ArrayOfNullableBoolean { values: vec![] }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Datatypes",
    "name": "ArrayOfNullableBoolean",
    "fields": [
        {
            "name": "values",
            "type": {
                "type": "array",
                "items": [
                    "null",
                    "boolean"
                ]
            }
        }
    ],
    "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfNullableBoolean",
    "depends": []
}"#;
