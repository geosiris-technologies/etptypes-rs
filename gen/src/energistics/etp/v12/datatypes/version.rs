// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;


#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct Version{

	#[serde(rename = "major")]
    #[derivative(Default(value = "0" ))]
    pub major:i32,


	#[serde(rename = "minor")]
    #[derivative(Default(value = "0" ))]
    pub minor:i32,


	#[serde(rename = "revision")]
    #[derivative(Default(value = "0" ))]
    pub revision:i32,


	#[serde(rename = "patch")]
    #[derivative(Default(value = "0" ))]
    pub patch:i32,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "Version", "fields": [{"name": "major", "type": "int", "default": 0}, {"name": "minor", "type": "int", "default": 0}, {"name": "revision", "type": "int", "default": 0}, {"name": "patch", "type": "int", "default": 0}], "fullName": "Energistics.Etp.v12.Datatypes.Version", "depends": []}"#;

