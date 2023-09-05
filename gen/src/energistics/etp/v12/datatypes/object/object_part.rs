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
pub struct ObjectPart{

	#[serde(rename = "uid")]
    pub uid:String,


	#[serde(rename = "data")]
    pub data:Vec<u8>,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "ObjectPart", "fields": [{"name": "uid", "type": "string"}, {"name": "data", "type": "bytes"}], "fullName": "Energistics.Etp.v12.Datatypes.Object.ObjectPart", "depends": []}"#;

impl ObjectPart{
    /* Protocol , MessageType :  */
    pub fn default_with_params(_data: Vec<u8>, )
    -> ObjectPart {
        ObjectPart {
            uid : "".to_string(),
            data : _data,
        }
    }
}

