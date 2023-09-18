// SPDX-FileCopyrightText: 2023 Geosiris
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use crate::helpers::*;
use apache_avro::{Error, Schema};
use bytes;
use derivative::Derivative;
use std::collections::HashMap;
use std::time::SystemTime;

use crate::helpers::ETPMetadata;
use crate::helpers::Schemable;
use crate::protocols::ProtocolMessage;
use apache_avro::{from_avro_datum, from_value, AvroResult};
use std::io::Read;
#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize, Derivative)]
#[serde(rename_all = "PascalCase")]
pub struct UnsubscribeChannels {
    #[serde(rename = "channelIds")]
    pub channel_ids: HashMap<String, i64>,
}

fn unsubscribechannels_avro_schema() -> Option<Schema> {
    match Schema::parse_str(AVRO_SCHEMA) {
        Ok(result) => Some(result),
        Err(e) => {
            panic!("{:?}", e);
        }
    }
}

impl Schemable for UnsubscribeChannels {
    fn avro_schema(&self) -> Option<Schema> {
        unsubscribechannels_avro_schema()
    }
    fn avro_schema_str(&self) -> &'static str {
        AVRO_SCHEMA
    }
}

impl AvroSerializable for UnsubscribeChannels {}

impl AvroDeserializable for UnsubscribeChannels {
    fn avro_deserialize<R: Read>(input: &mut R) -> AvroResult<UnsubscribeChannels> {
        let record =
            from_avro_datum(&unsubscribechannels_avro_schema().unwrap(), input, None).unwrap();
        from_value::<UnsubscribeChannels>(&record)
    }
}

impl ETPMetadata for UnsubscribeChannels {
    fn protocol(&self) -> i32 {
        21
    }
    fn message_type(&self) -> i32 {
        7
    }
    fn sender_role(&self) -> Vec<Role> {
        vec![Role::Customer]
    }
    fn protocol_roles(&self) -> Vec<Role> {
        vec![Role::Store, Role::Customer]
    }
    fn multipart_flag(&self) -> bool {
        false
    }
}

impl UnsubscribeChannels {
    pub fn as_protocol_message(&self) -> ProtocolMessage {
        ProtocolMessage::ChannelSubscribe_UnsubscribeChannels(self.clone())
    }
}

impl Default for UnsubscribeChannels {
    /* Protocol 21, MessageType : 7 */
    fn default() -> UnsubscribeChannels {
        UnsubscribeChannels {
            channel_ids: HashMap::new(),
        }
    }
}

pub static AVRO_SCHEMA: &'static str = r#"{
    "type": "record",
    "namespace": "Energistics.Etp.v12.Protocol.ChannelSubscribe",
    "name": "UnsubscribeChannels",
    "protocol": "21",
    "messageType": "7",
    "senderRole": "customer",
    "protocolRoles": "store,customer",
    "multipartFlag": false,
    "fields": [
        {
            "name": "channelIds",
            "type": {
                "type": "map",
                "values": "long"
            }
        }
    ],
    "fullName": "Energistics.Etp.v12.Protocol.ChannelSubscribe.UnsubscribeChannels",
    "depends": []
}"#;
