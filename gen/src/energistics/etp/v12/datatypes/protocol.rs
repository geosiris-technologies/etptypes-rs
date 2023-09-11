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
pub enum Protocol {
    /* None */
    Core = 0,
    ChannelStreaming = 1,
    ChannelDataFrame = 2,
    Discovery = 3,
    Store = 4,
    StoreNotification = 5,
    GrowingObject = 6,
    GrowingObjectNotification = 7,
    DEPRECATED8 = 8,
    DataArray = 9,
    RESERVED10 = 10,
    RESERVED11 = 11,
    RESERVED12 = 12,
    DiscoveryQuery = 13,
    StoreQuery = 14,
    RESERVED15 = 15,
    GrowingObjectQuery = 16,
    RESERVED17 = 17,
    Transaction = 18,
    RESERVED19 = 19,
    RESERVED20 = 20,
    ChannelSubscribe = 21,
    ChannelDataLoad = 22,
    RESERVED23 = 23,
    Dataspace = 24,
    SupportedTypes = 25,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Protocol::Core => "Core",
                Protocol::ChannelStreaming => "ChannelStreaming",
                Protocol::ChannelDataFrame => "ChannelDataFrame",
                Protocol::Discovery => "Discovery",
                Protocol::Store => "Store",
                Protocol::StoreNotification => "StoreNotification",
                Protocol::GrowingObject => "GrowingObject",
                Protocol::GrowingObjectNotification => "GrowingObjectNotification",
                Protocol::DEPRECATED8 => "DEPRECATED_8",
                Protocol::DataArray => "DataArray",
                Protocol::RESERVED10 => "RESERVED_10",
                Protocol::RESERVED11 => "RESERVED_11",
                Protocol::RESERVED12 => "RESERVED_12",
                Protocol::DiscoveryQuery => "DiscoveryQuery",
                Protocol::StoreQuery => "StoreQuery",
                Protocol::RESERVED15 => "RESERVED_15",
                Protocol::GrowingObjectQuery => "GrowingObjectQuery",
                Protocol::RESERVED17 => "RESERVED_17",
                Protocol::Transaction => "Transaction",
                Protocol::RESERVED19 => "RESERVED_19",
                Protocol::RESERVED20 => "RESERVED_20",
                Protocol::ChannelSubscribe => "ChannelSubscribe",
                Protocol::ChannelDataLoad => "ChannelDataLoad",
                Protocol::RESERVED23 => "RESERVED_23",
                Protocol::Dataspace => "Dataspace",
                Protocol::SupportedTypes => "SupportedTypes",
            }
        )
    }
}

impl Protocol {
    pub fn iter() -> Iter<'static, Protocol> {
        static VEC_ENUM: [Protocol; 26] = [
            Protocol::Core,
            Protocol::ChannelStreaming,
            Protocol::ChannelDataFrame,
            Protocol::Discovery,
            Protocol::Store,
            Protocol::StoreNotification,
            Protocol::GrowingObject,
            Protocol::GrowingObjectNotification,
            Protocol::DEPRECATED8,
            Protocol::DataArray,
            Protocol::RESERVED10,
            Protocol::RESERVED11,
            Protocol::RESERVED12,
            Protocol::DiscoveryQuery,
            Protocol::StoreQuery,
            Protocol::RESERVED15,
            Protocol::GrowingObjectQuery,
            Protocol::RESERVED17,
            Protocol::Transaction,
            Protocol::RESERVED19,
            Protocol::RESERVED20,
            Protocol::ChannelSubscribe,
            Protocol::ChannelDataLoad,
            Protocol::RESERVED23,
            Protocol::Dataspace,
            Protocol::SupportedTypes,
        ];
        VEC_ENUM.iter()
    }
}
