// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::*;
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct PutResponse {
    #[serde(rename = "createdContainedObjectUris")]
    #[derivative(Default(value = "Vec::new()"))]
    pub created_contained_object_uris: Vec<String>,

    #[serde(rename = "deletedContainedObjectUris")]
    #[derivative(Default(value = "Vec::new()"))]
    pub deleted_contained_object_uris: Vec<String>,

    #[serde(rename = "joinedContainedObjectUris")]
    #[derivative(Default(value = "Vec::new()"))]
    pub joined_contained_object_uris: Vec<String>,

    #[serde(rename = "unjoinedContainedObjectUris")]
    #[derivative(Default(value = "Vec::new()"))]
    pub unjoined_contained_object_uris: Vec<String>,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "PutResponse", "fields": [{"name": "createdContainedObjectUris", "type": {"type": "array", "items": "string"}, "default": []}, {"name": "deletedContainedObjectUris", "type": {"type": "array", "items": "string"}, "default": []}, {"name": "joinedContainedObjectUris", "type": {"type": "array", "items": "string"}, "default": []}, {"name": "unjoinedContainedObjectUris", "type": {"type": "array", "items": "string"}, "default": []}], "fullName": "Energistics.Etp.v12.Datatypes.Object.PutResponse", "depends": []}"#;

impl Default for PutResponse {
    /* Protocol , MessageType :  */
    fn default() -> PutResponse {
        PutResponse {
            created_contained_object_uris: vec![],
            deleted_contained_object_uris: vec![],
            joined_contained_object_uris: vec![],
            unjoined_contained_object_uris: vec![],
        }
    }
}
