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
pub struct Contact {
    #[serde(rename = "organizationName")]
    #[derivative(Default(value = r#"String::from("")"#))]
    pub organization_name: String,

    #[serde(rename = "contactName")]
    #[derivative(Default(value = r#"String::from("")"#))]
    pub contact_name: String,

    #[serde(rename = "contactPhone")]
    #[derivative(Default(value = r#"String::from("")"#))]
    pub contact_phone: String,

    #[serde(rename = "contactEmail")]
    #[derivative(Default(value = r#"String::from("")"#))]
    pub contact_email: String,
}

impl Schemable for Contact {
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

impl Default for Contact {
    /* Protocol , MessageType :  */
    fn default() -> Contact {
        Contact {
            organization_name: "".to_string(),
            contact_name: "".to_string(),
            contact_phone: "".to_string(),
            contact_email: "".to_string(),
        }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Datatypes",
    "name": "Contact",
    "fields": [
        {
            "name": "organizationName",
            "type": "string",
            "default": ""
        },
        {
            "name": "contactName",
            "type": "string",
            "default": ""
        },
        {
            "name": "contactPhone",
            "type": "string",
            "default": ""
        },
        {
            "name": "contactEmail",
            "type": "string",
            "default": ""
        }
    ],
    "fullName": "Energistics.Etp.v12.Datatypes.Contact",
    "depends": []
}"#;
