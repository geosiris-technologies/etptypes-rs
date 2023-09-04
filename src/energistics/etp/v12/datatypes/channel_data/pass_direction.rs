#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;




use std::fmt;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PassDirection{
    up,
    holding_steady,
    down,
}

impl fmt::Display for PassDirection{
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PassDirection::up => "Up",
                PassDirection::holding_steady => "HoldingSteady",
                PassDirection::down => "Down",
            }
        )
    }
}


