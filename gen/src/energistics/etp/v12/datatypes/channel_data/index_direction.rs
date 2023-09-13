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
pub enum IndexDirection {
    /* None */
    Increasing,
    Decreasing,
    Unordered,
}

impl fmt::Display for IndexDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IndexDirection::Increasing => "Increasing",
                IndexDirection::Decreasing => "Decreasing",
                IndexDirection::Unordered => "Unordered",
            }
        )
    }
}

impl IndexDirection {
    pub fn iter() -> Iter<'static, IndexDirection> {
        static VEC_ENUM: [IndexDirection; 3] = [
            IndexDirection::Increasing,
            IndexDirection::Decreasing,
            IndexDirection::Unordered,
        ];
        VEC_ENUM.iter()
    }
}
