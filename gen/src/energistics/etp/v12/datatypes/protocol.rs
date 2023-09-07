// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::*;
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::SystemTime;

use std::fmt;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Protocol {
    core = 0,
    channel_streaming = 1,
    channel_data_frame = 2,
    discovery = 3,
    store = 4,
    store_notification = 5,
    growing_object = 6,
    growing_object_notification = 7,
    deprecated_8 = 8,
    data_array = 9,
    reserved_10 = 10,
    reserved_11 = 11,
    reserved_12 = 12,
    discovery_query = 13,
    store_query = 14,
    reserved_15 = 15,
    growing_object_query = 16,
    reserved_17 = 17,
    transaction = 18,
    reserved_19 = 19,
    reserved_20 = 20,
    channel_subscribe = 21,
    channel_data_load = 22,
    reserved_23 = 23,
    dataspace = 24,
    supported_types = 25,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Protocol::core => "Core",
                Protocol::channel_streaming => "ChannelStreaming",
                Protocol::channel_data_frame => "ChannelDataFrame",
                Protocol::discovery => "Discovery",
                Protocol::store => "Store",
                Protocol::store_notification => "StoreNotification",
                Protocol::growing_object => "GrowingObject",
                Protocol::growing_object_notification => "GrowingObjectNotification",
                Protocol::deprecated_8 => "DEPRECATED_8",
                Protocol::data_array => "DataArray",
                Protocol::reserved_10 => "RESERVED_10",
                Protocol::reserved_11 => "RESERVED_11",
                Protocol::reserved_12 => "RESERVED_12",
                Protocol::discovery_query => "DiscoveryQuery",
                Protocol::store_query => "StoreQuery",
                Protocol::reserved_15 => "RESERVED_15",
                Protocol::growing_object_query => "GrowingObjectQuery",
                Protocol::reserved_17 => "RESERVED_17",
                Protocol::transaction => "Transaction",
                Protocol::reserved_19 => "RESERVED_19",
                Protocol::reserved_20 => "RESERVED_20",
                Protocol::channel_subscribe => "ChannelSubscribe",
                Protocol::channel_data_load => "ChannelDataLoad",
                Protocol::reserved_23 => "RESERVED_23",
                Protocol::dataspace => "Dataspace",
                Protocol::supported_types => "SupportedTypes",
            }
        )
    }
}
