// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::Schemable;
use crate::helpers::*;
use apache_avro::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::SystemTime;
#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct ChannelChangeRequestInfo {
    #[serde(rename = "sinceChangeTime")]
    pub since_change_time: i64,

    #[serde(rename = "channelIds")]
    pub channel_ids: Vec<i64>,
}

impl Schemable for ChannelChangeRequestInfo {
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
}

impl Default for ChannelChangeRequestInfo {
    /* Protocol , MessageType :  */
    fn default() -> ChannelChangeRequestInfo {
        ChannelChangeRequestInfo {
            since_change_time: time_to_etp(SystemTime::now()),
            channel_ids: vec![],
        }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Datatypes.ChannelData",
    "name": "ChannelChangeRequestInfo",
    "fields": [
        {
            "name": "sinceChangeTime",
            "type": "long"
        },
        {
            "name": "channelIds",
            "type": {
                "type": "array",
                "items": "long"
            }
        }
    ],
    "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.ChannelChangeRequestInfo",
    "depends": []
}"#;
