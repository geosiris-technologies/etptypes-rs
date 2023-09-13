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
use std::fmt;
use std::slice::Iter;
use std::time::SystemTime;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum AnyLogicalArrayType {
    /* None */
    ArrayOfBoolean,
    ArrayOfInt8,
    ArrayOfUInt8,
    ArrayOfInt16LE,
    ArrayOfInt32LE,
    ArrayOfInt64LE,
    ArrayOfUInt16LE,
    ArrayOfUInt32LE,
    ArrayOfUInt64LE,
    ArrayOfFloat32LE,
    ArrayOfDouble64LE,
    ArrayOfInt16BE,
    ArrayOfInt32BE,
    ArrayOfInt64BE,
    ArrayOfUInt16BE,
    ArrayOfUInt32BE,
    ArrayOfUInt64BE,
    ArrayOfFloat32BE,
    ArrayOfDouble64BE,
    ArrayOfString,
    ArrayOfCustom,
}

impl fmt::Display for AnyLogicalArrayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AnyLogicalArrayType::ArrayOfBoolean => "arrayOfBoolean",
                AnyLogicalArrayType::ArrayOfInt8 => "arrayOfInt8",
                AnyLogicalArrayType::ArrayOfUInt8 => "arrayOfUInt8",
                AnyLogicalArrayType::ArrayOfInt16LE => "arrayOfInt16LE",
                AnyLogicalArrayType::ArrayOfInt32LE => "arrayOfInt32LE",
                AnyLogicalArrayType::ArrayOfInt64LE => "arrayOfInt64LE",
                AnyLogicalArrayType::ArrayOfUInt16LE => "arrayOfUInt16LE",
                AnyLogicalArrayType::ArrayOfUInt32LE => "arrayOfUInt32LE",
                AnyLogicalArrayType::ArrayOfUInt64LE => "arrayOfUInt64LE",
                AnyLogicalArrayType::ArrayOfFloat32LE => "arrayOfFloat32LE",
                AnyLogicalArrayType::ArrayOfDouble64LE => "arrayOfDouble64LE",
                AnyLogicalArrayType::ArrayOfInt16BE => "arrayOfInt16BE",
                AnyLogicalArrayType::ArrayOfInt32BE => "arrayOfInt32BE",
                AnyLogicalArrayType::ArrayOfInt64BE => "arrayOfInt64BE",
                AnyLogicalArrayType::ArrayOfUInt16BE => "arrayOfUInt16BE",
                AnyLogicalArrayType::ArrayOfUInt32BE => "arrayOfUInt32BE",
                AnyLogicalArrayType::ArrayOfUInt64BE => "arrayOfUInt64BE",
                AnyLogicalArrayType::ArrayOfFloat32BE => "arrayOfFloat32BE",
                AnyLogicalArrayType::ArrayOfDouble64BE => "arrayOfDouble64BE",
                AnyLogicalArrayType::ArrayOfString => "arrayOfString",
                AnyLogicalArrayType::ArrayOfCustom => "arrayOfCustom",
            }
        )
    }
}

impl AnyLogicalArrayType {
    pub fn iter() -> Iter<'static, AnyLogicalArrayType> {
        static VEC_ENUM: [AnyLogicalArrayType; 21] = [
            AnyLogicalArrayType::ArrayOfBoolean,
            AnyLogicalArrayType::ArrayOfInt8,
            AnyLogicalArrayType::ArrayOfUInt8,
            AnyLogicalArrayType::ArrayOfInt16LE,
            AnyLogicalArrayType::ArrayOfInt32LE,
            AnyLogicalArrayType::ArrayOfInt64LE,
            AnyLogicalArrayType::ArrayOfUInt16LE,
            AnyLogicalArrayType::ArrayOfUInt32LE,
            AnyLogicalArrayType::ArrayOfUInt64LE,
            AnyLogicalArrayType::ArrayOfFloat32LE,
            AnyLogicalArrayType::ArrayOfDouble64LE,
            AnyLogicalArrayType::ArrayOfInt16BE,
            AnyLogicalArrayType::ArrayOfInt32BE,
            AnyLogicalArrayType::ArrayOfInt64BE,
            AnyLogicalArrayType::ArrayOfUInt16BE,
            AnyLogicalArrayType::ArrayOfUInt32BE,
            AnyLogicalArrayType::ArrayOfUInt64BE,
            AnyLogicalArrayType::ArrayOfFloat32BE,
            AnyLogicalArrayType::ArrayOfDouble64BE,
            AnyLogicalArrayType::ArrayOfString,
            AnyLogicalArrayType::ArrayOfCustom,
        ];
        VEC_ENUM.iter()
    }
}
