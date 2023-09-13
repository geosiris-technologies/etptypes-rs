// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT

use etptypes::energistics::etp::v12::datatypes::endpoint_capability_kind::EndpointCapabilityKind;
use std::str::FromStr;

#[test]
fn test_from_and_to() {
    for cap_kind in EndpointCapabilityKind::iter() {
        assert_eq!(
            cap_kind,
            &EndpointCapabilityKind::from_str(format!("{}", cap_kind).as_str()).unwrap()
        );
    }
}
