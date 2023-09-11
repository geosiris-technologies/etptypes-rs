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
pub enum DataObjectCapabilityKind {
    /* None */
    ActiveTimeoutPeriod,
    MaxContainedDataObjectCount,
    MaxDataObjectSize,
    OrphanedChildrenPrunedOnDelete,
    SupportsGet,
    SupportsPut,
    SupportsDelete,
    MaxSecondaryIndexCount,
}

impl fmt::Display for DataObjectCapabilityKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataObjectCapabilityKind::ActiveTimeoutPeriod => "ActiveTimeoutPeriod",
                DataObjectCapabilityKind::MaxContainedDataObjectCount =>
                    "MaxContainedDataObjectCount",
                DataObjectCapabilityKind::MaxDataObjectSize => "MaxDataObjectSize",
                DataObjectCapabilityKind::OrphanedChildrenPrunedOnDelete =>
                    "OrphanedChildrenPrunedOnDelete",
                DataObjectCapabilityKind::SupportsGet => "SupportsGet",
                DataObjectCapabilityKind::SupportsPut => "SupportsPut",
                DataObjectCapabilityKind::SupportsDelete => "SupportsDelete",
                DataObjectCapabilityKind::MaxSecondaryIndexCount => "MaxSecondaryIndexCount",
            }
        )
    }
}

impl DataObjectCapabilityKind {
    pub fn iter() -> Iter<'static, DataObjectCapabilityKind> {
        static VEC_ENUM: [DataObjectCapabilityKind; 8] = [
            DataObjectCapabilityKind::ActiveTimeoutPeriod,
            DataObjectCapabilityKind::MaxContainedDataObjectCount,
            DataObjectCapabilityKind::MaxDataObjectSize,
            DataObjectCapabilityKind::OrphanedChildrenPrunedOnDelete,
            DataObjectCapabilityKind::SupportsGet,
            DataObjectCapabilityKind::SupportsPut,
            DataObjectCapabilityKind::SupportsDelete,
            DataObjectCapabilityKind::MaxSecondaryIndexCount,
        ];
        VEC_ENUM.iter()
    }
}
