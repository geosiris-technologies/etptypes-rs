// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;

use crate::energistics::etp::v12::datatypes::channel_data::channel_data_kind::ChannelDataKind;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct AttributeMetadataRecord {
    #[serde(rename = "attributeId")]
    pub attribute_id: i32,

    #[serde(rename = "attributeName")]
    pub attribute_name: String,

    #[serde(rename = "dataKind")]
    pub data_kind: ChannelDataKind,

    #[serde(rename = "uom")]
    pub uom: String,

    #[serde(rename = "depthDatum")]
    pub depth_datum: String,

    #[serde(rename = "attributePropertyKindUri")]
    pub attribute_property_kind_uri: String,

    #[serde(rename = "axisVectorLengths")]
    pub axis_vector_lengths: Vec<i32>,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "AttributeMetadataRecord", "fields": [{"name": "attributeId", "type": "int"}, {"name": "attributeName", "type": "string"}, {"name": "dataKind", "type": {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "ChannelDataKind", "symbols": ["DateTime", "ElapsedTime", "MeasuredDepth", "PassIndexedDepth", "TrueVerticalDepth", "typeBoolean", "typeInt", "typeLong", "typeFloat", "typeDouble", "typeString", "typeBytes"], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.ChannelDataKind", "depends": []}}, {"name": "uom", "type": "string"}, {"name": "depthDatum", "type": "string"}, {"name": "attributePropertyKindUri", "type": "string"}, {"name": "axisVectorLengths", "type": {"type": "array", "items": "int"}}], "fullName": "Energistics.Etp.v12.Datatypes.AttributeMetadataRecord", "depends": ["Energistics.Etp.v12.Datatypes.ChannelData.ChannelDataKind"]}"#;
