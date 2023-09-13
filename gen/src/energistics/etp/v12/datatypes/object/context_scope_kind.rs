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
pub enum ContextScopeKind {
    /* None */
    Self_,
    Sources,
    Targets,
    SourcesOrSelf,
    TargetsOrSelf,
}

impl fmt::Display for ContextScopeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ContextScopeKind::Self_ => "self",
                ContextScopeKind::Sources => "sources",
                ContextScopeKind::Targets => "targets",
                ContextScopeKind::SourcesOrSelf => "sourcesOrSelf",
                ContextScopeKind::TargetsOrSelf => "targetsOrSelf",
            }
        )
    }
}

impl ContextScopeKind {
    pub fn iter() -> Iter<'static, ContextScopeKind> {
        static VEC_ENUM: [ContextScopeKind; 5] = [
            ContextScopeKind::Self_,
            ContextScopeKind::Sources,
            ContextScopeKind::Targets,
            ContextScopeKind::SourcesOrSelf,
            ContextScopeKind::TargetsOrSelf,
        ];
        VEC_ENUM.iter()
    }
}
