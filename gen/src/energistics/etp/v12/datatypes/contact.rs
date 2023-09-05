// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
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

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "Contact", "fields": [{"name": "organizationName", "type": "string", "default": ""}, {"name": "contactName", "type": "string", "default": ""}, {"name": "contactPhone", "type": "string", "default": ""}, {"name": "contactEmail", "type": "string", "default": ""}], "fullName": "Energistics.Etp.v12.Datatypes.Contact", "depends": []}"#;
