// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;




use std::fmt;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ContextScopeKind{
    self_ ,
    sources,
    targets,
    sources_or_self,
    targets_or_self,
}

impl fmt::Display for ContextScopeKind{
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ContextScopeKind::self_ => "self",
                ContextScopeKind::sources => "sources",
                ContextScopeKind::targets => "targets",
                ContextScopeKind::sources_or_self => "sourcesOrSelf",
                ContextScopeKind::targets_or_self => "targetsOrSelf",
            }
        )
    }
}


