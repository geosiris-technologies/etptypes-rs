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
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChannelDataKind {
    /* None */
    DateTime,
    ElapsedTime,
    MeasuredDepth,
    PassIndexedDepth,
    TrueVerticalDepth,
    TypeBoolean,
    TypeInt,
    TypeLong,
    TypeFloat,
    TypeDouble,
    TypeString,
    TypeBytes,
}

impl fmt::Display for ChannelDataKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ChannelDataKind::DateTime => "DateTime",
                ChannelDataKind::ElapsedTime => "ElapsedTime",
                ChannelDataKind::MeasuredDepth => "MeasuredDepth",
                ChannelDataKind::PassIndexedDepth => "PassIndexedDepth",
                ChannelDataKind::TrueVerticalDepth => "TrueVerticalDepth",
                ChannelDataKind::TypeBoolean => "typeBoolean",
                ChannelDataKind::TypeInt => "typeInt",
                ChannelDataKind::TypeLong => "typeLong",
                ChannelDataKind::TypeFloat => "typeFloat",
                ChannelDataKind::TypeDouble => "typeDouble",
                ChannelDataKind::TypeString => "typeString",
                ChannelDataKind::TypeBytes => "typeBytes",
            }
        )
    }
}

impl FromStr for ChannelDataKind {
    type Err = ();
    fn from_str(input: &str) -> Result<ChannelDataKind, Self::Err> {
        match input {
            "DateTime" => Ok(ChannelDataKind::DateTime),
            "ElapsedTime" => Ok(ChannelDataKind::ElapsedTime),
            "MeasuredDepth" => Ok(ChannelDataKind::MeasuredDepth),
            "PassIndexedDepth" => Ok(ChannelDataKind::PassIndexedDepth),
            "TrueVerticalDepth" => Ok(ChannelDataKind::TrueVerticalDepth),
            "typeBoolean" => Ok(ChannelDataKind::TypeBoolean),
            "typeInt" => Ok(ChannelDataKind::TypeInt),
            "typeLong" => Ok(ChannelDataKind::TypeLong),
            "typeFloat" => Ok(ChannelDataKind::TypeFloat),
            "typeDouble" => Ok(ChannelDataKind::TypeDouble),
            "typeString" => Ok(ChannelDataKind::TypeString),
            "typeBytes" => Ok(ChannelDataKind::TypeBytes),
            _ => Err(()),
        }
    }
}

impl ChannelDataKind {
    pub fn iter() -> Iter<'static, ChannelDataKind> {
        static VEC_ENUM: [ChannelDataKind; 12] = [
            ChannelDataKind::DateTime,
            ChannelDataKind::ElapsedTime,
            ChannelDataKind::MeasuredDepth,
            ChannelDataKind::PassIndexedDepth,
            ChannelDataKind::TrueVerticalDepth,
            ChannelDataKind::TypeBoolean,
            ChannelDataKind::TypeInt,
            ChannelDataKind::TypeLong,
            ChannelDataKind::TypeFloat,
            ChannelDataKind::TypeDouble,
            ChannelDataKind::TypeString,
            ChannelDataKind::TypeBytes,
        ];
        VEC_ENUM.iter()
    }
}
