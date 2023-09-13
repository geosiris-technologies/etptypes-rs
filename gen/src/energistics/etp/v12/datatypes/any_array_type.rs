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

use crate::helpers::Schemable;
use apache_avro::{from_avro_datum, from_value, AvroResult};
use std::fmt;
use std::io::Read;
use std::slice::Iter;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum AnyArrayType {
    /* None */
    ArrayOfBoolean,
    ArrayOfInt,
    ArrayOfLong,
    ArrayOfFloat,
    ArrayOfDouble,
    ArrayOfString,
    Bytes,
}

impl fmt::Display for AnyArrayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AnyArrayType::ArrayOfBoolean => "arrayOfBoolean",
                AnyArrayType::ArrayOfInt => "arrayOfInt",
                AnyArrayType::ArrayOfLong => "arrayOfLong",
                AnyArrayType::ArrayOfFloat => "arrayOfFloat",
                AnyArrayType::ArrayOfDouble => "arrayOfDouble",
                AnyArrayType::ArrayOfString => "arrayOfString",
                AnyArrayType::Bytes => "bytes",
            }
        )
    }
}

impl AnyArrayType {
    pub fn iter() -> Iter<'static, AnyArrayType> {
        static VEC_ENUM: [AnyArrayType; 7] = [
            AnyArrayType::ArrayOfBoolean,
            AnyArrayType::ArrayOfInt,
            AnyArrayType::ArrayOfLong,
            AnyArrayType::ArrayOfFloat,
            AnyArrayType::ArrayOfDouble,
            AnyArrayType::ArrayOfString,
            AnyArrayType::Bytes,
        ];
        VEC_ENUM.iter()
    }
}
