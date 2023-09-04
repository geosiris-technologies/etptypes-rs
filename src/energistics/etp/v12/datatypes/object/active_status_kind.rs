#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;




use std::fmt;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ActiveStatusKind{
    active,
    inactive,
}

impl fmt::Display for ActiveStatusKind{
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ActiveStatusKind::active => "Active",
                ActiveStatusKind::inactive => "Inactive",
            }
        )
    }
}


