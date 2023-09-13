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
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum EndpointCapabilityKind {
    /* None */
    ActiveTimeoutPeriod,
    AuthorizationDetails,
    ChangePropagationPeriod,
    ChangeRetentionPeriod,
    MaxConcurrentMultipart,
    MaxDataObjectSize,
    MaxPartSize,
    MaxSessionClientCount,
    MaxSessionGlobalCount,
    MaxWebSocketFramePayloadSize,
    MaxWebSocketMessagePayloadSize,
    MultipartMessageTimeoutPeriod,
    ResponseTimeoutPeriod,
    RequestSessionTimeoutPeriod,
    SessionEstablishmentTimeoutPeriod,
    SupportsAlternateRequestUris,
    SupportsMessageHeaderExtensions,
}

impl fmt::Display for EndpointCapabilityKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EndpointCapabilityKind::ActiveTimeoutPeriod => "ActiveTimeoutPeriod",
                EndpointCapabilityKind::AuthorizationDetails => "AuthorizationDetails",
                EndpointCapabilityKind::ChangePropagationPeriod => "ChangePropagationPeriod",
                EndpointCapabilityKind::ChangeRetentionPeriod => "ChangeRetentionPeriod",
                EndpointCapabilityKind::MaxConcurrentMultipart => "MaxConcurrentMultipart",
                EndpointCapabilityKind::MaxDataObjectSize => "MaxDataObjectSize",
                EndpointCapabilityKind::MaxPartSize => "MaxPartSize",
                EndpointCapabilityKind::MaxSessionClientCount => "MaxSessionClientCount",
                EndpointCapabilityKind::MaxSessionGlobalCount => "MaxSessionGlobalCount",
                EndpointCapabilityKind::MaxWebSocketFramePayloadSize =>
                    "MaxWebSocketFramePayloadSize",
                EndpointCapabilityKind::MaxWebSocketMessagePayloadSize =>
                    "MaxWebSocketMessagePayloadSize",
                EndpointCapabilityKind::MultipartMessageTimeoutPeriod =>
                    "MultipartMessageTimeoutPeriod",
                EndpointCapabilityKind::ResponseTimeoutPeriod => "ResponseTimeoutPeriod",
                EndpointCapabilityKind::RequestSessionTimeoutPeriod =>
                    "RequestSessionTimeoutPeriod",
                EndpointCapabilityKind::SessionEstablishmentTimeoutPeriod =>
                    "SessionEstablishmentTimeoutPeriod",
                EndpointCapabilityKind::SupportsAlternateRequestUris =>
                    "SupportsAlternateRequestUris",
                EndpointCapabilityKind::SupportsMessageHeaderExtensions =>
                    "SupportsMessageHeaderExtensions",
            }
        )
    }
}

impl FromStr for EndpointCapabilityKind {
    type Err = ();
    fn from_str(input: &str) -> Result<EndpointCapabilityKind, Self::Err> {
        match input {
            "ActiveTimeoutPeriod" => Ok(EndpointCapabilityKind::ActiveTimeoutPeriod),
            "AuthorizationDetails" => Ok(EndpointCapabilityKind::AuthorizationDetails),
            "ChangePropagationPeriod" => Ok(EndpointCapabilityKind::ChangePropagationPeriod),
            "ChangeRetentionPeriod" => Ok(EndpointCapabilityKind::ChangeRetentionPeriod),
            "MaxConcurrentMultipart" => Ok(EndpointCapabilityKind::MaxConcurrentMultipart),
            "MaxDataObjectSize" => Ok(EndpointCapabilityKind::MaxDataObjectSize),
            "MaxPartSize" => Ok(EndpointCapabilityKind::MaxPartSize),
            "MaxSessionClientCount" => Ok(EndpointCapabilityKind::MaxSessionClientCount),
            "MaxSessionGlobalCount" => Ok(EndpointCapabilityKind::MaxSessionGlobalCount),
            "MaxWebSocketFramePayloadSize" => {
                Ok(EndpointCapabilityKind::MaxWebSocketFramePayloadSize)
            }
            "MaxWebSocketMessagePayloadSize" => {
                Ok(EndpointCapabilityKind::MaxWebSocketMessagePayloadSize)
            }
            "MultipartMessageTimeoutPeriod" => {
                Ok(EndpointCapabilityKind::MultipartMessageTimeoutPeriod)
            }
            "ResponseTimeoutPeriod" => Ok(EndpointCapabilityKind::ResponseTimeoutPeriod),
            "RequestSessionTimeoutPeriod" => {
                Ok(EndpointCapabilityKind::RequestSessionTimeoutPeriod)
            }
            "SessionEstablishmentTimeoutPeriod" => {
                Ok(EndpointCapabilityKind::SessionEstablishmentTimeoutPeriod)
            }
            "SupportsAlternateRequestUris" => {
                Ok(EndpointCapabilityKind::SupportsAlternateRequestUris)
            }
            "SupportsMessageHeaderExtensions" => {
                Ok(EndpointCapabilityKind::SupportsMessageHeaderExtensions)
            }
            _ => Err(()),
        }
    }
}

impl EndpointCapabilityKind {
    pub fn iter() -> Iter<'static, EndpointCapabilityKind> {
        static VEC_ENUM: [EndpointCapabilityKind; 17] = [
            EndpointCapabilityKind::ActiveTimeoutPeriod,
            EndpointCapabilityKind::AuthorizationDetails,
            EndpointCapabilityKind::ChangePropagationPeriod,
            EndpointCapabilityKind::ChangeRetentionPeriod,
            EndpointCapabilityKind::MaxConcurrentMultipart,
            EndpointCapabilityKind::MaxDataObjectSize,
            EndpointCapabilityKind::MaxPartSize,
            EndpointCapabilityKind::MaxSessionClientCount,
            EndpointCapabilityKind::MaxSessionGlobalCount,
            EndpointCapabilityKind::MaxWebSocketFramePayloadSize,
            EndpointCapabilityKind::MaxWebSocketMessagePayloadSize,
            EndpointCapabilityKind::MultipartMessageTimeoutPeriod,
            EndpointCapabilityKind::ResponseTimeoutPeriod,
            EndpointCapabilityKind::RequestSessionTimeoutPeriod,
            EndpointCapabilityKind::SessionEstablishmentTimeoutPeriod,
            EndpointCapabilityKind::SupportsAlternateRequestUris,
            EndpointCapabilityKind::SupportsMessageHeaderExtensions,
        ];
        VEC_ENUM.iter()
    }
}
