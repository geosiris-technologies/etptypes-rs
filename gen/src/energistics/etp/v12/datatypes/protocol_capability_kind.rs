// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::{SystemTime};
use crate::helpers::*;




use std::fmt;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ProtocolCapabilityKind{
    frame_change_detection_period,
    max_data_array_size,
    max_data_object_size,
    max_frame_response_row_count,
    max_index_count,
    max_range_channel_count,
    max_range_data_item_count,
    max_response_count,
    max_streaming_channels_session_count,
    max_subscription_session_count,
    max_transaction_count,
    supports_secondary_index_filtering,
    transaction_timeout_period,
}

impl fmt::Display for ProtocolCapabilityKind{
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProtocolCapabilityKind::frame_change_detection_period => "FrameChangeDetectionPeriod",
                ProtocolCapabilityKind::max_data_array_size => "MaxDataArraySize",
                ProtocolCapabilityKind::max_data_object_size => "MaxDataObjectSize",
                ProtocolCapabilityKind::max_frame_response_row_count => "MaxFrameResponseRowCount",
                ProtocolCapabilityKind::max_index_count => "MaxIndexCount",
                ProtocolCapabilityKind::max_range_channel_count => "MaxRangeChannelCount",
                ProtocolCapabilityKind::max_range_data_item_count => "MaxRangeDataItemCount",
                ProtocolCapabilityKind::max_response_count => "MaxResponseCount",
                ProtocolCapabilityKind::max_streaming_channels_session_count => "MaxStreamingChannelsSessionCount",
                ProtocolCapabilityKind::max_subscription_session_count => "MaxSubscriptionSessionCount",
                ProtocolCapabilityKind::max_transaction_count => "MaxTransactionCount",
                ProtocolCapabilityKind::supports_secondary_index_filtering => "SupportsSecondaryIndexFiltering",
                ProtocolCapabilityKind::transaction_timeout_period => "TransactionTimeoutPeriod",
            }
        )
    }
}


