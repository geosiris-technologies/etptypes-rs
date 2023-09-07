// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::*;
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::SystemTime;

use crate::energistics::etp::v12::datatypes::object::relationship_kind::RelationshipKind;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct ContextInfo {
    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "depth")]
    pub depth: i32,

    #[serde(rename = "dataObjectTypes")]
    #[derivative(Default(value = "Vec::new()"))]
    pub data_object_types: Vec<String>,

    #[serde(rename = "navigableEdges")]
    pub navigable_edges: RelationshipKind,

    #[serde(rename = "includeSecondaryTargets")]
    #[derivative(Default(value = "false"))]
    pub include_secondary_targets: bool,

    #[serde(rename = "includeSecondarySources")]
    #[derivative(Default(value = "false"))]
    pub include_secondary_sources: bool,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "ContextInfo", "fields": [{"name": "uri", "type": "string"}, {"name": "depth", "type": "int"}, {"name": "dataObjectTypes", "type": {"type": "array", "items": "string"}, "default": []}, {"name": "navigableEdges", "type": {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "RelationshipKind", "symbols": ["Primary", "Secondary", "Both"], "fullName": "Energistics.Etp.v12.Datatypes.Object.RelationshipKind", "depends": []}}, {"name": "includeSecondaryTargets", "type": "boolean", "default": false}, {"name": "includeSecondarySources", "type": "boolean", "default": false}], "fullName": "Energistics.Etp.v12.Datatypes.Object.ContextInfo", "depends": ["Energistics.Etp.v12.Datatypes.Object.RelationshipKind"]}"#;

impl ContextInfo {
    /* Protocol , MessageType :  */
    pub fn default_with_params(depth: i32, navigable_edges: RelationshipKind) -> ContextInfo {
        ContextInfo {
            uri: "".to_string(),
            depth,
            data_object_types: vec![],
            navigable_edges,
            include_secondary_targets: false,
            include_secondary_sources: false,
        }
    }
}
