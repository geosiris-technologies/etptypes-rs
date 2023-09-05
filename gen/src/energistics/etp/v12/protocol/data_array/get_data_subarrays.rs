// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use avro_rs::{Schema, Error};
use crate::helpers::EtpMessageBody;

use crate::energistics::etp::v12::datatypes::data_array_types::get_data_subarrays_type::GetDataSubarraysType;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct GetDataSubarrays{

	#[serde(rename = "dataSubarrays")]
    pub data_subarrays:HashMap<String, GetDataSubarraysType,>,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.DataArray", "name": "GetDataSubarrays", "protocol": "9", "messageType": "3", "senderRole": "customer", "protocolRoles": "store,customer", "multipartFlag": false, "fields": [{"name": "dataSubarrays", "type": {"type": "map", "values": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.DataArrayTypes", "name": "GetDataSubarraysType", "fields": [{"name": "uid", "type": {"type": "record", "namespace": "Energistics.Etp.v12.Datatypes.DataArrayTypes", "name": "DataArrayIdentifier", "fields": [{"name": "uri", "type": "string"}, {"name": "pathInResource", "type": "string"}], "fullName": "Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArrayIdentifier", "depends": []}}, {"name": "starts", "type": {"type": "array", "items": "long"}, "default": []}, {"name": "counts", "type": {"type": "array", "items": "long"}, "default": []}], "fullName": "Energistics.Etp.v12.Datatypes.DataArrayTypes.GetDataSubarraysType", "depends": ["Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArrayIdentifier"]}}}], "fullName": "Energistics.Etp.v12.Protocol.DataArray.GetDataSubarrays", "depends": ["Energistics.Etp.v12.Datatypes.DataArrayTypes.GetDataSubarraysType"]}"#;

impl EtpMessageBody for GetDataSubarrays{
    fn avro_schema() -> Option<Schema>{
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) ->i32{
        9
    }
    fn message_type(&self) ->i32{
        3
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

