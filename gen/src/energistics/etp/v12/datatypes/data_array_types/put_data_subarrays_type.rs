// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::energistics::etp::v12::datatypes::any_array::AnyArray;
use crate::energistics::etp::v12::datatypes::data_array_types::data_array_identifier::DataArrayIdentifier;
use crate::helpers::*;
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct PutDataSubarraysType {
    #[serde(rename = "uid")]
    pub uid: DataArrayIdentifier,

    #[serde(rename = "data")]
    pub data: AnyArray,

    #[serde(rename = "starts")]
    pub starts: Vec<i64>,

    #[serde(rename = "counts")]
    pub counts: Vec<i64>,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.DataArrayTypes", "name": "PutDataSubarraysType", "fields": [{"name": "uid", "type": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.DataArrayTypes", "name": "DataArrayIdentifier", "fields": [{"name": "uri", "type": "string"}, {"name": "pathInResource", "type": "string"}], "fullName": "Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArrayIdentifier", "depends": []}}, {"name": "data", "type": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "AnyArray", "fields": [{"name": "item", "type": [{"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfBoolean", "fields": [{"name": "values", "type": {"type": "array", "items": "boolean"}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfBoolean", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfInt", "fields": [{"name": "values", "type": {"type": "array", "items": "int"}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfInt", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfLong", "fields": [{"name": "values", "type": {"type": "array", "items": "long"}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfLong", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfFloat", "fields": [{"name": "values", "type": {"type": "array", "items": "float"}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfFloat", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfDouble", "fields": [{"name": "values", "type": {"type": "array", "items": "double"}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfDouble", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfString", "fields": [{"name": "values", "type": {"type": "array", "items": "string"}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfString", "depends": []}, "bytes"]}], "fullName": "Energistics.Etp.v12.Datatypes.AnyArray", "depends": ["Energistics.Etp.v12.Datatypes.ArrayOfBoolean", "Energistics.Etp.v12.Datatypes.ArrayOfInt", "Energistics.Etp.v12.Datatypes.ArrayOfLong", "Energistics.Etp.v12.Datatypes.ArrayOfFloat", "Energistics.Etp.v12.Datatypes.ArrayOfDouble", "Energistics.Etp.v12.Datatypes.ArrayOfString"]}}, {"name": "starts", "type": {"type": "array", "items": "long"}}, {"name": "counts", "type": {"type": "array", "items": "long"}}], "fullName": "Energistics.Etp.v12.Datatypes.DataArrayTypes.PutDataSubarraysType", "depends": ["Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArrayIdentifier", "Energistics.Etp.v12.Datatypes.AnyArray"]}"#;

impl PutDataSubarraysType {
    /* Protocol , MessageType :  */
    pub fn default_with_params(uid: DataArrayIdentifier, data: AnyArray) -> PutDataSubarraysType {
        PutDataSubarraysType {
            uid,
            data,
            starts: vec![],
            counts: vec![],
        }
    }
}
