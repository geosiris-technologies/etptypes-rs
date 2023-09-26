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

use crate::energistics::etp::v12::datatypes::any_sparse_array::AnySparseArray;
use crate::energistics::etp::v12::datatypes::array_of_boolean::ArrayOfBoolean;
use crate::energistics::etp::v12::datatypes::array_of_bytes::ArrayOfBytes;
use crate::energistics::etp::v12::datatypes::array_of_double::ArrayOfDouble;
use crate::energistics::etp::v12::datatypes::array_of_float::ArrayOfFloat;
use crate::energistics::etp::v12::datatypes::array_of_int::ArrayOfInt;
use crate::energistics::etp::v12::datatypes::array_of_long::ArrayOfLong;
use crate::energistics::etp::v12::datatypes::array_of_nullable_boolean::ArrayOfNullableBoolean;
use crate::energistics::etp::v12::datatypes::array_of_nullable_int::ArrayOfNullableInt;
use crate::energistics::etp::v12::datatypes::array_of_nullable_long::ArrayOfNullableLong;
use crate::energistics::etp::v12::datatypes::array_of_string::ArrayOfString;
use crate::helpers::Schemable;
use apache_avro::{from_avro_datum, from_value, AvroResult};
use std::io::Read;
#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum UnionBooleanIntLongFloatDoubleStringArrayOfBooleanArrayOfNullableBooleanArrayOfIntArrayOfNullableIntArrayOfLongArrayOfNullableLongArrayOfFloatArrayOfDoubleArrayOfStringArrayOfBytesBytesAnySparseArray
{
    Boolean(bool),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    ArrayOfBoolean(ArrayOfBoolean),
    ArrayOfNullableBoolean(ArrayOfNullableBoolean),
    ArrayOfInt(ArrayOfInt),
    ArrayOfNullableInt(ArrayOfNullableInt),
    ArrayOfLong(ArrayOfLong),
    ArrayOfNullableLong(ArrayOfNullableLong),
    ArrayOfFloat(ArrayOfFloat),
    ArrayOfDouble(ArrayOfDouble),
    ArrayOfString(ArrayOfString),
    ArrayOfBytes(ArrayOfBytes),
    Bytes(Vec<u8>),
    AnySparseArray(AnySparseArray),
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct DataValue{

	#[serde(rename = "item")]
    pub item: Option<UnionBooleanIntLongFloatDoubleStringArrayOfBooleanArrayOfNullableBooleanArrayOfIntArrayOfNullableIntArrayOfLongArrayOfNullableLongArrayOfFloatArrayOfDoubleArrayOfStringArrayOfBytesBytesAnySparseArray>,

}

fn datavalue_avro_schema() -> Option<Schema> {
    match Schema::parse_str(AVRO_SCHEMA) {
        Ok(result) => Some(result),
        Err(e) => {
            panic!("{:?}", e);
        }
    }
}

impl Schemable for DataValue {
    fn avro_schema(&self) -> Option<Schema> {
        datavalue_avro_schema()
    }
    fn avro_schema_str(&self) -> &'static str {
        AVRO_SCHEMA
    }
}

impl AvroSerializable for DataValue {}

impl AvroDeserializable for DataValue {
    fn avro_deserialize<R: Read>(input: &mut R) -> AvroResult<DataValue> {
        let record = from_avro_datum(&datavalue_avro_schema().unwrap(), input, None).unwrap();
        from_value::<DataValue>(&record)
    }
}

impl Default for DataValue {
    /* Protocol , MessageType :  */
    fn default() -> DataValue {
        DataValue { item: None }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
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
}"#;
