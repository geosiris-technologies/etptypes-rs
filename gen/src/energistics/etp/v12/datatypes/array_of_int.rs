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
use std::time::SystemTime;
#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct ArrayOfInt {
    #[serde(rename = "values")]
    pub values: Vec<i32>,
}

impl Schemable for ArrayOfInt {
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

impl Default for ArrayOfInt {
    /* Protocol , MessageType :  */
    fn default() -> ArrayOfInt {
        ArrayOfInt { values: vec![] }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Datatypes",
    "name": "ArrayOfInt",
    "fields": [
        {
            "name": "values",
            "type": {
                "type": "array",
                "items": "int"
            }
        }
    ],
    "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfInt",
    "depends": []
}"#;
