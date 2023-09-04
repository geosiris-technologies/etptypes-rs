#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;




use std::fmt;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ObjectChangeKind{
    insert,
    update,
    authorized,
    joined,
    unjoined,
    joined_subscription,
    unjoined_subscription,
}

impl fmt::Display for ObjectChangeKind{
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ObjectChangeKind::insert => "insert",
                ObjectChangeKind::update => "update",
                ObjectChangeKind::authorized => "authorized",
                ObjectChangeKind::joined => "joined",
                ObjectChangeKind::unjoined => "unjoined",
                ObjectChangeKind::joined_subscription => "joinedSubscription",
                ObjectChangeKind::unjoined_subscription => "unjoinedSubscription",
            }
        )
    }
}


