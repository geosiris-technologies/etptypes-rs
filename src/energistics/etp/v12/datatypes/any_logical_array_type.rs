#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;




use std::fmt;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AnyLogicalArrayType{
    array_of_boolean,
    array_of_int8,
    array_of_uint8,
    array_of_int16_le,
    array_of_int32_le,
    array_of_int64_le,
    array_of_uint16_le,
    array_of_uint32_le,
    array_of_uint64_le,
    array_of_float32_le,
    array_of_double64_le,
    array_of_int16_be,
    array_of_int32_be,
    array_of_int64_be,
    array_of_uint16_be,
    array_of_uint32_be,
    array_of_uint64_be,
    array_of_float32_be,
    array_of_double64_be,
    array_of_string,
    array_of_custom,
}

impl fmt::Display for AnyLogicalArrayType{
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AnyLogicalArrayType::array_of_boolean => "arrayOfBoolean",
                AnyLogicalArrayType::array_of_int8 => "arrayOfInt8",
                AnyLogicalArrayType::array_of_uint8 => "arrayOfUInt8",
                AnyLogicalArrayType::array_of_int16_le => "arrayOfInt16LE",
                AnyLogicalArrayType::array_of_int32_le => "arrayOfInt32LE",
                AnyLogicalArrayType::array_of_int64_le => "arrayOfInt64LE",
                AnyLogicalArrayType::array_of_uint16_le => "arrayOfUInt16LE",
                AnyLogicalArrayType::array_of_uint32_le => "arrayOfUInt32LE",
                AnyLogicalArrayType::array_of_uint64_le => "arrayOfUInt64LE",
                AnyLogicalArrayType::array_of_float32_le => "arrayOfFloat32LE",
                AnyLogicalArrayType::array_of_double64_le => "arrayOfDouble64LE",
                AnyLogicalArrayType::array_of_int16_be => "arrayOfInt16BE",
                AnyLogicalArrayType::array_of_int32_be => "arrayOfInt32BE",
                AnyLogicalArrayType::array_of_int64_be => "arrayOfInt64BE",
                AnyLogicalArrayType::array_of_uint16_be => "arrayOfUInt16BE",
                AnyLogicalArrayType::array_of_uint32_be => "arrayOfUInt32BE",
                AnyLogicalArrayType::array_of_uint64_be => "arrayOfUInt64BE",
                AnyLogicalArrayType::array_of_float32_be => "arrayOfFloat32BE",
                AnyLogicalArrayType::array_of_double64_be => "arrayOfDouble64BE",
                AnyLogicalArrayType::array_of_string => "arrayOfString",
                AnyLogicalArrayType::array_of_custom => "arrayOfCustom",
            }
        )
    }
}


