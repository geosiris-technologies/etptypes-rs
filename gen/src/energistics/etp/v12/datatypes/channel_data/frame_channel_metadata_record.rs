// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::*;
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::SystemTime;

use crate::energistics::etp::v12::datatypes::attribute_metadata_record::AttributeMetadataRecord;
use crate::energistics::etp::v12::datatypes::channel_data::channel_data_kind::ChannelDataKind;
use crate::energistics::etp::v12::datatypes::data_value::DataValue;
use crate::energistics::etp::v12::datatypes::object::active_status_kind::ActiveStatusKind;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct FrameChannelMetadataRecord {
    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "channelName")]
    pub channel_name: String,

    #[serde(rename = "dataKind")]
    pub data_kind: ChannelDataKind,

    #[serde(rename = "uom")]
    pub uom: String,

    #[serde(rename = "depthDatum")]
    pub depth_datum: String,

    #[serde(rename = "channelPropertyKindUri")]
    pub channel_property_kind_uri: String,

    #[serde(rename = "status")]
    pub status: ActiveStatusKind,

    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "axisVectorLengths")]
    pub axis_vector_lengths: Vec<i32>,

    #[serde(rename = "attributeMetadata")]
    #[derivative(Default(value = "Vec::new()"))]
    pub attribute_metadata: Vec<AttributeMetadataRecord>,

    #[serde(rename = "customData")]
    #[derivative(Default(value = "HashMap::new()"))]
    pub custom_data: HashMap<String, DataValue>,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "FrameChannelMetadataRecord", "fields": [{"name": "uri", "type": "string"}, {"name": "channelName", "type": "string"}, {"name": "dataKind", "type": {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "ChannelDataKind", "symbols": ["DateTime", "ElapsedTime", "MeasuredDepth", "PassIndexedDepth", "TrueVerticalDepth", "typeBoolean", "typeInt", "typeLong", "typeFloat", "typeDouble", "typeString", "typeBytes"], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.ChannelDataKind", "depends": []}}, {"name": "uom", "type": "string"}, {"name": "depthDatum", "type": "string"}, {"name": "channelPropertyKindUri", "type": "string"}, {"name": "status", "type": {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "ActiveStatusKind", "symbols": ["Active", "Inactive"], "fullName": "Energistics.Etp.v12.Datatypes.Object.ActiveStatusKind", "depends": []}}, {"name": "source", "type": "string"}, {"name": "axisVectorLengths", "type": {"type": "array", "items": "int"}}, {"name": "attributeMetadata", "type": {"type": "array", "items": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "AttributeMetadataRecord", "fields": [{"name": "attributeId", "type": "int"}, {"name": "attributeName", "type": "string"}, {"name": "dataKind", "type": "Energistics.Etp.v12.Datatypes.ChannelData.ChannelDataKind"}, {"name": "uom", "type": "string"}, {"name": "depthDatum", "type": "string"}, {"name": "attributePropertyKindUri", "type": "string"}, {"name": "axisVectorLengths", "type": {"type": "array", "items": "int"}}], "fullName": "Energistics.Etp.v12.Datatypes.AttributeMetadataRecord", "depends": ["Energistics.Etp.v12.Datatypes.ChannelData.ChannelDataKind"]}}, "default": []}, {"name": "customData", "type": {"type": "map", "values": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "DataValue", "fields": [{"name": "item", "type": ["null", "boolean", "int", "long", "float", "double", "string", {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfBoolean", "fields": [{"name": "values", "type": {"type": "array", "items": "boolean"}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfBoolean", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfNullableBoolean", "fields": [{"name": "values", "type": {"type": "array", "items": ["null", "boolean"]}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfNullableBoolean", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfInt", "fields": [{"name": "values", "type": {"type": "array", "items": "int"}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfInt", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfNullableInt", "fields": [{"name": "values", "type": {"type": "array", "items": ["null", "int"]}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfNullableInt", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfLong", "fields": [{"name": "values", "type": {"type": "array", "items": "long"}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfLong", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfNullableLong", "fields": [{"name": "values", "type": {"type": "array", "items": ["null", "long"]}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfNullableLong", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfFloat", "fields": [{"name": "values", "type": {"type": "array", "items": "float"}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfFloat", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfDouble", "fields": [{"name": "values", "type": {"type": "array", "items": "double"}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfDouble", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfString", "fields": [{"name": "values", "type": {"type": "array", "items": "string"}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfString", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfBytes", "fields": [{"name": "values", "type": {"type": "array", "items": "bytes"}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfBytes", "depends": []}, "bytes", {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "AnySparseArray", "fields": [{"name": "slices", "type": {"type": "array", "items": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "AnySubarray", "fields": [{"name": "start", "type": "long"}, {"name": "slice", "type": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "AnyArray", "fields": [{"name": "item", "type": ["Energistics.Etp.v12.Datatypes.ArrayOfBoolean", "Energistics.Etp.v12.Datatypes.ArrayOfInt", "Energistics.Etp.v12.Datatypes.ArrayOfLong", "Energistics.Etp.v12.Datatypes.ArrayOfFloat", "Energistics.Etp.v12.Datatypes.ArrayOfDouble", "Energistics.Etp.v12.Datatypes.ArrayOfString", "bytes"]}], "fullName": "Energistics.Etp.v12.Datatypes.AnyArray", "depends": ["Energistics.Etp.v12.Datatypes.ArrayOfBoolean", "Energistics.Etp.v12.Datatypes.ArrayOfInt", "Energistics.Etp.v12.Datatypes.ArrayOfLong", "Energistics.Etp.v12.Datatypes.ArrayOfFloat", "Energistics.Etp.v12.Datatypes.ArrayOfDouble", "Energistics.Etp.v12.Datatypes.ArrayOfString"]}}], "fullName": "Energistics.Etp.v12.Datatypes.AnySubarray", "depends": ["Energistics.Etp.v12.Datatypes.AnyArray"]}}}], "fullName": "Energistics.Etp.v12.Datatypes.AnySparseArray", "depends": ["Energistics.Etp.v12.Datatypes.AnySubarray"]}]}], "fullName": "Energistics.Etp.v12.Datatypes.DataValue", "depends": ["Energistics.Etp.v12.Datatypes.ArrayOfBoolean", "Energistics.Etp.v12.Datatypes.ArrayOfNullableBoolean", "Energistics.Etp.v12.Datatypes.ArrayOfInt", "Energistics.Etp.v12.Datatypes.ArrayOfNullableInt", "Energistics.Etp.v12.Datatypes.ArrayOfLong", "Energistics.Etp.v12.Datatypes.ArrayOfNullableLong", "Energistics.Etp.v12.Datatypes.ArrayOfFloat", "Energistics.Etp.v12.Datatypes.ArrayOfDouble", "Energistics.Etp.v12.Datatypes.ArrayOfString", "Energistics.Etp.v12.Datatypes.ArrayOfBytes", "Energistics.Etp.v12.Datatypes.AnySparseArray"]}}, "default": {}}], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.FrameChannelMetadataRecord", "depends": ["Energistics.Etp.v12.Datatypes.ChannelData.ChannelDataKind", "Energistics.Etp.v12.Datatypes.Object.ActiveStatusKind", "Energistics.Etp.v12.Datatypes.AttributeMetadataRecord", "Energistics.Etp.v12.Datatypes.DataValue"]}"#;

impl FrameChannelMetadataRecord {
    /* Protocol , MessageType :  */
    pub fn default_with_params(
        data_kind: ChannelDataKind,
        status: ActiveStatusKind,
    ) -> FrameChannelMetadataRecord {
        FrameChannelMetadataRecord {
            uri: "".to_string(),
            channel_name: "".to_string(),
            data_kind,
            uom: "".to_string(),
            depth_datum: "".to_string(),
            channel_property_kind_uri: "".to_string(),
            status,
            source: "".to_string(),
            axis_vector_lengths: vec![],
            attribute_metadata: vec![],
            custom_data: HashMap::new(),
        }
    }
}
