// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::*;
use apache_avro::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::SystemTime;

use crate::helpers::ETPMetadata;
use crate::helpers::Schemable;
use apache_avro::{from_avro_datum, from_value, AvroResult};
use std::io::Read;
#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct ReplaceRangeResponse {
    #[serde(rename = "channelChangeTime")]
    pub channel_change_time: i64,
}

impl Schemable for ReplaceRangeResponse {
    fn avro_schema() -> Option<Schema> {
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn avro_schema_str() -> &'static str {
        AVRO_SCHEMA
    }

    fn avro_deserialize<R: Read>(input: &mut R) -> AvroResult<ReplaceRangeResponse> {
        let record =
            from_avro_datum(&ReplaceRangeResponse::avro_schema().unwrap(), input, None).unwrap();
        from_value::<ReplaceRangeResponse>(&record)
    }
}

impl ETPMetadata for ReplaceRangeResponse {
    fn protocol(&self) -> i32 {
        22
    }
    fn message_type(&self) -> i32 {
        8
    }
    fn sender_role(&self) -> Vec<Role> {
        vec![Role::Store]
    }
    fn protocol_roles(&self) -> Vec<Role> {
        vec![Role::Store, Role::Customer]
    }
    fn multipart_flag(&self) -> bool {
        false
    }
}

impl Default for ReplaceRangeResponse {
    /* Protocol 22, MessageType : 8 */
    fn default() -> ReplaceRangeResponse {
        ReplaceRangeResponse {
            channel_change_time: time_to_etp(SystemTime::now()),
        }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Protocol.ChannelDataLoad",
    "name": "ReplaceRangeResponse",
    "protocol": "22",
    "messageType": "8",
    "senderRole": "store",
    "protocolRoles": "store,customer",
    "multipartFlag": false,
    "fields": [
        {
            "name": "channelChangeTime",
            "type": "long"
        }
    ],
    "fullName": "Energistics.Etp.v12.Protocol.ChannelDataLoad.ReplaceRangeResponse",
    "depends": []
}"#;
