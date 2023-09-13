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

use crate::energistics::etp::v12::datatypes::channel_data::data_item::DataItem;
use crate::energistics::etp::v12::datatypes::object::index_interval::IndexInterval;
use crate::helpers::ETPMetadata;
use crate::helpers::Schemable;
use apache_avro::{from_avro_datum, from_value, AvroResult};
use std::io::Read;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct RangeReplaced {
    #[serde(rename = "changeTime")]
    pub change_time: i64,

    #[serde(rename = "channelIds")]
    pub channel_ids: Vec<i64>,

    #[serde(rename = "changedInterval")]
    pub changed_interval: IndexInterval,

    #[serde(rename = "data")]
    pub data: Vec<DataItem>,
}

impl Schemable for RangeReplaced {
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

    fn avro_deserialize<R: Read>(input: &mut R) -> AvroResult<RangeReplaced> {
        let record = from_avro_datum(&RangeReplaced::avro_schema().unwrap(), input, None).unwrap();
        from_value::<RangeReplaced>(&record)
    }
}

impl ETPMetadata for RangeReplaced {
    fn protocol(&self) -> i32 {
        21
    }
    fn message_type(&self) -> i32 {
        6
    }
    fn sender_role(&self) -> Vec<Role> {
        vec![Role::Store]
    }
    fn protocol_roles(&self) -> Vec<Role> {
        vec![Role::Store, Role::Customer]
    }
    fn multipart_flag(&self) -> bool {
        true
    }
}

impl RangeReplaced {
    /* Protocol 21, MessageType : 6 */
    pub fn default_with_params(changed_interval: IndexInterval) -> RangeReplaced {
        RangeReplaced {
            change_time: time_to_etp(SystemTime::now()),
            channel_ids: vec![],
            changed_interval,
            data: vec![],
        }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Protocol.ChannelSubscribe",
    "name": "RangeReplaced",
    "protocol": "21",
    "messageType": "6",
    "senderRole": "store",
    "protocolRoles": "store,customer",
    "multipartFlag": true,
    "fields": [
        {
            "name": "changeTime",
            "type": "long"
        },
        {
            "name": "channelIds",
            "type": {
                "type": "array",
                "items": "long"
            }
        },
        {
            "name": "changedInterval",
            "type": {
                "type": "record",
                "namespace": "Energistics.Etp.v12.Datatypes.Object",
                "name": "IndexInterval",
                "fields": [
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
                        "name": "endIndex",
                        "type": "Energistics.Etp.v12.Datatypes.IndexValue"
                    },
                    {
                        "name": "uom",
                        "type": "string"
                    },
                    {
                        "name": "depthDatum",
                        "type": "string",
                        "default": ""
                    }
                ],
                "fullName": "Energistics.Etp.v12.Datatypes.Object.IndexInterval",
                "depends": [
                    "Energistics.Etp.v12.Datatypes.IndexValue"
                ]
            }
        },
        {
            "name": "data",
            "type": {
                "type": "array",
                "items": {
                    "type": "record",
                    "namespace": "Energistics.Etp.v12.Datatypes.ChannelData",
                    "name": "DataItem",
                    "fields": [
                        {
                            "name": "channelId",
                            "type": "long"
                        },
                        {
                            "name": "indexes",
                            "type": {
                                "type": "array",
                                "items": "Energistics.Etp.v12.Datatypes.IndexValue"
                            }
                        },
                        {
                            "name": "value",
                            "type": {
                                "type": "record",
                                "namespace": "Energistics.Etp.v12.Datatypes",
                                "name": "DataValue",
                                "fields": [
                                    {
                                        "name": "item",
                                        "type": [
                                            "null",
                                            "boolean",
                                            "int",
                                            "long",
                                            "float",
                                            "double",
                                            "string",
                                            {
                                                "type": "record",
                                                "namespace": "Energistics.Etp.v12.Datatypes",
                                                "name": "ArrayOfBoolean",
                                                "fields": [
                                                    {
                                                        "name": "values",
                                                        "type": {
                                                            "type": "array",
                                                            "items": "boolean"
                                                        }
                                                    }
                                                ],
                                                "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfBoolean",
                                                "depends": []
                                            },
                                            {
                                                "type": "record",
                                                "namespace": "Energistics.Etp.v12.Datatypes",
                                                "name": "ArrayOfNullableBoolean",
                                                "fields": [
                                                    {
                                                        "name": "values",
                                                        "type": {
                                                            "type": "array",
                                                            "items": [
                                                                "null",
                                                                "boolean"
                                                            ]
                                                        }
                                                    }
                                                ],
                                                "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfNullableBoolean",
                                                "depends": []
                                            },
                                            {
                                                "type": "record",
                                                "namespace": "Energistics.Etp.v12.Datatypes",
                                                "name": "ArrayOfInt",
                                                "fields": [
                                                    {
                                                        "name": "values",
                                                        "type": {
                                                            "type": "array",
                                                            "items": "int"
                                                        }
                                                    }
                                                ],
                                                "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfInt",
                                                "depends": []
                                            },
                                            {
                                                "type": "record",
                                                "namespace": "Energistics.Etp.v12.Datatypes",
                                                "name": "ArrayOfNullableInt",
                                                "fields": [
                                                    {
                                                        "name": "values",
                                                        "type": {
                                                            "type": "array",
                                                            "items": [
                                                                "null",
                                                                "int"
                                                            ]
                                                        }
                                                    }
                                                ],
                                                "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfNullableInt",
                                                "depends": []
                                            },
                                            {
                                                "type": "record",
                                                "namespace": "Energistics.Etp.v12.Datatypes",
                                                "name": "ArrayOfLong",
                                                "fields": [
                                                    {
                                                        "name": "values",
                                                        "type": {
                                                            "type": "array",
                                                            "items": "long"
                                                        }
                                                    }
                                                ],
                                                "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfLong",
                                                "depends": []
                                            },
                                            {
                                                "type": "record",
                                                "namespace": "Energistics.Etp.v12.Datatypes",
                                                "name": "ArrayOfNullableLong",
                                                "fields": [
                                                    {
                                                        "name": "values",
                                                        "type": {
                                                            "type": "array",
                                                            "items": [
                                                                "null",
                                                                "long"
                                                            ]
                                                        }
                                                    }
                                                ],
                                                "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfNullableLong",
                                                "depends": []
                                            },
                                            {
                                                "type": "record",
                                                "namespace": "Energistics.Etp.v12.Datatypes",
                                                "name": "ArrayOfFloat",
                                                "fields": [
                                                    {
                                                        "name": "values",
                                                        "type": {
                                                            "type": "array",
                                                            "items": "float"
                                                        }
                                                    }
                                                ],
                                                "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfFloat",
                                                "depends": []
                                            },
                                            {
                                                "type": "record",
                                                "namespace": "Energistics.Etp.v12.Datatypes",
                                                "name": "ArrayOfDouble",
                                                "fields": [
                                                    {
                                                        "name": "values",
                                                        "type": {
                                                            "type": "array",
                                                            "items": "double"
                                                        }
                                                    }
                                                ],
                                                "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfDouble",
                                                "depends": []
                                            },
                                            {
                                                "type": "record",
                                                "namespace": "Energistics.Etp.v12.Datatypes",
                                                "name": "ArrayOfString",
                                                "fields": [
                                                    {
                                                        "name": "values",
                                                        "type": {
                                                            "type": "array",
                                                            "items": "string"
                                                        }
                                                    }
                                                ],
                                                "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfString",
                                                "depends": []
                                            },
                                            {
                                                "type": "record",
                                                "namespace": "Energistics.Etp.v12.Datatypes",
                                                "name": "ArrayOfBytes",
                                                "fields": [
                                                    {
                                                        "name": "values",
                                                        "type": {
                                                            "type": "array",
                                                            "items": "bytes"
                                                        }
                                                    }
                                                ],
                                                "fullName": "Energistics.Etp.v12.Datatypes.ArrayOfBytes",
                                                "depends": []
                                            },
                                            "bytes",
                                            {
                                                "type": "record",
                                                "namespace": "Energistics.Etp.v12.Datatypes",
                                                "name": "AnySparseArray",
                                                "fields": [
                                                    {
                                                        "name": "slices",
                                                        "type": {
                                                            "type": "array",
                                                            "items": {
                                                                "type": "record",
                                                                "namespace": "Energistics.Etp.v12.Datatypes",
                                                                "name": "AnySubarray",
                                                                "fields": [
                                                                    {
                                                                        "name": "start",
                                                                        "type": "long"
                                                                    },
                                                                    {
                                                                        "name": "slice",
                                                                        "type": {
                                                                            "type": "record",
                                                                            "namespace": "Energistics.Etp.v12.Datatypes",
                                                                            "name": "AnyArray",
                                                                            "fields": [
                                                                                {
                                                                                    "name": "item",
                                                                                    "type": [
                                                                                        "Energistics.Etp.v12.Datatypes.ArrayOfBoolean",
                                                                                        "Energistics.Etp.v12.Datatypes.ArrayOfInt",
                                                                                        "Energistics.Etp.v12.Datatypes.ArrayOfLong",
                                                                                        "Energistics.Etp.v12.Datatypes.ArrayOfFloat",
                                                                                        "Energistics.Etp.v12.Datatypes.ArrayOfDouble",
                                                                                        "Energistics.Etp.v12.Datatypes.ArrayOfString",
                                                                                        "bytes"
                                                                                    ]
                                                                                }
                                                                            ],
                                                                            "fullName": "Energistics.Etp.v12.Datatypes.AnyArray",
                                                                            "depends": [
                                                                                "Energistics.Etp.v12.Datatypes.ArrayOfBoolean",
                                                                                "Energistics.Etp.v12.Datatypes.ArrayOfInt",
                                                                                "Energistics.Etp.v12.Datatypes.ArrayOfLong",
                                                                                "Energistics.Etp.v12.Datatypes.ArrayOfFloat",
                                                                                "Energistics.Etp.v12.Datatypes.ArrayOfDouble",
                                                                                "Energistics.Etp.v12.Datatypes.ArrayOfString"
                                                                            ]
                                                                        }
                                                                    }
                                                                ],
                                                                "fullName": "Energistics.Etp.v12.Datatypes.AnySubarray",
                                                                "depends": [
                                                                    "Energistics.Etp.v12.Datatypes.AnyArray"
                                                                ]
                                                            }
                                                        }
                                                    }
                                                ],
                                                "fullName": "Energistics.Etp.v12.Datatypes.AnySparseArray",
                                                "depends": [
                                                    "Energistics.Etp.v12.Datatypes.AnySubarray"
                                                ]
                                            }
                                        ]
                                    }
                                ],
                                "fullName": "Energistics.Etp.v12.Datatypes.DataValue",
                                "depends": [
                                    "Energistics.Etp.v12.Datatypes.ArrayOfBoolean",
                                    "Energistics.Etp.v12.Datatypes.ArrayOfNullableBoolean",
                                    "Energistics.Etp.v12.Datatypes.ArrayOfInt",
                                    "Energistics.Etp.v12.Datatypes.ArrayOfNullableInt",
                                    "Energistics.Etp.v12.Datatypes.ArrayOfLong",
                                    "Energistics.Etp.v12.Datatypes.ArrayOfNullableLong",
                                    "Energistics.Etp.v12.Datatypes.ArrayOfFloat",
                                    "Energistics.Etp.v12.Datatypes.ArrayOfDouble",
                                    "Energistics.Etp.v12.Datatypes.ArrayOfString",
                                    "Energistics.Etp.v12.Datatypes.ArrayOfBytes",
                                    "Energistics.Etp.v12.Datatypes.AnySparseArray"
                                ]
                            }
                        },
                        {
                            "name": "valueAttributes",
                            "type": {
                                "type": "array",
                                "items": {
                                    "type": "record",
                                    "namespace": "Energistics.Etp.v12.Datatypes",
                                    "name": "DataAttribute",
                                    "fields": [
                                        {
                                            "name": "attributeId",
                                            "type": "int"
                                        },
                                        {
                                            "name": "attributeValue",
                                            "type": "Energistics.Etp.v12.Datatypes.DataValue"
                                        }
                                    ],
                                    "fullName": "Energistics.Etp.v12.Datatypes.DataAttribute",
                                    "depends": [
                                        "Energistics.Etp.v12.Datatypes.DataValue"
                                    ]
                                }
                            },
                            "default": []
                        }
                    ],
                    "fullName": "Energistics.Etp.v12.Datatypes.ChannelData.DataItem",
                    "depends": [
                        "Energistics.Etp.v12.Datatypes.IndexValue",
                        "Energistics.Etp.v12.Datatypes.DataValue",
                        "Energistics.Etp.v12.Datatypes.DataAttribute"
                    ]
                }
            }
        }
    ],
    "fullName": "Energistics.Etp.v12.Protocol.ChannelSubscribe.RangeReplaced",
    "depends": [
        "Energistics.Etp.v12.Datatypes.Object.IndexInterval",
        "Energistics.Etp.v12.Datatypes.ChannelData.DataItem"
    ]
}"#;
