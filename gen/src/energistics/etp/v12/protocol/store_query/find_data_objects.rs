// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::{SystemTime};
use crate::helpers::*;
use avro_rs::{Schema, Error};
use crate::helpers::EtpMessageBody;

use crate::energistics::etp::v12::datatypes::object::context_info::ContextInfo;
use crate::energistics::etp::v12::datatypes::object::context_scope_kind::ContextScopeKind;
use crate::energistics::etp::v12::datatypes::object::active_status_kind::ActiveStatusKind;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct FindDataObjects{

	#[serde(rename = "context")]
    pub context:ContextInfo,


	#[serde(rename = "scope")]
    pub scope:ContextScopeKind,


	#[serde(rename = "storeLastWriteFilter")]
    pub store_last_write_filter:Option<i64>,


	#[serde(rename = "activeStatusFilter")]
    pub active_status_filter:Option<ActiveStatusKind>,


	#[serde(rename = "format")]
    #[derivative(Default(value = r#"String::from("xml")"# ))]
    pub format:String,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.StoreQuery", "name": "FindDataObjects", "protocol": "14", "messageType": "1", "senderRole": "customer", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "context", "type": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "ContextInfo", "fields": [{"name": "uri", "type": "string"}, {"name": "depth", "type": "int"}, {"name": "dataObjectTypes", "type": {"type": "array", "items": "string"}, "default": []}, {"name": "navigableEdges", "type": {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "RelationshipKind", "symbols": ["Primary", "Secondary", "Both"], "fullName": "Energistics.Etp.v12.Datatypes.Object.RelationshipKind", "depends": []}}, {"name": "includeSecondaryTargets", "type": "boolean", "default": false}, {"name": "includeSecondarySources", "type": "boolean", "default": false}], "fullName": "Energistics.Etp.v12.Datatypes.Object.ContextInfo", "depends": ["Energistics.Etp.v12.Datatypes.Object.RelationshipKind"]}}, {"name": "scope", "type": {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "ContextScopeKind", "symbols": ["self", "sources", "targets", "sourcesOrSelf", "targetsOrSelf"], "fullName": "Energistics.Etp.v12.Datatypes.Object.ContextScopeKind", "depends": []}}, {"name": "storeLastWriteFilter", "type": ["null", "long"]}, {"name": "activeStatusFilter", "type": ["null", {"type": "enum", "namespace": "Energistics.Etp.v12.Datatypes.Object", "name": "ActiveStatusKind", "symbols": ["Active", "Inactive"], "fullName": "Energistics.Etp.v12.Datatypes.Object.ActiveStatusKind", "depends": []}]}, {"name": "format", "type": "string", "default": "xml"}], "fullName": "Energistics.Etp.v12.Protocol.StoreQuery.FindDataObjects", "depends": ["Energistics.Etp.v12.Datatypes.Object.ContextInfo", "Energistics.Etp.v12.Datatypes.Object.ContextScopeKind", "Energistics.Etp.v12.Datatypes.Object.ActiveStatusKind"]}"#;

impl EtpMessageBody for FindDataObjects{
    fn avro_schema() -> Option<Schema>{
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) ->i32{
        14
    }
    fn message_type(&self) ->i32{
        1
    }
    fn sender_role(&self) ->String{
        "customer".to_string()
    }
    fn protocol_roles(&self) ->String{
        "store,customer".to_string()
    }
    fn multipart_flag(&self) ->bool{
        false
    }
}

impl FindDataObjects{
    /* Protocol 14, MessageType : 1 */
    pub fn default_with_params(_context: ContextInfo, _scope: ContextScopeKind, _store_last_write_filter: Option<i64>, _active_status_filter: Option<ActiveStatusKind>, )
    -> FindDataObjects {
        FindDataObjects {
            context : _context,
            scope : _scope,
            store_last_write_filter : _store_last_write_filter,
            active_status_filter : _active_status_filter,
            format : "xml".to_string(),
        }
    }
}

