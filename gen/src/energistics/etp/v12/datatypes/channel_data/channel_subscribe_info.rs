// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::energistics::etp::v12::datatypes::index_value::IndexValue;
use crate::helpers::Schemable;
use crate::helpers::*;
use apache_avro::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct ChannelSubscribeInfo {
    #[serde(rename = "channelId")]
    pub channel_id: i64,

    #[serde(rename = "startIndex")]
    pub start_index: IndexValue,

    #[serde(rename = "dataChanges")]
    #[derivative(Default(value = "true"))]
    pub data_changes: bool,

    #[serde(rename = "requestLatestIndexCount")]
    pub request_latest_index_count: Option<i32>,
}

impl Schemable for ChannelSubscribeInfo {
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

impl ChannelSubscribeInfo {
    /* Protocol , MessageType :  */
    pub fn default_with_params(
        channel_id: i64,
        start_index: IndexValue,
        request_latest_index_count: Option<i32>,
    ) -> ChannelSubscribeInfo {
        ChannelSubscribeInfo {
            channel_id,
            start_index,
            data_changes: true,
            request_latest_index_count,
        }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Datatypes.ChannelData",
    "name": "ChannelSubscribeInfo",
    "fields": [
        {
            "name": "channelId",
            "type": "long"
        },
        {
            "name": "startIndex",
            "type": {
                "type": "record",
                "namespace": "Energistics.Etp.v12.Datatypes",
                "name": "IndexValue",
                "fields": [
                    {
                        "name": "item",
                        "type": [
                            "null",
                            "long",
                            "double",
                            {
                                "type": "record",
                                "namespace": "Energistics.Etp.v12.Datatypes.ChannelData",
                                "name": "PassIndexedDepth",
                                "fields": [
                                    {
                                        "name": "pass",
                                        "type": "long"
                                    },
                                    {
                                        "name": "direction",
                                        "type": {
                                            "type": "enum",
                                            "namespace": "Energistics.Etp.v12.Datatypes.ChannelData",
                                            "name": "PassDirection",
                                            "symbols": [
                                                "Up",
                                                "HoldingSteady",
                                                "Down"
                                            ],
                                            "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.PassDirection",
                                            "depends": []
                                        }
                                    },
                                    {
                                        "name": "depth",
                                        "type": "double"
                                    }
                                ],
                                "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.PassIndexedDepth",
                                "depends": [
                                    "Energistics.Etp.v12.Datatypes.ChannelData.PassDirection"
                                ]
                            }
                        ]
                    }
                ],
                "fullName": "Energistics.Etp.v12.Datatypes.IndexValue",
                "depends": [
                    "Energistics.Etp.v12.Datatypes.ChannelData.PassIndexedDepth"
                ]
            }
        },
        {
            "name": "dataChanges",
            "type": "boolean",
            "default": true
        },
        {
            "name": "requestLatestIndexCount",
            "type": [
                "null",
                "int"
            ]
        }
    ],
    "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.ChannelSubscribeInfo",
    "depends": [
        "Energistics.Etp.v12.Datatypes.IndexValue"
    ]
}"#;
