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
pub struct ErrorInfo {
    #[serde(rename = "message")]
    pub message: String,

    #[serde(rename = "code")]
    pub code: i32,
}

impl Schemable for ErrorInfo {
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

impl ErrorInfo {
    /* Protocol , MessageType :  */
    pub fn default_with_params(code: i32) -> ErrorInfo {
        ErrorInfo {
            message: "".to_string(),
            code,
        }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Datatypes",
    "name": "ErrorInfo",
    "fields": [
        {
            "name": "message",
            "type": "string"
        },
        {
            "name": "code",
            "type": "int"
        }
    ],
    "fullName": "Energistics.Etp.v12.Datatypes.ErrorInfo",
    "depends": []
}"#;
