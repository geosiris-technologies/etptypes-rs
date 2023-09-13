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
pub enum PassDirection {
    /* None */
    Up,
    HoldingSteady,
    Down,
}

impl fmt::Display for PassDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PassDirection::Up => "Up",
                PassDirection::HoldingSteady => "HoldingSteady",
                PassDirection::Down => "Down",
            }
        )
    }
}

impl PassDirection {
    pub fn iter() -> Iter<'static, PassDirection> {
        static VEC_ENUM: [PassDirection; 3] = [
            PassDirection::Up,
            PassDirection::HoldingSteady,
            PassDirection::Down,
        ];
        VEC_ENUM.iter()
    }
}
