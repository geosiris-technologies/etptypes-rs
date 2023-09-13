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

use crate::energistics::etp::v12::datatypes::data_array_types::data_array::DataArray;
use crate::energistics::etp::v12::datatypes::data_array_types::data_array_identifier::DataArrayIdentifier;
use crate::energistics::etp::v12::datatypes::data_value::DataValue;
use crate::helpers::Schemable;
use apache_avro::{from_avro_datum, from_value, AvroResult};
use std::io::Read;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct PutDataArraysType {
    #[serde(rename = "uid")]
    pub uid: DataArrayIdentifier,

    #[serde(rename = "array")]
    pub array: DataArray,

    #[serde(rename = "customData")]
    #[derivative(Default(value = "HashMap::new()"))]
    pub custom_data: HashMap<String, DataValue>,
}

impl Schemable for PutDataArraysType {
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

    fn avro_deserialize<R: Read>(input: &mut R) -> AvroResult<PutDataArraysType> {
        let record =
            from_avro_datum(&PutDataArraysType::avro_schema().unwrap(), input, None).unwrap();
        from_value::<PutDataArraysType>(&record)
    }
}

impl PutDataArraysType {
    /* Protocol , MessageType :  */
    pub fn default_with_params(uid: DataArrayIdentifier, array: DataArray) -> PutDataArraysType {
        PutDataArraysType {
            uid,
            array,
            custom_data: HashMap::new(),
        }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Datatypes.DataArrayTypes",
    "name": "PutDataArraysType",
    "fields": [
        {
            "name": "uid",
            "type": {
                "type": "record",
                "namespace": "Energistics.Etp.v12.Datatypes.DataArrayTypes",
                "name": "DataArrayIdentifier",
                "fields": [
                    {
                        "name": "uri",
                        "type": "string"
                    },
                    {
                        "name": "pathInResource",
                        "type": "string"
                    }
                ],
                "fullName": "Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArrayIdentifier",
                "depends": []
            }
        },
        {
            "name": "array",
            "type": {
                "type": "record",
                "namespace": "Energistics.Etp.v12.Datatypes.DataArrayTypes",
                "name": "DataArray",
                "fields": [
                    {
                        "name": "dimensions",
                        "type": {
                            "type": "array",
                            "items": "long"
                        }
                    },
                    {
                        "name": "data",
                        "type": {
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
                        }
                    }
                ],
                "fullName": "Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArray",
                "depends": [
                    "Energistics.Etp.v12.Datatypes.AnyArray"
                ]
            }
        },
        {
            "name": "customData",
            "type": {
                "type": "map",
                "values": {
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
                                "Energistics.Etp.v12.Datatypes.ArrayOfBoolean",
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
                                "Energistics.Etp.v12.Datatypes.ArrayOfInt",
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
                                "Energistics.Etp.v12.Datatypes.ArrayOfLong",
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
                                "Energistics.Etp.v12.Datatypes.ArrayOfFloat",
                                "Energistics.Etp.v12.Datatypes.ArrayOfDouble",
                                "Energistics.Etp.v12.Datatypes.ArrayOfString",
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
                                                            "type": "Energistics.Etp.v12.Datatypes.AnyArray"
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
            "default": {}
        }
    ],
    "fullName": "Energistics.Etp.v12.Datatypes.DataArrayTypes.PutDataArraysType",
    "depends": [
        "Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArrayIdentifier",
        "Energistics.Etp.v12.Datatypes.DataArrayTypes.DataArray",
        "Energistics.Etp.v12.Datatypes.DataValue"
    ]
}"#;
