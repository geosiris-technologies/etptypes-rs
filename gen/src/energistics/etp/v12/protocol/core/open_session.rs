// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::ETPMetadata;
use crate::helpers::*;
use avro_rs::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::SystemTime;

use crate::energistics::etp::v12::datatypes::data_value::DataValue;
use crate::energistics::etp::v12::datatypes::supported_data_object::SupportedDataObject;
use crate::energistics::etp::v12::datatypes::supported_protocol::SupportedProtocol;
use crate::energistics::etp::v12::datatypes::uuid::Uuid;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct OpenSession {
    #[serde(rename = "applicationName")]
    pub application_name: String,

    #[serde(rename = "applicationVersion")]
    pub application_version: String,

    #[serde(rename = "serverInstanceId")]
    pub server_instance_id: Uuid,

    #[serde(rename = "supportedProtocols")]
    pub supported_protocols: Vec<SupportedProtocol>,

    #[serde(rename = "supportedDataObjects")]
    pub supported_data_objects: Vec<SupportedDataObject>,

    #[serde(rename = "supportedCompression")]
    #[derivative(Default(value = r#"String::from("")"#))]
    pub supported_compression: String,

    #[serde(rename = "supportedFormats")]
    #[derivative(Default(value = "Vec::new()"))]
    pub supported_formats: Vec<String>,

    #[serde(rename = "currentDateTime")]
    pub current_date_time: i64,

    #[serde(rename = "earliestRetainedChangeTime")]
    pub earliest_retained_change_time: i64,

    #[serde(rename = "sessionId")]
    pub session_id: Uuid,

    #[serde(rename = "endpointCapabilities")]
    #[derivative(Default(value = "HashMap::new()"))]
    pub endpoint_capabilities: HashMap<String, DataValue>,
}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.Core", "name": "OpenSession", "protocol": "0", "messageType": "2", "senderRole": "server", "protocolRoles": "client, server", "multipartFlag": false, "fields": [{"name": "applicationName", "type": "string"}, {"name": "applicationVersion", "type": "string"}, {"name": "serverInstanceId", "type": {"type": "fixed", "namespace": "Energistics.Etp.v12.Datatypes", "name": "Uuid", "size": 16, "fullName": "Energistics.Etp.v12.Datatypes.Uuid", "depends": []}}, {"name": "supportedProtocols", "type": {"type": "array", "items": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "SupportedProtocol", "fields": [{"name": "protocol", "type": "int"}, {"name": "protocolVersion", "type": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "Version", "fields": [{"name": "major", "type": "int", "default": 0}, {"name": "minor", "type": "int", "default": 0}, {"name": "revision", "type": "int", "default": 0}, {"name": "patch", "type": "int", "default": 0}], "fullName": "Energistics.Etp.v12.Datatypes.Version", "depends": []}}, {"name": "role", "type": "string"}, {"name": "protocolCapabilities", "type": {"type": "map", "values": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "DataValue", "fields": [{"name": "item", "type": ["null", "boolean", "int", "long", "float", "double", "string", {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfBoolean", "fields": [{"name": "values", "type": {"type": "array", "items": "boolean"}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfBoolean", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfNullableBoolean", "fields": [{"name": "values", "type": {"type": "array", "items": ["null", "boolean"]}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfNullableBoolean", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfInt", "fields": [{"name": "values", "type": {"type": "array", "items": "int"}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfInt", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfNullableInt", "fields": [{"name": "values", "type": {"type": "array", "items": ["null", "int"]}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfNullableInt", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfLong", "fields": [{"name": "values", "type": {"type": "array", "items": "long"}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfLong", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfNullableLong", "fields": [{"name": "values", "type": {"type": "array", "items": ["null", "long"]}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfNullableLong", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfFloat", "fields": [{"name": "values", "type": {"type": "array", "items": "float"}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfFloat", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfDouble", "fields": [{"name": "values", "type": {"type": "array", "items": "double"}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfDouble", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfString", "fields": [{"name": "values", "type": {"type": "array", "items": "string"}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfString", "depends": []}, {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "ArrayOfBytes", "fields": [{"name": "values", "type": {"type": "array", "items": "bytes"}}], "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfBytes", "depends": []}, "bytes", {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "AnySparseArray", "fields": [{"name": "slices", "type": {"type": "array", "items": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "AnySubarray", "fields": [{"name": "start", "type": "long"}, {"name": "slice", "type": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "AnyArray", "fields": [{"name": "item", "type": ["Energistics.Etp.v12.Datatypes.ArrayOfBoolean", "Energistics.Etp.v12.Datatypes.ArrayOfInt", "Energistics.Etp.v12.Datatypes.ArrayOfLong", "Energistics.Etp.v12.Datatypes.ArrayOfFloat", "Energistics.Etp.v12.Datatypes.ArrayOfDouble", "Energistics.Etp.v12.Datatypes.ArrayOfString", "bytes"]}], "fullName": "Energistics.Etp.v12.Datatypes.AnyArray", "depends": ["Energistics.Etp.v12.Datatypes.ArrayOfBoolean", "Energistics.Etp.v12.Datatypes.ArrayOfInt", "Energistics.Etp.v12.Datatypes.ArrayOfLong", "Energistics.Etp.v12.Datatypes.ArrayOfFloat", "Energistics.Etp.v12.Datatypes.ArrayOfDouble", "Energistics.Etp.v12.Datatypes.ArrayOfString"]}}], "fullName": "Energistics.Etp.v12.Datatypes.AnySubarray", "depends": ["Energistics.Etp.v12.Datatypes.AnyArray"]}}}], "fullName": "Energistics.Etp.v12.Datatypes.AnySparseArray", "depends": ["Energistics.Etp.v12.Datatypes.AnySubarray"]}]}], "fullName": "Energistics.Etp.v12.Datatypes.DataValue", "depends": ["Energistics.Etp.v12.Datatypes.ArrayOfBoolean", "Energistics.Etp.v12.Datatypes.ArrayOfNullableBoolean", "Energistics.Etp.v12.Datatypes.ArrayOfInt", "Energistics.Etp.v12.Datatypes.ArrayOfNullableInt", "Energistics.Etp.v12.Datatypes.ArrayOfLong", "Energistics.Etp.v12.Datatypes.ArrayOfNullableLong", "Energistics.Etp.v12.Datatypes.ArrayOfFloat", "Energistics.Etp.v12.Datatypes.ArrayOfDouble", "Energistics.Etp.v12.Datatypes.ArrayOfString", "Energistics.Etp.v12.Datatypes.ArrayOfBytes", "Energistics.Etp.v12.Datatypes.AnySparseArray"]}}, "default": {}}], "fullName": "Energistics.Etp.v12.Datatypes.SupportedProtocol", "depends": ["Energistics.Etp.v12.Datatypes.Version", "Energistics.Etp.v12.Datatypes.DataValue"]}}}, {"name": "supportedDataObjects", "type": {"type": "array", "items": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes", "name": "SupportedDataObject", "fields": [{"name": "qualifiedType", "type": "string"}, {"name": "dataObjectCapabilities", "type": {"type": "map", "values": "Energistics.Etp.v12.Datatypes.DataValue"}, "default": {}}], "fullName": "Energistics.Etp.v12.Datatypes.SupportedDataObject", "depends": ["Energistics.Etp.v12.Datatypes.DataValue"]}}}, {"name": "supportedCompression", "type": "string", "default": ""}, {"name": "supportedFormats", "type": {"type": "array", "items": "string"}, "default": ["xml"]}, {"name": "currentDateTime", "type": "long"}, {"name": "earliestRetainedChangeTime", "type": "long"}, {"name": "sessionId", "type": "Energistics.Etp.v12.Datatypes.Uuid"}, {"name": "endpointCapabilities", "type": {"type": "map", "values": "Energistics.Etp.v12.Datatypes.DataValue"}, "default": {}}], "fullName": "Energistics.Etp.v12.Protocol.Core.OpenSession", "depends": ["Energistics.Etp.v12.Datatypes.Uuid", "Energistics.Etp.v12.Datatypes.SupportedProtocol", "Energistics.Etp.v12.Datatypes.SupportedDataObject", "Energistics.Etp.v12.Datatypes.DataValue"]}"#;

impl ETPMetadata for OpenSession {
    fn avro_schema() -> Option<Schema> {
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) -> i32 {
        0
    }
    fn message_type(&self) -> i32 {
        2
    }
    fn sender_role(&self) -> Vec<Role> {
        vec![Role::Server]
    }
    fn protocol_roles(&self) -> Vec<Role> {
        vec![Role::Client, Role::Server]
    }
    fn multipart_flag(&self) -> bool {
        false
    }
}

impl Default for OpenSession {
    /* Protocol 0, MessageType : 2 */
    fn default() -> OpenSession {
        OpenSession {
            application_name: "".to_string(),
            application_version: "".to_string(),
            server_instance_id: Uuid::new_v4(),
            supported_protocols: vec![],
            supported_data_objects: vec![],
            supported_compression: "".to_string(),
            supported_formats: vec!["xml".to_string()],
            current_date_time: time_to_etp(SystemTime::now()),
            earliest_retained_change_time: time_to_etp(SystemTime::now()),
            session_id: Uuid::new_v4(),
            endpoint_capabilities: HashMap::new(),
        }
    }
}
