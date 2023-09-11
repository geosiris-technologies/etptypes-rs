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
