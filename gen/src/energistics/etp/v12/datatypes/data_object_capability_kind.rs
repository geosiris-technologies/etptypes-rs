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
pub enum DataObjectCapabilityKind {
    active_timeout_period,
    max_contained_data_object_count,
    max_data_object_size,
    orphaned_children_pruned_on_delete,
    supports_get,
    supports_put,
    supports_delete,
    max_secondary_index_count,
}

impl fmt::Display for DataObjectCapabilityKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataObjectCapabilityKind::active_timeout_period => "ActiveTimeoutPeriod",
                DataObjectCapabilityKind::max_contained_data_object_count =>
                    "MaxContainedDataObjectCount",
                DataObjectCapabilityKind::max_data_object_size => "MaxDataObjectSize",
                DataObjectCapabilityKind::orphaned_children_pruned_on_delete =>
                    "OrphanedChildrenPrunedOnDelete",
                DataObjectCapabilityKind::supports_get => "SupportsGet",
                DataObjectCapabilityKind::supports_put => "SupportsPut",
                DataObjectCapabilityKind::supports_delete => "SupportsDelete",
                DataObjectCapabilityKind::max_secondary_index_count => "MaxSecondaryIndexCount",
            }
        )
    }
}
