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
pub enum ChannelIndexKind {
    /* None */
    DateTime,
    ElapsedTime,
    MeasuredDepth,
    TrueVerticalDepth,
    PassIndexedDepth,
    Pressure,
    Temperature,
    Scalar,
}

impl fmt::Display for ChannelIndexKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ChannelIndexKind::DateTime => "DateTime",
                ChannelIndexKind::ElapsedTime => "ElapsedTime",
                ChannelIndexKind::MeasuredDepth => "MeasuredDepth",
                ChannelIndexKind::TrueVerticalDepth => "TrueVerticalDepth",
                ChannelIndexKind::PassIndexedDepth => "PassIndexedDepth",
                ChannelIndexKind::Pressure => "Pressure",
                ChannelIndexKind::Temperature => "Temperature",
                ChannelIndexKind::Scalar => "Scalar",
            }
        )
    }
}

impl ChannelIndexKind {
    pub fn iter() -> Iter<'static, ChannelIndexKind> {
        static VEC_ENUM: [ChannelIndexKind; 8] = [
            ChannelIndexKind::DateTime,
            ChannelIndexKind::ElapsedTime,
            ChannelIndexKind::MeasuredDepth,
            ChannelIndexKind::TrueVerticalDepth,
            ChannelIndexKind::PassIndexedDepth,
            ChannelIndexKind::Pressure,
            ChannelIndexKind::Temperature,
            ChannelIndexKind::Scalar,
        ];
        VEC_ENUM.iter()
    }
}
