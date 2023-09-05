// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::{SystemTime};
use crate::helpers::*;


#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct DataArrayIdentifier{

	#[serde(rename = "uri")]
    pub uri:String,


	#[serde(rename = "pathInResource")]
    pub path_in_resource:String,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.DataArrayTypes", "name": "DataArrayIdentifier", "fields": [{"name": "uri", "type": "string"}, {"name": "pathInResource", "type": "string"}], "fullName": "Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArrayIdentifier", "depends": []}"#;

impl Default for DataArrayIdentifier{
    /* Protocol , MessageType :  */
    fn default()
    -> DataArrayIdentifier {
        DataArrayIdentifier {
            uri : "".to_string(),
            path_in_resource : "".to_string(),
        }
    }
}

