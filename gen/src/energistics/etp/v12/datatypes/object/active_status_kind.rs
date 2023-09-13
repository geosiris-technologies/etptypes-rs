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
pub enum ActiveStatusKind {
    /* None */
    Active,
    Inactive,
}

impl fmt::Display for ActiveStatusKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ActiveStatusKind::Active => "Active",
                ActiveStatusKind::Inactive => "Inactive",
            }
        )
    }
}

impl ActiveStatusKind {
    pub fn iter() -> Iter<'static, ActiveStatusKind> {
        static VEC_ENUM: [ActiveStatusKind; 2] =
            [ActiveStatusKind::Active, ActiveStatusKind::Inactive];
        VEC_ENUM.iter()
    }
}
