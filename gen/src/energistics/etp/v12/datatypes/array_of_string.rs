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
pub struct ArrayOfString {
    #[serde(rename = "values")]
    pub values: Vec<String>,
}

impl Schemable for ArrayOfString {
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

impl Default for ArrayOfString {
    /* Protocol , MessageType :  */
    fn default() -> ArrayOfString {
        ArrayOfString { values: vec![] }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Datatypes",
    "name": "ArrayOfString",
    "fields": [
        {
            "name": "values",
            "type": {
                "type": "array",
                "items": "string"
            }
        }
    ],
    "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfString",
    "depends": []
}"#;
