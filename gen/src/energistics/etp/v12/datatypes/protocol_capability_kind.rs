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
pub enum ProtocolCapabilityKind {
    /* None */
    FrameChangeDetectionPeriod,
    MaxDataArraySize,
    MaxDataObjectSize,
    MaxFrameResponseRowCount,
    MaxIndexCount,
    MaxRangeChannelCount,
    MaxRangeDataItemCount,
    MaxResponseCount,
    MaxStreamingChannelsSessionCount,
    MaxSubscriptionSessionCount,
    MaxTransactionCount,
    SupportsSecondaryIndexFiltering,
    TransactionTimeoutPeriod,
}

impl fmt::Display for ProtocolCapabilityKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProtocolCapabilityKind::FrameChangeDetectionPeriod => "FrameChangeDetectionPeriod",
                ProtocolCapabilityKind::MaxDataArraySize => "MaxDataArraySize",
                ProtocolCapabilityKind::MaxDataObjectSize => "MaxDataObjectSize",
                ProtocolCapabilityKind::MaxFrameResponseRowCount => "MaxFrameResponseRowCount",
                ProtocolCapabilityKind::MaxIndexCount => "MaxIndexCount",
                ProtocolCapabilityKind::MaxRangeChannelCount => "MaxRangeChannelCount",
                ProtocolCapabilityKind::MaxRangeDataItemCount => "MaxRangeDataItemCount",
                ProtocolCapabilityKind::MaxResponseCount => "MaxResponseCount",
                ProtocolCapabilityKind::MaxStreamingChannelsSessionCount =>
                    "MaxStreamingChannelsSessionCount",
                ProtocolCapabilityKind::MaxSubscriptionSessionCount =>
                    "MaxSubscriptionSessionCount",
                ProtocolCapabilityKind::MaxTransactionCount => "MaxTransactionCount",
                ProtocolCapabilityKind::SupportsSecondaryIndexFiltering =>
                    "SupportsSecondaryIndexFiltering",
                ProtocolCapabilityKind::TransactionTimeoutPeriod => "TransactionTimeoutPeriod",
            }
        )
    }
}

impl ProtocolCapabilityKind {
    pub fn iter() -> Iter<'static, ProtocolCapabilityKind> {
        static VEC_ENUM: [ProtocolCapabilityKind; 13] = [
            ProtocolCapabilityKind::FrameChangeDetectionPeriod,
            ProtocolCapabilityKind::MaxDataArraySize,
            ProtocolCapabilityKind::MaxDataObjectSize,
            ProtocolCapabilityKind::MaxFrameResponseRowCount,
            ProtocolCapabilityKind::MaxIndexCount,
            ProtocolCapabilityKind::MaxRangeChannelCount,
            ProtocolCapabilityKind::MaxRangeDataItemCount,
            ProtocolCapabilityKind::MaxResponseCount,
            ProtocolCapabilityKind::MaxStreamingChannelsSessionCount,
            ProtocolCapabilityKind::MaxSubscriptionSessionCount,
            ProtocolCapabilityKind::MaxTransactionCount,
            ProtocolCapabilityKind::SupportsSecondaryIndexFiltering,
            ProtocolCapabilityKind::TransactionTimeoutPeriod,
        ];
        VEC_ENUM.iter()
    }
}
