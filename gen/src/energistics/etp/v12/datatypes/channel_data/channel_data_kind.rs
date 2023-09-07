// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::*;
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::SystemTime;

use std::fmt;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChannelDataKind {
    date_time,
    elapsed_time,
    measured_depth,
    pass_indexed_depth,
    true_vertical_depth,
    type_boolean,
    type_int,
    type_long,
    type_float,
    type_double,
    type_string,
    type_bytes,
}

impl fmt::Display for ChannelDataKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ChannelDataKind::date_time => "DateTime",
                ChannelDataKind::elapsed_time => "ElapsedTime",
                ChannelDataKind::measured_depth => "MeasuredDepth",
                ChannelDataKind::pass_indexed_depth => "PassIndexedDepth",
                ChannelDataKind::true_vertical_depth => "TrueVerticalDepth",
                ChannelDataKind::type_boolean => "typeBoolean",
                ChannelDataKind::type_int => "typeInt",
                ChannelDataKind::type_long => "typeLong",
                ChannelDataKind::type_float => "typeFloat",
                ChannelDataKind::type_double => "typeDouble",
                ChannelDataKind::type_string => "typeString",
                ChannelDataKind::type_bytes => "typeBytes",
            }
        )
    }
}
