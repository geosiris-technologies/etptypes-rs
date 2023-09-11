// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::*;
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::fmt;
use std::slice::Iter;
use std::time::SystemTime;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum ObjectChangeKind {
    /* None */
    Insert,
    Update,
    Authorized,
    Joined,
    Unjoined,
    JoinedSubscription,
    UnjoinedSubscription,
}

impl fmt::Display for ObjectChangeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ObjectChangeKind::Insert => "insert",
                ObjectChangeKind::Update => "update",
                ObjectChangeKind::Authorized => "authorized",
                ObjectChangeKind::Joined => "joined",
                ObjectChangeKind::Unjoined => "unjoined",
                ObjectChangeKind::JoinedSubscription => "joinedSubscription",
                ObjectChangeKind::UnjoinedSubscription => "unjoinedSubscription",
            }
        )
    }
}

impl ObjectChangeKind {
    pub fn iter() -> Iter<'static, ObjectChangeKind> {
        static VEC_ENUM: [ObjectChangeKind; 7] = [
            ObjectChangeKind::Insert,
            ObjectChangeKind::Update,
            ObjectChangeKind::Authorized,
            ObjectChangeKind::Joined,
            ObjectChangeKind::Unjoined,
            ObjectChangeKind::JoinedSubscription,
            ObjectChangeKind::UnjoinedSubscription,
        ];
        VEC_ENUM.iter()
    }
}
