// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;

use crate::energistics::etp::v12::datatypes::channel_data::channel_index_kind::ChannelIndexKind;
use crate::energistics::etp::v12::datatypes::object::index_interval::IndexInterval;
use crate::energistics::etp::v12::datatypes::channel_data::index_direction::IndexDirection;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct IndexMetadataRecord{

	#[serde(rename = "indexKind")]
    pub index_kind:ChannelIndexKind,


	#[serde(rename = "interval")]
    pub interval:IndexInterval,


	#[serde(rename = "direction")]
    pub direction:IndexDirection,


	#[serde(rename = "name")]
    #[derivative(Default(value = r#"String::from("")"# ))]
    pub name:String,


	#[serde(rename = "uom")]
    pub uom:String,


	#[serde(rename = "depthDatum")]
    #[derivative(Default(value = r#"String::from("")"# ))]
    pub depth_datum:String,


	#[serde(rename = "indexPropertyKindUri")]
    pub index_property_kind_uri:String,


	#[serde(rename = "filterable")]
    #[derivative(Default(value = "true" ))]
    pub filterable:bool,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "IndexMetadataRecord", "fields": [{"name": "indexKind", "type": {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "ChannelIndexKind", "symbols": ["DateTime", "ElapsedTime", "MeasuredDepth", "TrueVerticalDepth", "PassIndexedDepth", "Pressure", "Temperature", "Scalar"], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.ChannelIndexKind", "depends": []}, "default": "DateTime"}, {"name": "interval", "type": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "IndexInterval", "fields": [{"name": "startIndex", "type": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "IndexValue", "fields": [{"name": "item", "type": ["null", "long", "double", {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "PassIndexedDepth", "fields": [{"name": "pass", "type": "long"}, {"name": "direction", "type": {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "PassDirection", "symbols": ["Up", "HoldingSteady", "Down"], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.PassDirection", "depends": []}}, {"name": "depth", "type": "double"}], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.PassIndexedDepth", "depends": ["Energistics.Etp.v12.Datatypes.ChannelData.PassDirection"]}]}], "fullName": "Energistics.Etp.v12.Datatypes.IndexValue", "depends": ["Energistics.Etp.v12.Datatypes.ChannelData.PassIndexedDepth"]}}, {"name": "endIndex", "type": "Energistics.Etp.v12.Datatypes.IndexValue"}, {"name": "uom", "type": "string"}, {"name": "depthDatum", "type": "string", "default": ""}], "fullName": "Energistics.Etp.v12.Datatypes.Object.IndexInterval", "depends": ["Energistics.Etp.v12.Datatypes.IndexValue"]}}, {"name": "direction", "type": {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.ChannelData", "name": "IndexDirection", "symbols": ["Increasing", "Decreasing", "Unordered"], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.IndexDirection", "depends": []}, "default": "Increasing"}, {"name": "name", "type": "string", "default": ""}, {"name": "uom", "type": "string"}, {"name": "depthDatum", "type": "string", "default": ""}, {"name": "indexPropertyKindUri", "type": "string"}, {"name": "filterable", "type": "boolean", "default": true}], "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.IndexMetadataRecord", "depends": ["Energistics.Etp.v12.Datatypes.ChannelData.ChannelIndexKind", "Energistics.Etp.v12.Datatypes.Object.IndexInterval", "Energistics.Etp.v12.Datatypes.ChannelData.IndexDirection"]}"#;

