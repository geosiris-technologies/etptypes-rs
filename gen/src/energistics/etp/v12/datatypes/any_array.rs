// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::energistics::etp::v12::datatypes::array_of_boolean::ArrayOfBoolean;
use crate::energistics::etp::v12::datatypes::array_of_double::ArrayOfDouble;
use crate::energistics::etp::v12::datatypes::array_of_float::ArrayOfFloat;
use crate::energistics::etp::v12::datatypes::array_of_int::ArrayOfInt;
use crate::energistics::etp::v12::datatypes::array_of_long::ArrayOfLong;
use crate::energistics::etp::v12::datatypes::array_of_string::ArrayOfString;
use crate::helpers::Schemable;
use crate::helpers::*;
use apache_avro::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::SystemTime;
#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum UnionArrayOfBooleanArrayOfIntArrayOfLongArrayOfFloatArrayOfDoubleArrayOfStringBytes {
    ArrayOfBoolean(ArrayOfBoolean),
    ArrayOfInt(ArrayOfInt),
    ArrayOfLong(ArrayOfLong),
    ArrayOfFloat(ArrayOfFloat),
    ArrayOfDouble(ArrayOfDouble),
    ArrayOfString(ArrayOfString),
    Bytes(Vec<u8>),
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct AnyArray {
    #[serde(rename = "item")]
    pub item: UnionArrayOfBooleanArrayOfIntArrayOfLongArrayOfFloatArrayOfDoubleArrayOfStringBytes,
}

impl Schemable for AnyArray {
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

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Datatypes",
    "name": "AnyArray",
    "fields": [
        {
            "name": "item",
            "type": [
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
}"#;
