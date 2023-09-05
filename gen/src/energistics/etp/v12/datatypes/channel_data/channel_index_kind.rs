// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::{SystemTime};
use crate::helpers::*;




use std::fmt;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChannelIndexKind{
    date_time,
    elapsed_time,
    measured_depth,
    true_vertical_depth,
    pass_indexed_depth,
    pressure,
    temperature,
    scalar,
}

impl fmt::Display for ChannelIndexKind{
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ChannelIndexKind::date_time => "DateTime",
                ChannelIndexKind::elapsed_time => "ElapsedTime",
                ChannelIndexKind::measured_depth => "MeasuredDepth",
                ChannelIndexKind::true_vertical_depth => "TrueVerticalDepth",
                ChannelIndexKind::pass_indexed_depth => "PassIndexedDepth",
                ChannelIndexKind::pressure => "Pressure",
                ChannelIndexKind::temperature => "Temperature",
                ChannelIndexKind::scalar => "Scalar",
            }
        )
    }
}


