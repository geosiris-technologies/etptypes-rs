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
pub enum RelationshipKind {
    /* None */
    Primary,
    Secondary,
    Both,
}

impl fmt::Display for RelationshipKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RelationshipKind::Primary => "Primary",
                RelationshipKind::Secondary => "Secondary",
                RelationshipKind::Both => "Both",
            }
        )
    }
}

impl RelationshipKind {
    pub fn iter() -> Iter<'static, RelationshipKind> {
        static VEC_ENUM: [RelationshipKind; 3] = [
            RelationshipKind::Primary,
            RelationshipKind::Secondary,
            RelationshipKind::Both,
        ];
        VEC_ENUM.iter()
    }
}
