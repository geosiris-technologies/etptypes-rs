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
pub enum EndpointCapabilityKind{
    active_timeout_period,
    authorization_details,
    change_propagation_period,
    change_retention_period,
    max_concurrent_multipart,
    max_data_object_size,
    max_part_size,
    max_session_client_count,
    max_session_global_count,
    max_web_socket_frame_payload_size,
    max_web_socket_message_payload_size,
    multipart_message_timeout_period,
    response_timeout_period,
    request_session_timeout_period,
    session_establishment_timeout_period,
    supports_alternate_request_uris,
    supports_message_header_extensions,
}

impl fmt::Display for EndpointCapabilityKind{
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EndpointCapabilityKind::active_timeout_period => "ActiveTimeoutPeriod",
                EndpointCapabilityKind::authorization_details => "AuthorizationDetails",
                EndpointCapabilityKind::change_propagation_period => "ChangePropagationPeriod",
                EndpointCapabilityKind::change_retention_period => "ChangeRetentionPeriod",
                EndpointCapabilityKind::max_concurrent_multipart => "MaxConcurrentMultipart",
                EndpointCapabilityKind::max_data_object_size => "MaxDataObjectSize",
                EndpointCapabilityKind::max_part_size => "MaxPartSize",
                EndpointCapabilityKind::max_session_client_count => "MaxSessionClientCount",
                EndpointCapabilityKind::max_session_global_count => "MaxSessionGlobalCount",
                EndpointCapabilityKind::max_web_socket_frame_payload_size => "MaxWebSocketFramePayloadSize",
                EndpointCapabilityKind::max_web_socket_message_payload_size => "MaxWebSocketMessagePayloadSize",
                EndpointCapabilityKind::multipart_message_timeout_period => "MultipartMessageTimeoutPeriod",
                EndpointCapabilityKind::response_timeout_period => "ResponseTimeoutPeriod",
                EndpointCapabilityKind::request_session_timeout_period => "RequestSessionTimeoutPeriod",
                EndpointCapabilityKind::session_establishment_timeout_period => "SessionEstablishmentTimeoutPeriod",
                EndpointCapabilityKind::supports_alternate_request_uris => "SupportsAlternateRequestUris",
                EndpointCapabilityKind::supports_message_header_extensions => "SupportsMessageHeaderExtensions",
            }
        )
    }
}


