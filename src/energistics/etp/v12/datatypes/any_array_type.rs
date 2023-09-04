#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;




use std::fmt;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AnyArrayType{
    array_of_boolean,
    array_of_int,
    array_of_long,
    array_of_float,
    array_of_double,
    array_of_string,
    bytes,
}

impl fmt::Display for AnyArrayType{
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AnyArrayType::array_of_boolean => "arrayOfBoolean",
                AnyArrayType::array_of_int => "arrayOfInt",
                AnyArrayType::array_of_long => "arrayOfLong",
                AnyArrayType::array_of_float => "arrayOfFloat",
                AnyArrayType::array_of_double => "arrayOfDouble",
                AnyArrayType::array_of_string => "arrayOfString",
                AnyArrayType::bytes => "bytes",
            }
        )
    }
}


