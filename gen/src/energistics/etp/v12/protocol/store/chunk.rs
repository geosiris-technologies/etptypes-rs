// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::{SystemTime};
use crate::helpers::*;
use avro_rs::{Schema, Error};
use crate::helpers::EtpMessageBody;

use crate::energistics::etp::v12::datatypes::uuid::Uuid;

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "camelCase")]
pub struct Chunk{

	#[serde(rename = "blobId")]
    pub blob_id:Uuid,


	#[serde(rename = "data")]
    pub data:Vec<u8>,


	#[serde(rename = "final")]
    pub final_ :bool,

}

pub static AVRO_SCHEMA: &'static str = r#"{"type": "record", "namespace": "Energistics.Etp.v12.Protocol.Store", "name": "Chunk", "protocol": "4", "messageType": "8", "senderRole": "store,customer", "protocolRoles": "store,customer", "multipartFlag": true, "fields": [{"name": "blobId", "type": {"type": "fixed", "namespace": "Energistics.Etp.v12.Datatypes", "name": "Uuid", "size": 16, "fullName": "Energistics.Etp.v12.Datatypes.Uuid", "depends": []}}, {"name": "data", "type": "bytes"}, {"name": "final", "type": "boolean"}], "fullName": "Energistics.Etp.v12.Protocol.Store.Chunk", "depends": ["Energistics.Etp.v12.Datatypes.Uuid"]}"#;

impl EtpMessageBody for Chunk{
    fn avro_schema() -> Option<Schema>{
        match Schema::parse_str(AVRO_SCHEMA) {
            Ok(result) => Some(result),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
    fn protocol(&self) ->i32{
        4
    }
    fn message_type(&self) ->i32{
        8
    }
    fn sender_role(&self) ->String{
        "store,customer".to_string()
    }
    fn protocol_roles(&self) ->String{
        "store,customer".to_string()
    }
    fn multipart_flag(&self) ->bool{
        true
    }
}

impl Chunk{
    /* Protocol 4, MessageType : 8 */
    pub fn default_with_params(_data: Vec<u8>, )
    -> Chunk {
        Chunk {
            blob_id : Uuid::new_v4(),
            data : _data,
            final_  : true,
        }
    }
}

