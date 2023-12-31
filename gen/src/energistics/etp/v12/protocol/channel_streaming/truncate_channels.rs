// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::*;
use apache_avro::{from_avro_datum, from_value, AvroResult};
use apache_avro::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::io::Read;
use std::time::SystemTime;

use crate::energistics::etp::v12::datatypes::channel_data::truncate_info::TruncateInfo;
use crate::helpers::ETPMetadata;
use crate::helpers::Schemable;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct TruncateChannels {
    #[serde(rename = "channels")]
    pub channels: Vec<TruncateInfo>,
}

impl Schemable for TruncateChannels {
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

impl ETPMetadata for TruncateChannels {
    fn protocol(&self) -> i32 {
        1
    }
    fn message_type(&self) -> i32 {
        5
    }
    fn sender_role(&self) -> Vec<Role> {
        vec![Role::Producer]
    }
    fn protocol_roles(&self) -> Vec<Role> {
        vec![Role::Producer, Role::Consumer]
    }
    fn multipart_flag(&self) -> bool {
        false
    }

    fn avro_deserialize<R: Read>(input: &mut R) -> AvroResult<TruncateChannels> {
        let record =
            from_avro_datum(&TruncateChannels::avro_schema().unwrap(), input, None).unwrap();
        from_value::<TruncateChannels>(&record)
    }
}

impl Default for TruncateChannels {
    /* Protocol 1, MessageType : 5 */
    fn default() -> TruncateChannels {
        TruncateChannels { channels: vec![] }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Protocol.ChannelStreaming",
    "name": "TruncateChannels",
    "protocol": "1",
    "messageType": "5",
    "senderRole": "producer",
    "protocolRoles": "producer,consumer",
    "multipartFlag": false,
    "fields": [
        {
            "name": "channels",
            "type": {
                "type": "array",
                "items": {
                    "type": "record",
                    "namespace": "Energistics.Etp.v12.Datatypes.ChannelData",
                    "name": "TruncateInfo",
                    "fields": [
                        {
                            "name": "channelId",
                            "type": "long"
                        },
                        {
                            "name": "newEndIndex",
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
                        }
                    ],
                    "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.TruncateInfo",
                    "depends": [
                        "Energistics.Etp.v12.Datatypes.IndexValue"
                    ]
                }
            }
        }
    ],
    "fullName": "Energistics.Etp.v12.Protocol.ChannelStreaming.TruncateChannels",
    "depends": [
        "Energistics.Etp.v12.Datatypes.ChannelData.TruncateInfo"
    ]
}"#;
