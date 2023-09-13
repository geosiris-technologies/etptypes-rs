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

use crate::energistics::etp::v12::datatypes::data_value::DataValue;
use crate::helpers::Schemable;
use apache_avro::{from_avro_datum, from_value, AvroResult};
use std::io::Read;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct DataAttribute {
    #[serde(rename = "attributeId")]
    pub attribute_id: i32,

    #[serde(rename = "attributeValue")]
    pub attribute_value: DataValue,
}

impl Schemable for DataAttribute {
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

    fn avro_deserialize<R: Read>(input: &mut R) -> AvroResult<DataAttribute> {
        let record = from_avro_datum(&DataAttribute::avro_schema().unwrap(), input, None).unwrap();
        from_value::<DataAttribute>(&record)
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
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
        }
    ],
    "fullName": "Energistics.Etp.v12.Datatypes.DataAttribute",
    "depends": [
        "Energistics.Etp.v12.Datatypes.DataValue"
    ]
}"#;
