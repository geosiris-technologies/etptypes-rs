// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::EtpMessageBody;
use avro_rs::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;

use crate::energistics::etp::v12::datatypes::object::active_status_kind::ActiveStatusKind;
use crate::energistics::etp::v12::datatypes::object::context_info::ContextInfo;
use crate::energistics::etp::v12::datatypes::object::context_scope_kind::ContextScopeKind;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct GetResources {
    #[serde(rename = "context")]
    pub context: ContextInfo,

    #[serde(rename = "scope")]
    pub scope: ContextScopeKind,

    #[serde(rename = "countObjects")]
    #[derivative(Default(value = "false"))]
    pub count_objects: bool,

    #[serde(rename = "storeLastWriteFilter")]
    pub store_last_write_filter: Option<i64>,

    #[serde(rename = "activeStatusFilter")]
    pub active_status_filter: Option<ActiveStatusKind>,

    #[serde(rename = "includeEdges")]
    #[derivative(Default(value = "false"))]
    pub include_edges: bool,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.Discovery", "name": "GetResources", "protocol": "3", "messageType": "1", "senderRole": "customer", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "context", "type": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "ContextInfo", "fields": [{"name": "uri", "type": "string"}, {"name": "depth", "type": "int"}, {"name": "dataObjectTypes", "type": {"type": "array", "items": "string"}, "default": []}, {"name": "navigableEdges", "type": {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "RelationshipKind", "symbols": ["Primary", "Secondary", "Both"], "fullName": "Energistics.Etp.v12.Datatypes.Object.RelationshipKind", "depends": []}}, {"name": "includeSecondaryTargets", "type": "boolean", "default": false}, {"name": "includeSecondarySources", "type": "boolean", "default": false}], "fullName": "Energistics.Etp.v12.Datatypes.Object.ContextInfo", "depends": ["Energistics.Etp.v12.Datatypes.Object.RelationshipKind"]}}, {"name": "scope", "type": {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "ContextScopeKind", "symbols": ["self", "sources", "targets", "sourcesOrSelf", "targetsOrSelf"], "fullName": "Energistics.Etp.v12.Datatypes.Object.ContextScopeKind", "depends": []}}, {"name": "countObjects", "type": "boolean", "default": false}, {"name": "storeLastWriteFilter", "type": ["null", "long"]}, {"name": "activeStatusFilter", "type": ["null", {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "ActiveStatusKind", "symbols": ["Active", "Inactive"], "fullName": "Energistics.Etp.v12.Datatypes.Object.ActiveStatusKind", "depends": []}]}, {"name": "includeEdges", "type": "boolean", "default": false}], "fullName": "Energistics.Etp.v12.Protocol.Discovery.GetResources", "depends": ["Energistics.Etp.v12.Datatypes.Object.ContextInfo", "Energistics.Etp.v12.Datatypes.Object.ContextScopeKind", "Energistics.Etp.v12.Datatypes.Object.ActiveStatusKind"]}"#;

impl EtpMessageBody for GetResources {
    fn avro_schema() -> Option<Schema> {
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) -> i32 {
        3
    }
    fn message_type(&self) -> i32 {
        1
    }
    fn sender_role(&self) -> String {
        "customer".to_string()
    }
    fn protocol_roles(&self) -> String {
        "store,customer".to_string()
    }
    fn multipart_flag(&self) -> bool {
        false
    }
}