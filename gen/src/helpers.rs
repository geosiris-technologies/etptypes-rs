// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
use avro_rs::{Schema};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::energistics::etp::v12::datatypes::version::Version;

pub fn time_to_etp(time: SystemTime) -> i64 {
    let since_the_epoch = time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_millis();
    return in_ms.try_into().unwrap();
}

pub enum Role {
    All,
    Client,
    Server,
    Customer,
    Store,
    Producer,
    Consumer,
}

impl Role {
    pub fn to_string(self) -> String {
        match self {
            Self::All => "*".to_string(),
            Self::Client => "client".to_string(),
            Self::Server => "server".to_string(),
            Self::Customer => "customer".to_string(),
            Self::Store => "store".to_string(),
            Self::Producer => "producer".to_string(),
            Self::Consumer => "consumer".to_string(),
        }
    }
}

pub const ETP12VERSION: Version = Version {
    major: 1,
    minor: 2,
    revision: 0,
    patch: 0,
};

pub const ETP11VERSION: Version = Version {
    major: 1,
    minor: 1,
    revision: 0,
    patch: 0,
};

pub trait ETPMetadata {
    fn avro_schema() -> Option<Schema>;

    fn protocol(&self) -> i32;
    fn message_type(&self) -> i32;
    fn sender_role(&self) -> Vec<Role>;
    fn protocol_roles(&self) -> Vec<Role>;
    fn multipart_flag(&self) -> bool;
}
