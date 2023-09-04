#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use bytes;
use derivative::Derivative;
use std::collections::HashMap;

use std::time::{SystemTime};
use crate::energistics::etp::v12::datatypes::uuid::Uuid;
use crate::helpers::*;impl Default for crate::energistics::etp::v12::protocol::core::request_session::RequestSession {
	/* Protocol 0, MessageType : 1 */
	fn default()
	-> crate::energistics::etp::v12::protocol::core::request_session::RequestSession {
		crate::energistics::etp::v12::protocol::core::request_session::RequestSession {
			application_name : "".to_string(),
			application_version : "".to_string(),
			client_instance_id : Uuid::new_v4(),
			requested_protocols : vec![],
			supported_data_objects : vec![],
			supported_compression : vec!(),
			supported_formats : vec!("xml".to_string(),),
			current_date_time : time_to_etp(SystemTime::now()),
			earliest_retained_change_time : time_to_etp(SystemTime::now()),
			server_authorization_required : false,
			endpoint_capabilities : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::core::open_session::OpenSession {
	/* Protocol 0, MessageType : 2 */
	fn default()
	-> crate::energistics::etp::v12::protocol::core::open_session::OpenSession {
		crate::energistics::etp::v12::protocol::core::open_session::OpenSession {
			application_name : "".to_string(),
			application_version : "".to_string(),
			server_instance_id : Uuid::new_v4(),
			supported_protocols : vec![],
			supported_data_objects : vec![],
			supported_compression : "".to_string(),
			supported_formats : vec!("xml".to_string(),),
			current_date_time : time_to_etp(SystemTime::now()),
			earliest_retained_change_time : time_to_etp(SystemTime::now()),
			session_id : Uuid::new_v4(),
			endpoint_capabilities : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::core::close_session::CloseSession {
	/* Protocol 0, MessageType : 5 */
	fn default()
	-> crate::energistics::etp::v12::protocol::core::close_session::CloseSession {
		crate::energistics::etp::v12::protocol::core::close_session::CloseSession {
			reason : "".to_string(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::core::authorize::Authorize {
	/* Protocol 0, MessageType : 6 */
	fn default()
	-> crate::energistics::etp::v12::protocol::core::authorize::Authorize {
		crate::energistics::etp::v12::protocol::core::authorize::Authorize {
			authorization : "".to_string(),
			supplemental_authorization : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::core::authorize_response::AuthorizeResponse {
	/* Protocol 0, MessageType : 7 */
	fn default()
	-> crate::energistics::etp::v12::protocol::core::authorize_response::AuthorizeResponse {
		crate::energistics::etp::v12::protocol::core::authorize_response::AuthorizeResponse {
			success : true,
			challenges : vec![],
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::core::ping::Ping {
	/* Protocol 0, MessageType : 8 */
	fn default()
	-> crate::energistics::etp::v12::protocol::core::ping::Ping {
		crate::energistics::etp::v12::protocol::core::ping::Ping {
			current_date_time : time_to_etp(SystemTime::now()),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::core::pong::Pong {
	/* Protocol 0, MessageType : 9 */
	fn default()
	-> crate::energistics::etp::v12::protocol::core::pong::Pong {
		crate::energistics::etp::v12::protocol::core::pong::Pong {
			current_date_time : time_to_etp(SystemTime::now()),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::core::protocol_exception::ProtocolException {
	/* Protocol 0, MessageType : 1000 */
	fn default()
	-> crate::energistics::etp::v12::protocol::core::protocol_exception::ProtocolException {
		crate::energistics::etp::v12::protocol::core::protocol_exception::ProtocolException {
			error : None,
			errors : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::core::acknowledge::Acknowledge {
	/* Protocol 0, MessageType : 1001 */
	fn default()
	-> crate::energistics::etp::v12::protocol::core::acknowledge::Acknowledge {
		crate::energistics::etp::v12::protocol::core::acknowledge::Acknowledge {
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_streaming::channel_metadata::ChannelMetadata {
	/* Protocol 1, MessageType : 1 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_streaming::channel_metadata::ChannelMetadata {
		crate::energistics::etp::v12::protocol::channel_streaming::channel_metadata::ChannelMetadata {
			channels : vec![],
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_streaming::channel_data::ChannelData {
	/* Protocol 1, MessageType : 2 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_streaming::channel_data::ChannelData {
		crate::energistics::etp::v12::protocol::channel_streaming::channel_data::ChannelData {
			data : vec![],
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_streaming::start_streaming::StartStreaming {
	/* Protocol 1, MessageType : 3 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_streaming::start_streaming::StartStreaming {
		crate::energistics::etp::v12::protocol::channel_streaming::start_streaming::StartStreaming {
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_streaming::stop_streaming::StopStreaming {
	/* Protocol 1, MessageType : 4 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_streaming::stop_streaming::StopStreaming {
		crate::energistics::etp::v12::protocol::channel_streaming::stop_streaming::StopStreaming {
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_streaming::truncate_channels::TruncateChannels {
	/* Protocol 1, MessageType : 5 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_streaming::truncate_channels::TruncateChannels {
		crate::energistics::etp::v12::protocol::channel_streaming::truncate_channels::TruncateChannels {
			channels : vec![],
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_data_frame::get_frame_metadata::GetFrameMetadata {
	/* Protocol 2, MessageType : 1 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_data_frame::get_frame_metadata::GetFrameMetadata {
		crate::energistics::etp::v12::protocol::channel_data_frame::get_frame_metadata::GetFrameMetadata {
			uri : "".to_string(),
			include_all_channel_secondary_indexes : false,
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_data_frame::get_frame_metadata_response::GetFrameMetadataResponse {
	/* Protocol 2, MessageType : 2 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_data_frame::get_frame_metadata_response::GetFrameMetadataResponse {
		crate::energistics::etp::v12::protocol::channel_data_frame::get_frame_metadata_response::GetFrameMetadataResponse {
			uri : "".to_string(),
			indexes : vec![],
			channels : vec![],
		}
	}
}

impl crate::energistics::etp::v12::protocol::channel_data_frame::get_frame::GetFrame {
	/* Protocol 2, MessageType : 3 */
	pub fn default_with_params(_requested_interval :crate::energistics::etp::v12::datatypes::object::index_interval::IndexInterval,)
	-> crate::energistics::etp::v12::protocol::channel_data_frame::get_frame::GetFrame {
		crate::energistics::etp::v12::protocol::channel_data_frame::get_frame::GetFrame {
			uri : "".to_string(),
			include_all_channel_secondary_indexes : false,
			requested_interval : _requested_interval,
			request_uuid : Uuid::new_v4(),
			requested_secondary_intervals : vec!(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_data_frame::get_frame_response_header::GetFrameResponseHeader {
	/* Protocol 2, MessageType : 4 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_data_frame::get_frame_response_header::GetFrameResponseHeader {
		crate::energistics::etp::v12::protocol::channel_data_frame::get_frame_response_header::GetFrameResponseHeader {
			channel_uris : vec![],
			indexes : vec![],
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_data_frame::cancel_get_frame::CancelGetFrame {
	/* Protocol 2, MessageType : 5 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_data_frame::cancel_get_frame::CancelGetFrame {
		crate::energistics::etp::v12::protocol::channel_data_frame::cancel_get_frame::CancelGetFrame {
			request_uuid : Uuid::new_v4(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_data_frame::get_frame_response_rows::GetFrameResponseRows {
	/* Protocol 2, MessageType : 6 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_data_frame::get_frame_response_rows::GetFrameResponseRows {
		crate::energistics::etp::v12::protocol::channel_data_frame::get_frame_response_rows::GetFrameResponseRows {
			frame : vec![],
		}
	}
}

impl crate::energistics::etp::v12::protocol::discovery::get_resources::GetResources {
	/* Protocol 3, MessageType : 1 */
	pub fn default_with_params(_context :crate::energistics::etp::v12::datatypes::object::context_info::ContextInfo,_scope :crate::energistics::etp::v12::datatypes::object::context_scope_kind::ContextScopeKind,)
	-> crate::energistics::etp::v12::protocol::discovery::get_resources::GetResources {
		crate::energistics::etp::v12::protocol::discovery::get_resources::GetResources {
			context : _context,
			scope : _scope,
			count_objects : false,
			store_last_write_filter : None,
			active_status_filter : None,
			include_edges : false,
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::discovery::get_resources_response::GetResourcesResponse {
	/* Protocol 3, MessageType : 4 */
	fn default()
	-> crate::energistics::etp::v12::protocol::discovery::get_resources_response::GetResourcesResponse {
		crate::energistics::etp::v12::protocol::discovery::get_resources_response::GetResourcesResponse {
			resources : vec!(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::discovery::get_deleted_resources::GetDeletedResources {
	/* Protocol 3, MessageType : 5 */
	fn default()
	-> crate::energistics::etp::v12::protocol::discovery::get_deleted_resources::GetDeletedResources {
		crate::energistics::etp::v12::protocol::discovery::get_deleted_resources::GetDeletedResources {
			dataspace_uri : "".to_string(),
			delete_time_filter : None,
			data_object_types : vec!(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::discovery::get_deleted_resources_response::GetDeletedResourcesResponse {
	/* Protocol 3, MessageType : 6 */
	fn default()
	-> crate::energistics::etp::v12::protocol::discovery::get_deleted_resources_response::GetDeletedResourcesResponse {
		crate::energistics::etp::v12::protocol::discovery::get_deleted_resources_response::GetDeletedResourcesResponse {
			deleted_resources : vec!(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::discovery::get_resources_edges_response::GetResourcesEdgesResponse {
	/* Protocol 3, MessageType : 7 */
	fn default()
	-> crate::energistics::etp::v12::protocol::discovery::get_resources_edges_response::GetResourcesEdgesResponse {
		crate::energistics::etp::v12::protocol::discovery::get_resources_edges_response::GetResourcesEdgesResponse {
			edges : vec![],
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::store::get_data_objects::GetDataObjects {
	/* Protocol 4, MessageType : 1 */
	fn default()
	-> crate::energistics::etp::v12::protocol::store::get_data_objects::GetDataObjects {
		crate::energistics::etp::v12::protocol::store::get_data_objects::GetDataObjects {
			uris : HashMap::new(),
			format : "xml".to_string(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::store::put_data_objects::PutDataObjects {
	/* Protocol 4, MessageType : 2 */
	fn default()
	-> crate::energistics::etp::v12::protocol::store::put_data_objects::PutDataObjects {
		crate::energistics::etp::v12::protocol::store::put_data_objects::PutDataObjects {
			data_objects : HashMap::new(),
			prune_contained_objects : false,
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::store::delete_data_objects::DeleteDataObjects {
	/* Protocol 4, MessageType : 3 */
	fn default()
	-> crate::energistics::etp::v12::protocol::store::delete_data_objects::DeleteDataObjects {
		crate::energistics::etp::v12::protocol::store::delete_data_objects::DeleteDataObjects {
			uris : HashMap::new(),
			prune_contained_objects : false,
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::store::get_data_objects_response::GetDataObjectsResponse {
	/* Protocol 4, MessageType : 4 */
	fn default()
	-> crate::energistics::etp::v12::protocol::store::get_data_objects_response::GetDataObjectsResponse {
		crate::energistics::etp::v12::protocol::store::get_data_objects_response::GetDataObjectsResponse {
			data_objects : HashMap::new(),
		}
	}
}

impl crate::energistics::etp::v12::protocol::store::chunk::Chunk {
	/* Protocol 4, MessageType : 8 */
	pub fn default_with_params(_data :Vec<u8>,)
	-> crate::energistics::etp::v12::protocol::store::chunk::Chunk {
		crate::energistics::etp::v12::protocol::store::chunk::Chunk {
			blob_id : Uuid::new_v4(),
			data : _data,
			final_  : true,
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::store::put_data_objects_response::PutDataObjectsResponse {
	/* Protocol 4, MessageType : 9 */
	fn default()
	-> crate::energistics::etp::v12::protocol::store::put_data_objects_response::PutDataObjectsResponse {
		crate::energistics::etp::v12::protocol::store::put_data_objects_response::PutDataObjectsResponse {
			success : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::store::delete_data_objects_response::DeleteDataObjectsResponse {
	/* Protocol 4, MessageType : 10 */
	fn default()
	-> crate::energistics::etp::v12::protocol::store::delete_data_objects_response::DeleteDataObjectsResponse {
		crate::energistics::etp::v12::protocol::store::delete_data_objects_response::DeleteDataObjectsResponse {
			deleted_uris : HashMap::new(),
		}
	}
}

impl crate::energistics::etp::v12::protocol::store_notification::object_changed::ObjectChanged {
	/* Protocol 5, MessageType : 2 */
	pub fn default_with_params(_change :crate::energistics::etp::v12::datatypes::object::object_change::ObjectChange,)
	-> crate::energistics::etp::v12::protocol::store_notification::object_changed::ObjectChanged {
		crate::energistics::etp::v12::protocol::store_notification::object_changed::ObjectChanged {
			change : _change,
			request_uuid : Uuid::new_v4(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::store_notification::object_deleted::ObjectDeleted {
	/* Protocol 5, MessageType : 3 */
	fn default()
	-> crate::energistics::etp::v12::protocol::store_notification::object_deleted::ObjectDeleted {
		crate::energistics::etp::v12::protocol::store_notification::object_deleted::ObjectDeleted {
			uri : "".to_string(),
			change_time : time_to_etp(SystemTime::now()),
			request_uuid : Uuid::new_v4(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::store_notification::unsubscribe_notifications::UnsubscribeNotifications {
	/* Protocol 5, MessageType : 4 */
	fn default()
	-> crate::energistics::etp::v12::protocol::store_notification::unsubscribe_notifications::UnsubscribeNotifications {
		crate::energistics::etp::v12::protocol::store_notification::unsubscribe_notifications::UnsubscribeNotifications {
			request_uuid : Uuid::new_v4(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::store_notification::object_access_revoked::ObjectAccessRevoked {
	/* Protocol 5, MessageType : 5 */
	fn default()
	-> crate::energistics::etp::v12::protocol::store_notification::object_access_revoked::ObjectAccessRevoked {
		crate::energistics::etp::v12::protocol::store_notification::object_access_revoked::ObjectAccessRevoked {
			uri : "".to_string(),
			change_time : time_to_etp(SystemTime::now()),
			request_uuid : Uuid::new_v4(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::store_notification::subscribe_notifications::SubscribeNotifications {
	/* Protocol 5, MessageType : 6 */
	fn default()
	-> crate::energistics::etp::v12::protocol::store_notification::subscribe_notifications::SubscribeNotifications {
		crate::energistics::etp::v12::protocol::store_notification::subscribe_notifications::SubscribeNotifications {
			request : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::store_notification::subscription_ended::SubscriptionEnded {
	/* Protocol 5, MessageType : 7 */
	fn default()
	-> crate::energistics::etp::v12::protocol::store_notification::subscription_ended::SubscriptionEnded {
		crate::energistics::etp::v12::protocol::store_notification::subscription_ended::SubscriptionEnded {
			reason : "".to_string(),
			request_uuid : Uuid::new_v4(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::store_notification::unsolicited_store_notifications::UnsolicitedStoreNotifications {
	/* Protocol 5, MessageType : 8 */
	fn default()
	-> crate::energistics::etp::v12::protocol::store_notification::unsolicited_store_notifications::UnsolicitedStoreNotifications {
		crate::energistics::etp::v12::protocol::store_notification::unsolicited_store_notifications::UnsolicitedStoreNotifications {
			subscriptions : vec![],
		}
	}
}

impl crate::energistics::etp::v12::protocol::store_notification::chunk::Chunk {
	/* Protocol 5, MessageType : 9 */
	pub fn default_with_params(_data :Vec<u8>,)
	-> crate::energistics::etp::v12::protocol::store_notification::chunk::Chunk {
		crate::energistics::etp::v12::protocol::store_notification::chunk::Chunk {
			blob_id : Uuid::new_v4(),
			data : _data,
			final_  : true,
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::store_notification::subscribe_notifications_response::SubscribeNotificationsResponse {
	/* Protocol 5, MessageType : 10 */
	fn default()
	-> crate::energistics::etp::v12::protocol::store_notification::subscribe_notifications_response::SubscribeNotificationsResponse {
		crate::energistics::etp::v12::protocol::store_notification::subscribe_notifications_response::SubscribeNotificationsResponse {
			success : HashMap::new(),
		}
	}
}

impl crate::energistics::etp::v12::protocol::store_notification::object_active_status_changed::ObjectActiveStatusChanged {
	/* Protocol 5, MessageType : 11 */
	pub fn default_with_params(_active_status :crate::energistics::etp::v12::datatypes::object::active_status_kind::ActiveStatusKind,_resource :crate::energistics::etp::v12::datatypes::object::resource::Resource,)
	-> crate::energistics::etp::v12::protocol::store_notification::object_active_status_changed::ObjectActiveStatusChanged {
		crate::energistics::etp::v12::protocol::store_notification::object_active_status_changed::ObjectActiveStatusChanged {
			active_status : _active_status,
			change_time : time_to_etp(SystemTime::now()),
			resource : _resource,
			request_uuid : Uuid::new_v4(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object::delete_parts::DeleteParts {
	/* Protocol 6, MessageType : 1 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object::delete_parts::DeleteParts {
		crate::energistics::etp::v12::protocol::growing_object::delete_parts::DeleteParts {
			uri : "".to_string(),
			uids : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object::get_parts::GetParts {
	/* Protocol 6, MessageType : 3 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object::get_parts::GetParts {
		crate::energistics::etp::v12::protocol::growing_object::get_parts::GetParts {
			uri : "".to_string(),
			format : "xml".to_string(),
			uids : HashMap::new(),
		}
	}
}

impl crate::energistics::etp::v12::protocol::growing_object::get_parts_by_range::GetPartsByRange {
	/* Protocol 6, MessageType : 4 */
	pub fn default_with_params(_index_interval :crate::energistics::etp::v12::datatypes::object::index_interval::IndexInterval,)
	-> crate::energistics::etp::v12::protocol::growing_object::get_parts_by_range::GetPartsByRange {
		crate::energistics::etp::v12::protocol::growing_object::get_parts_by_range::GetPartsByRange {
			uri : "".to_string(),
			format : "xml".to_string(),
			index_interval : _index_interval,
			include_overlapping_intervals : true,
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object::put_parts::PutParts {
	/* Protocol 6, MessageType : 5 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object::put_parts::PutParts {
		crate::energistics::etp::v12::protocol::growing_object::put_parts::PutParts {
			uri : "".to_string(),
			format : "xml".to_string(),
			parts : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object::get_parts_response::GetPartsResponse {
	/* Protocol 6, MessageType : 6 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object::get_parts_response::GetPartsResponse {
		crate::energistics::etp::v12::protocol::growing_object::get_parts_response::GetPartsResponse {
			uri : "".to_string(),
			format : "xml".to_string(),
			parts : HashMap::new(),
		}
	}
}

impl crate::energistics::etp::v12::protocol::growing_object::replace_parts_by_range::ReplacePartsByRange {
	/* Protocol 6, MessageType : 7 */
	pub fn default_with_params(_delete_interval :crate::energistics::etp::v12::datatypes::object::index_interval::IndexInterval,)
	-> crate::energistics::etp::v12::protocol::growing_object::replace_parts_by_range::ReplacePartsByRange {
		crate::energistics::etp::v12::protocol::growing_object::replace_parts_by_range::ReplacePartsByRange {
			uri : "".to_string(),
			delete_interval : _delete_interval,
			include_overlapping_intervals : true,
			format : "xml".to_string(),
			parts : vec![],
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object::get_parts_metadata::GetPartsMetadata {
	/* Protocol 6, MessageType : 8 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object::get_parts_metadata::GetPartsMetadata {
		crate::energistics::etp::v12::protocol::growing_object::get_parts_metadata::GetPartsMetadata {
			uris : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object::get_parts_metadata_response::GetPartsMetadataResponse {
	/* Protocol 6, MessageType : 9 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object::get_parts_metadata_response::GetPartsMetadataResponse {
		crate::energistics::etp::v12::protocol::growing_object::get_parts_metadata_response::GetPartsMetadataResponse {
			metadata : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object::get_parts_by_range_response::GetPartsByRangeResponse {
	/* Protocol 6, MessageType : 10 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object::get_parts_by_range_response::GetPartsByRangeResponse {
		crate::energistics::etp::v12::protocol::growing_object::get_parts_by_range_response::GetPartsByRangeResponse {
			uri : "".to_string(),
			format : "xml".to_string(),
			parts : vec![],
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object::delete_parts_response::DeletePartsResponse {
	/* Protocol 6, MessageType : 11 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object::delete_parts_response::DeletePartsResponse {
		crate::energistics::etp::v12::protocol::growing_object::delete_parts_response::DeletePartsResponse {
			success : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object::put_parts_response::PutPartsResponse {
	/* Protocol 6, MessageType : 13 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object::put_parts_response::PutPartsResponse {
		crate::energistics::etp::v12::protocol::growing_object::put_parts_response::PutPartsResponse {
			success : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object::get_growing_data_objects_header::GetGrowingDataObjectsHeader {
	/* Protocol 6, MessageType : 14 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object::get_growing_data_objects_header::GetGrowingDataObjectsHeader {
		crate::energistics::etp::v12::protocol::growing_object::get_growing_data_objects_header::GetGrowingDataObjectsHeader {
			uris : HashMap::new(),
			format : "xml".to_string(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object::get_growing_data_objects_header_response::GetGrowingDataObjectsHeaderResponse {
	/* Protocol 6, MessageType : 15 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object::get_growing_data_objects_header_response::GetGrowingDataObjectsHeaderResponse {
		crate::energistics::etp::v12::protocol::growing_object::get_growing_data_objects_header_response::GetGrowingDataObjectsHeaderResponse {
			data_objects : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object::put_growing_data_objects_header::PutGrowingDataObjectsHeader {
	/* Protocol 6, MessageType : 16 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object::put_growing_data_objects_header::PutGrowingDataObjectsHeader {
		crate::energistics::etp::v12::protocol::growing_object::put_growing_data_objects_header::PutGrowingDataObjectsHeader {
			data_objects : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object::put_growing_data_objects_header_response::PutGrowingDataObjectsHeaderResponse {
	/* Protocol 6, MessageType : 17 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object::put_growing_data_objects_header_response::PutGrowingDataObjectsHeaderResponse {
		crate::energistics::etp::v12::protocol::growing_object::put_growing_data_objects_header_response::PutGrowingDataObjectsHeaderResponse {
			success : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object::replace_parts_by_range_response::ReplacePartsByRangeResponse {
	/* Protocol 6, MessageType : 18 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object::replace_parts_by_range_response::ReplacePartsByRangeResponse {
		crate::energistics::etp::v12::protocol::growing_object::replace_parts_by_range_response::ReplacePartsByRangeResponse {
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object::get_change_annotations::GetChangeAnnotations {
	/* Protocol 6, MessageType : 19 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object::get_change_annotations::GetChangeAnnotations {
		crate::energistics::etp::v12::protocol::growing_object::get_change_annotations::GetChangeAnnotations {
			since_change_time : time_to_etp(SystemTime::now()),
			uris : HashMap::new(),
			latest_only : false,
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object::get_change_annotations_response::GetChangeAnnotationsResponse {
	/* Protocol 6, MessageType : 20 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object::get_change_annotations_response::GetChangeAnnotationsResponse {
		crate::energistics::etp::v12::protocol::growing_object::get_change_annotations_response::GetChangeAnnotationsResponse {
			changes : HashMap::new(),
		}
	}
}

impl crate::energistics::etp::v12::protocol::growing_object_notification::parts_changed::PartsChanged {
	/* Protocol 7, MessageType : 2 */
	pub fn default_with_params(_change_kind :crate::energistics::etp::v12::datatypes::object::object_change_kind::ObjectChangeKind,)
	-> crate::energistics::etp::v12::protocol::growing_object_notification::parts_changed::PartsChanged {
		crate::energistics::etp::v12::protocol::growing_object_notification::parts_changed::PartsChanged {
			uri : "".to_string(),
			request_uuid : Uuid::new_v4(),
			change_kind : _change_kind,
			change_time : time_to_etp(SystemTime::now()),
			format : "".to_string(),
			parts : vec![],
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object_notification::parts_deleted::PartsDeleted {
	/* Protocol 7, MessageType : 3 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object_notification::parts_deleted::PartsDeleted {
		crate::energistics::etp::v12::protocol::growing_object_notification::parts_deleted::PartsDeleted {
			uri : "".to_string(),
			request_uuid : Uuid::new_v4(),
			change_time : time_to_etp(SystemTime::now()),
			uids : vec![],
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object_notification::unsubscribe_part_notification::UnsubscribePartNotification {
	/* Protocol 7, MessageType : 4 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object_notification::unsubscribe_part_notification::UnsubscribePartNotification {
		crate::energistics::etp::v12::protocol::growing_object_notification::unsubscribe_part_notification::UnsubscribePartNotification {
			request_uuid : Uuid::new_v4(),
		}
	}
}

impl crate::energistics::etp::v12::protocol::growing_object_notification::parts_replaced_by_range::PartsReplacedByRange {
	/* Protocol 7, MessageType : 6 */
	pub fn default_with_params(_deleted_interval :crate::energistics::etp::v12::datatypes::object::index_interval::IndexInterval,)
	-> crate::energistics::etp::v12::protocol::growing_object_notification::parts_replaced_by_range::PartsReplacedByRange {
		crate::energistics::etp::v12::protocol::growing_object_notification::parts_replaced_by_range::PartsReplacedByRange {
			uri : "".to_string(),
			request_uuid : Uuid::new_v4(),
			change_time : time_to_etp(SystemTime::now()),
			deleted_interval : _deleted_interval,
			include_overlapping_intervals : true,
			format : "".to_string(),
			parts : vec![],
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object_notification::subscribe_part_notifications::SubscribePartNotifications {
	/* Protocol 7, MessageType : 7 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object_notification::subscribe_part_notifications::SubscribePartNotifications {
		crate::energistics::etp::v12::protocol::growing_object_notification::subscribe_part_notifications::SubscribePartNotifications {
			request : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object_notification::part_subscription_ended::PartSubscriptionEnded {
	/* Protocol 7, MessageType : 8 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object_notification::part_subscription_ended::PartSubscriptionEnded {
		crate::energistics::etp::v12::protocol::growing_object_notification::part_subscription_ended::PartSubscriptionEnded {
			reason : "".to_string(),
			request_uuid : Uuid::new_v4(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object_notification::unsolicited_part_notifications::UnsolicitedPartNotifications {
	/* Protocol 7, MessageType : 9 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object_notification::unsolicited_part_notifications::UnsolicitedPartNotifications {
		crate::energistics::etp::v12::protocol::growing_object_notification::unsolicited_part_notifications::UnsolicitedPartNotifications {
			subscriptions : vec![],
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object_notification::subscribe_part_notifications_response::SubscribePartNotificationsResponse {
	/* Protocol 7, MessageType : 10 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object_notification::subscribe_part_notifications_response::SubscribePartNotificationsResponse {
		crate::energistics::etp::v12::protocol::growing_object_notification::subscribe_part_notifications_response::SubscribePartNotificationsResponse {
			success : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::data_array::get_data_arrays_response::GetDataArraysResponse {
	/* Protocol 9, MessageType : 1 */
	fn default()
	-> crate::energistics::etp::v12::protocol::data_array::get_data_arrays_response::GetDataArraysResponse {
		crate::energistics::etp::v12::protocol::data_array::get_data_arrays_response::GetDataArraysResponse {
			data_arrays : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::data_array::get_data_arrays::GetDataArrays {
	/* Protocol 9, MessageType : 2 */
	fn default()
	-> crate::energistics::etp::v12::protocol::data_array::get_data_arrays::GetDataArrays {
		crate::energistics::etp::v12::protocol::data_array::get_data_arrays::GetDataArrays {
			data_arrays : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::data_array::get_data_subarrays::GetDataSubarrays {
	/* Protocol 9, MessageType : 3 */
	fn default()
	-> crate::energistics::etp::v12::protocol::data_array::get_data_subarrays::GetDataSubarrays {
		crate::energistics::etp::v12::protocol::data_array::get_data_subarrays::GetDataSubarrays {
			data_subarrays : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::data_array::put_data_arrays::PutDataArrays {
	/* Protocol 9, MessageType : 4 */
	fn default()
	-> crate::energistics::etp::v12::protocol::data_array::put_data_arrays::PutDataArrays {
		crate::energistics::etp::v12::protocol::data_array::put_data_arrays::PutDataArrays {
			data_arrays : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::data_array::put_data_subarrays::PutDataSubarrays {
	/* Protocol 9, MessageType : 5 */
	fn default()
	-> crate::energistics::etp::v12::protocol::data_array::put_data_subarrays::PutDataSubarrays {
		crate::energistics::etp::v12::protocol::data_array::put_data_subarrays::PutDataSubarrays {
			data_subarrays : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::data_array::get_data_array_metadata::GetDataArrayMetadata {
	/* Protocol 9, MessageType : 6 */
	fn default()
	-> crate::energistics::etp::v12::protocol::data_array::get_data_array_metadata::GetDataArrayMetadata {
		crate::energistics::etp::v12::protocol::data_array::get_data_array_metadata::GetDataArrayMetadata {
			data_arrays : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::data_array::get_data_array_metadata_response::GetDataArrayMetadataResponse {
	/* Protocol 9, MessageType : 7 */
	fn default()
	-> crate::energistics::etp::v12::protocol::data_array::get_data_array_metadata_response::GetDataArrayMetadataResponse {
		crate::energistics::etp::v12::protocol::data_array::get_data_array_metadata_response::GetDataArrayMetadataResponse {
			array_metadata : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::data_array::get_data_subarrays_response::GetDataSubarraysResponse {
	/* Protocol 9, MessageType : 8 */
	fn default()
	-> crate::energistics::etp::v12::protocol::data_array::get_data_subarrays_response::GetDataSubarraysResponse {
		crate::energistics::etp::v12::protocol::data_array::get_data_subarrays_response::GetDataSubarraysResponse {
			data_subarrays : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::data_array::put_uninitialized_data_arrays::PutUninitializedDataArrays {
	/* Protocol 9, MessageType : 9 */
	fn default()
	-> crate::energistics::etp::v12::protocol::data_array::put_uninitialized_data_arrays::PutUninitializedDataArrays {
		crate::energistics::etp::v12::protocol::data_array::put_uninitialized_data_arrays::PutUninitializedDataArrays {
			data_arrays : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::data_array::put_data_arrays_response::PutDataArraysResponse {
	/* Protocol 9, MessageType : 10 */
	fn default()
	-> crate::energistics::etp::v12::protocol::data_array::put_data_arrays_response::PutDataArraysResponse {
		crate::energistics::etp::v12::protocol::data_array::put_data_arrays_response::PutDataArraysResponse {
			success : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::data_array::put_data_subarrays_response::PutDataSubarraysResponse {
	/* Protocol 9, MessageType : 11 */
	fn default()
	-> crate::energistics::etp::v12::protocol::data_array::put_data_subarrays_response::PutDataSubarraysResponse {
		crate::energistics::etp::v12::protocol::data_array::put_data_subarrays_response::PutDataSubarraysResponse {
			success : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::data_array::put_uninitialized_data_arrays_response::PutUninitializedDataArraysResponse {
	/* Protocol 9, MessageType : 12 */
	fn default()
	-> crate::energistics::etp::v12::protocol::data_array::put_uninitialized_data_arrays_response::PutUninitializedDataArraysResponse {
		crate::energistics::etp::v12::protocol::data_array::put_uninitialized_data_arrays_response::PutUninitializedDataArraysResponse {
			success : HashMap::new(),
		}
	}
}

impl crate::energistics::etp::v12::protocol::discovery_query::find_resources::FindResources {
	/* Protocol 13, MessageType : 1 */
	pub fn default_with_params(_context :crate::energistics::etp::v12::datatypes::object::context_info::ContextInfo,_scope :crate::energistics::etp::v12::datatypes::object::context_scope_kind::ContextScopeKind,)
	-> crate::energistics::etp::v12::protocol::discovery_query::find_resources::FindResources {
		crate::energistics::etp::v12::protocol::discovery_query::find_resources::FindResources {
			context : _context,
			scope : _scope,
			store_last_write_filter : None,
			active_status_filter : None,
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::discovery_query::find_resources_response::FindResourcesResponse {
	/* Protocol 13, MessageType : 2 */
	fn default()
	-> crate::energistics::etp::v12::protocol::discovery_query::find_resources_response::FindResourcesResponse {
		crate::energistics::etp::v12::protocol::discovery_query::find_resources_response::FindResourcesResponse {
			resources : vec!(),
			server_sort_order : "".to_string(),
		}
	}
}

impl crate::energistics::etp::v12::protocol::store_query::find_data_objects::FindDataObjects {
	/* Protocol 14, MessageType : 1 */
	pub fn default_with_params(_context :crate::energistics::etp::v12::datatypes::object::context_info::ContextInfo,_scope :crate::energistics::etp::v12::datatypes::object::context_scope_kind::ContextScopeKind,)
	-> crate::energistics::etp::v12::protocol::store_query::find_data_objects::FindDataObjects {
		crate::energistics::etp::v12::protocol::store_query::find_data_objects::FindDataObjects {
			context : _context,
			scope : _scope,
			store_last_write_filter : None,
			active_status_filter : None,
			format : "xml".to_string(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::store_query::find_data_objects_response::FindDataObjectsResponse {
	/* Protocol 14, MessageType : 2 */
	fn default()
	-> crate::energistics::etp::v12::protocol::store_query::find_data_objects_response::FindDataObjectsResponse {
		crate::energistics::etp::v12::protocol::store_query::find_data_objects_response::FindDataObjectsResponse {
			data_objects : vec!(),
			server_sort_order : "".to_string(),
		}
	}
}

impl crate::energistics::etp::v12::protocol::store_query::chunk::Chunk {
	/* Protocol 14, MessageType : 3 */
	pub fn default_with_params(_data :Vec<u8>,)
	-> crate::energistics::etp::v12::protocol::store_query::chunk::Chunk {
		crate::energistics::etp::v12::protocol::store_query::chunk::Chunk {
			blob_id : Uuid::new_v4(),
			data : _data,
			final_  : true,
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object_query::find_parts::FindParts {
	/* Protocol 16, MessageType : 1 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object_query::find_parts::FindParts {
		crate::energistics::etp::v12::protocol::growing_object_query::find_parts::FindParts {
			uri : "".to_string(),
			format : "xml".to_string(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::growing_object_query::find_parts_response::FindPartsResponse {
	/* Protocol 16, MessageType : 2 */
	fn default()
	-> crate::energistics::etp::v12::protocol::growing_object_query::find_parts_response::FindPartsResponse {
		crate::energistics::etp::v12::protocol::growing_object_query::find_parts_response::FindPartsResponse {
			uri : "".to_string(),
			server_sort_order : "".to_string(),
			format : "xml".to_string(),
			parts : vec!(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::transaction::start_transaction::StartTransaction {
	/* Protocol 18, MessageType : 1 */
	fn default()
	-> crate::energistics::etp::v12::protocol::transaction::start_transaction::StartTransaction {
		crate::energistics::etp::v12::protocol::transaction::start_transaction::StartTransaction {
			read_only : true,
			message : "".to_string(),
			dataspace_uris : vec!("".to_string(),),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::transaction::start_transaction_response::StartTransactionResponse {
	/* Protocol 18, MessageType : 2 */
	fn default()
	-> crate::energistics::etp::v12::protocol::transaction::start_transaction_response::StartTransactionResponse {
		crate::energistics::etp::v12::protocol::transaction::start_transaction_response::StartTransactionResponse {
			transaction_uuid : Uuid::new_v4(),
			successful : true,
			failure_reason : "".to_string(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::transaction::commit_transaction::CommitTransaction {
	/* Protocol 18, MessageType : 3 */
	fn default()
	-> crate::energistics::etp::v12::protocol::transaction::commit_transaction::CommitTransaction {
		crate::energistics::etp::v12::protocol::transaction::commit_transaction::CommitTransaction {
			transaction_uuid : Uuid::new_v4(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::transaction::rollback_transaction::RollbackTransaction {
	/* Protocol 18, MessageType : 4 */
	fn default()
	-> crate::energistics::etp::v12::protocol::transaction::rollback_transaction::RollbackTransaction {
		crate::energistics::etp::v12::protocol::transaction::rollback_transaction::RollbackTransaction {
			transaction_uuid : Uuid::new_v4(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::transaction::commit_transaction_response::CommitTransactionResponse {
	/* Protocol 18, MessageType : 5 */
	fn default()
	-> crate::energistics::etp::v12::protocol::transaction::commit_transaction_response::CommitTransactionResponse {
		crate::energistics::etp::v12::protocol::transaction::commit_transaction_response::CommitTransactionResponse {
			transaction_uuid : Uuid::new_v4(),
			successful : true,
			failure_reason : "".to_string(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::transaction::rollback_transaction_response::RollbackTransactionResponse {
	/* Protocol 18, MessageType : 6 */
	fn default()
	-> crate::energistics::etp::v12::protocol::transaction::rollback_transaction_response::RollbackTransactionResponse {
		crate::energistics::etp::v12::protocol::transaction::rollback_transaction_response::RollbackTransactionResponse {
			transaction_uuid : Uuid::new_v4(),
			successful : true,
			failure_reason : "".to_string(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_subscribe::get_channel_metadata::GetChannelMetadata {
	/* Protocol 21, MessageType : 1 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_subscribe::get_channel_metadata::GetChannelMetadata {
		crate::energistics::etp::v12::protocol::channel_subscribe::get_channel_metadata::GetChannelMetadata {
			uris : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_subscribe::get_channel_metadata_response::GetChannelMetadataResponse {
	/* Protocol 21, MessageType : 2 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_subscribe::get_channel_metadata_response::GetChannelMetadataResponse {
		crate::energistics::etp::v12::protocol::channel_subscribe::get_channel_metadata_response::GetChannelMetadataResponse {
			metadata : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_subscribe::subscribe_channels::SubscribeChannels {
	/* Protocol 21, MessageType : 3 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_subscribe::subscribe_channels::SubscribeChannels {
		crate::energistics::etp::v12::protocol::channel_subscribe::subscribe_channels::SubscribeChannels {
			channels : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_subscribe::channel_data::ChannelData {
	/* Protocol 21, MessageType : 4 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_subscribe::channel_data::ChannelData {
		crate::energistics::etp::v12::protocol::channel_subscribe::channel_data::ChannelData {
			data : vec![],
		}
	}
}

impl crate::energistics::etp::v12::protocol::channel_subscribe::range_replaced::RangeReplaced {
	/* Protocol 21, MessageType : 6 */
	pub fn default_with_params(_changed_interval :crate::energistics::etp::v12::datatypes::object::index_interval::IndexInterval,)
	-> crate::energistics::etp::v12::protocol::channel_subscribe::range_replaced::RangeReplaced {
		crate::energistics::etp::v12::protocol::channel_subscribe::range_replaced::RangeReplaced {
			change_time : time_to_etp(SystemTime::now()),
			channel_ids : vec![],
			changed_interval : _changed_interval,
			data : vec![],
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_subscribe::unsubscribe_channels::UnsubscribeChannels {
	/* Protocol 21, MessageType : 7 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_subscribe::unsubscribe_channels::UnsubscribeChannels {
		crate::energistics::etp::v12::protocol::channel_subscribe::unsubscribe_channels::UnsubscribeChannels {
			channel_ids : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_subscribe::subscriptions_stopped::SubscriptionsStopped {
	/* Protocol 21, MessageType : 8 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_subscribe::subscriptions_stopped::SubscriptionsStopped {
		crate::energistics::etp::v12::protocol::channel_subscribe::subscriptions_stopped::SubscriptionsStopped {
			reason : "".to_string(),
			channel_ids : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_subscribe::get_ranges::GetRanges {
	/* Protocol 21, MessageType : 9 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_subscribe::get_ranges::GetRanges {
		crate::energistics::etp::v12::protocol::channel_subscribe::get_ranges::GetRanges {
			request_uuid : Uuid::new_v4(),
			channel_ranges : vec![],
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_subscribe::get_ranges_response::GetRangesResponse {
	/* Protocol 21, MessageType : 10 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_subscribe::get_ranges_response::GetRangesResponse {
		crate::energistics::etp::v12::protocol::channel_subscribe::get_ranges_response::GetRangesResponse {
			data : vec!(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_subscribe::cancel_get_ranges::CancelGetRanges {
	/* Protocol 21, MessageType : 11 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_subscribe::cancel_get_ranges::CancelGetRanges {
		crate::energistics::etp::v12::protocol::channel_subscribe::cancel_get_ranges::CancelGetRanges {
			request_uuid : Uuid::new_v4(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_subscribe::subscribe_channels_response::SubscribeChannelsResponse {
	/* Protocol 21, MessageType : 12 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_subscribe::subscribe_channels_response::SubscribeChannelsResponse {
		crate::energistics::etp::v12::protocol::channel_subscribe::subscribe_channels_response::SubscribeChannelsResponse {
			success : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_subscribe::channels_truncated::ChannelsTruncated {
	/* Protocol 21, MessageType : 13 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_subscribe::channels_truncated::ChannelsTruncated {
		crate::energistics::etp::v12::protocol::channel_subscribe::channels_truncated::ChannelsTruncated {
			channels : vec![],
			change_time : time_to_etp(SystemTime::now()),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_subscribe::get_change_annotations::GetChangeAnnotations {
	/* Protocol 21, MessageType : 14 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_subscribe::get_change_annotations::GetChangeAnnotations {
		crate::energistics::etp::v12::protocol::channel_subscribe::get_change_annotations::GetChangeAnnotations {
			channels : HashMap::new(),
			latest_only : false,
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_subscribe::get_change_annotations_response::GetChangeAnnotationsResponse {
	/* Protocol 21, MessageType : 15 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_subscribe::get_change_annotations_response::GetChangeAnnotationsResponse {
		crate::energistics::etp::v12::protocol::channel_subscribe::get_change_annotations_response::GetChangeAnnotationsResponse {
			changes : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_data_load::open_channels::OpenChannels {
	/* Protocol 22, MessageType : 1 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_data_load::open_channels::OpenChannels {
		crate::energistics::etp::v12::protocol::channel_data_load::open_channels::OpenChannels {
			uris : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_data_load::open_channels_response::OpenChannelsResponse {
	/* Protocol 22, MessageType : 2 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_data_load::open_channels_response::OpenChannelsResponse {
		crate::energistics::etp::v12::protocol::channel_data_load::open_channels_response::OpenChannelsResponse {
			channels : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_data_load::close_channels::CloseChannels {
	/* Protocol 22, MessageType : 3 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_data_load::close_channels::CloseChannels {
		crate::energistics::etp::v12::protocol::channel_data_load::close_channels::CloseChannels {
			id : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_data_load::channel_data::ChannelData {
	/* Protocol 22, MessageType : 4 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_data_load::channel_data::ChannelData {
		crate::energistics::etp::v12::protocol::channel_data_load::channel_data::ChannelData {
			data : vec![],
		}
	}
}

impl crate::energistics::etp::v12::protocol::channel_data_load::replace_range::ReplaceRange {
	/* Protocol 22, MessageType : 6 */
	pub fn default_with_params(_changed_interval :crate::energistics::etp::v12::datatypes::object::index_interval::IndexInterval,)
	-> crate::energistics::etp::v12::protocol::channel_data_load::replace_range::ReplaceRange {
		crate::energistics::etp::v12::protocol::channel_data_load::replace_range::ReplaceRange {
			changed_interval : _changed_interval,
			channel_ids : vec![],
			data : vec![],
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_data_load::channels_closed::ChannelsClosed {
	/* Protocol 22, MessageType : 7 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_data_load::channels_closed::ChannelsClosed {
		crate::energistics::etp::v12::protocol::channel_data_load::channels_closed::ChannelsClosed {
			reason : "".to_string(),
			id : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_data_load::replace_range_response::ReplaceRangeResponse {
	/* Protocol 22, MessageType : 8 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_data_load::replace_range_response::ReplaceRangeResponse {
		crate::energistics::etp::v12::protocol::channel_data_load::replace_range_response::ReplaceRangeResponse {
			channel_change_time : time_to_etp(SystemTime::now()),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_data_load::truncate_channels::TruncateChannels {
	/* Protocol 22, MessageType : 9 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_data_load::truncate_channels::TruncateChannels {
		crate::energistics::etp::v12::protocol::channel_data_load::truncate_channels::TruncateChannels {
			channels : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::channel_data_load::truncate_channels_response::TruncateChannelsResponse {
	/* Protocol 22, MessageType : 10 */
	fn default()
	-> crate::energistics::etp::v12::protocol::channel_data_load::truncate_channels_response::TruncateChannelsResponse {
		crate::energistics::etp::v12::protocol::channel_data_load::truncate_channels_response::TruncateChannelsResponse {
			channels_truncated_time : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::dataspace::get_dataspaces::GetDataspaces {
	/* Protocol 24, MessageType : 1 */
	fn default()
	-> crate::energistics::etp::v12::protocol::dataspace::get_dataspaces::GetDataspaces {
		crate::energistics::etp::v12::protocol::dataspace::get_dataspaces::GetDataspaces {
			store_last_write_filter : None,
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::dataspace::get_dataspaces_response::GetDataspacesResponse {
	/* Protocol 24, MessageType : 2 */
	fn default()
	-> crate::energistics::etp::v12::protocol::dataspace::get_dataspaces_response::GetDataspacesResponse {
		crate::energistics::etp::v12::protocol::dataspace::get_dataspaces_response::GetDataspacesResponse {
			dataspaces : vec!(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::dataspace::put_dataspaces::PutDataspaces {
	/* Protocol 24, MessageType : 3 */
	fn default()
	-> crate::energistics::etp::v12::protocol::dataspace::put_dataspaces::PutDataspaces {
		crate::energistics::etp::v12::protocol::dataspace::put_dataspaces::PutDataspaces {
			dataspaces : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::dataspace::delete_dataspaces::DeleteDataspaces {
	/* Protocol 24, MessageType : 4 */
	fn default()
	-> crate::energistics::etp::v12::protocol::dataspace::delete_dataspaces::DeleteDataspaces {
		crate::energistics::etp::v12::protocol::dataspace::delete_dataspaces::DeleteDataspaces {
			uris : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::dataspace::delete_dataspaces_response::DeleteDataspacesResponse {
	/* Protocol 24, MessageType : 5 */
	fn default()
	-> crate::energistics::etp::v12::protocol::dataspace::delete_dataspaces_response::DeleteDataspacesResponse {
		crate::energistics::etp::v12::protocol::dataspace::delete_dataspaces_response::DeleteDataspacesResponse {
			success : HashMap::new(),
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::dataspace::put_dataspaces_response::PutDataspacesResponse {
	/* Protocol 24, MessageType : 6 */
	fn default()
	-> crate::energistics::etp::v12::protocol::dataspace::put_dataspaces_response::PutDataspacesResponse {
		crate::energistics::etp::v12::protocol::dataspace::put_dataspaces_response::PutDataspacesResponse {
			success : HashMap::new(),
		}
	}
}

impl crate::energistics::etp::v12::protocol::supported_types::get_supported_types::GetSupportedTypes {
	/* Protocol 25, MessageType : 1 */
	pub fn default_with_params(_scope :crate::energistics::etp::v12::datatypes::object::context_scope_kind::ContextScopeKind,)
	-> crate::energistics::etp::v12::protocol::supported_types::get_supported_types::GetSupportedTypes {
		crate::energistics::etp::v12::protocol::supported_types::get_supported_types::GetSupportedTypes {
			uri : "".to_string(),
			scope : _scope,
			return_empty_types : false,
			count_objects : false,
		}
	}
}

impl Default for crate::energistics::etp::v12::protocol::supported_types::get_supported_types_response::GetSupportedTypesResponse {
	/* Protocol 25, MessageType : 2 */
	fn default()
	-> crate::energistics::etp::v12::protocol::supported_types::get_supported_types_response::GetSupportedTypesResponse {
		crate::energistics::etp::v12::protocol::supported_types::get_supported_types_response::GetSupportedTypesResponse {
			supported_types : vec!(),
		}
	}
}

impl Default for crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_add_to_store::WMLS_AddToStore {
	/* Protocol 2100, MessageType : 1 */
	fn default()
	-> crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_add_to_store::WMLS_AddToStore {
		crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_add_to_store::WMLS_AddToStore {
			wmltype_in : "".to_string(),
			xmlin : "".to_string(),
			options_in : "".to_string(),
			capabilities_in : "".to_string(),
		}
	}
}

impl crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_add_to_store_response::WMLS_AddToStoreResponse {
	/* Protocol 2100, MessageType : 2 */
	pub fn default_with_params(_result :i32,)
	-> crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_add_to_store_response::WMLS_AddToStoreResponse {
		crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_add_to_store_response::WMLS_AddToStoreResponse {
			result : _result,
			supp_msg_out : "".to_string(),
		}
	}
}

impl Default for crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_delete_from_store::WMLS_DeleteFromStore {
	/* Protocol 2100, MessageType : 3 */
	fn default()
	-> crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_delete_from_store::WMLS_DeleteFromStore {
		crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_delete_from_store::WMLS_DeleteFromStore {
			wmltype_in : "".to_string(),
			xmlin : "".to_string(),
			options_in : "".to_string(),
			capabilities_in : "".to_string(),
		}
	}
}

impl crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_delete_from_store_response::WMLS_DeleteFromStoreResponse {
	/* Protocol 2100, MessageType : 4 */
	pub fn default_with_params(_result :i32,)
	-> crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_delete_from_store_response::WMLS_DeleteFromStoreResponse {
		crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_delete_from_store_response::WMLS_DeleteFromStoreResponse {
			result : _result,
			supp_msg_out : "".to_string(),
		}
	}
}

impl crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_base_msg::WMLS_GetBaseMsg {
	/* Protocol 2100, MessageType : 5 */
	pub fn default_with_params(_return_value_in :i32,)
	-> crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_base_msg::WMLS_GetBaseMsg {
		crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_base_msg::WMLS_GetBaseMsg {
			return_value_in : _return_value_in,
		}
	}
}

impl Default for crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_base_msg_response::WMLS_GetBaseMsgResponse {
	/* Protocol 2100, MessageType : 6 */
	fn default()
	-> crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_base_msg_response::WMLS_GetBaseMsgResponse {
		crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_base_msg_response::WMLS_GetBaseMsgResponse {
			result : "".to_string(),
		}
	}
}

impl Default for crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_cap::WMLS_GetCap {
	/* Protocol 2100, MessageType : 7 */
	fn default()
	-> crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_cap::WMLS_GetCap {
		crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_cap::WMLS_GetCap {
			options_in : "".to_string(),
		}
	}
}

impl crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_cap_response::WMLS_GetCapResponse {
	/* Protocol 2100, MessageType : 8 */
	pub fn default_with_params(_result :i32,)
	-> crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_cap_response::WMLS_GetCapResponse {
		crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_cap_response::WMLS_GetCapResponse {
			result : _result,
			capabilities_out : "".to_string(),
			supp_msg_out : "".to_string(),
		}
	}
}

impl Default for crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_from_store::WMLS_GetFromStore {
	/* Protocol 2100, MessageType : 9 */
	fn default()
	-> crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_from_store::WMLS_GetFromStore {
		crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_from_store::WMLS_GetFromStore {
			wmltype_in : "".to_string(),
			xmlin : "".to_string(),
			options_in : "".to_string(),
			capabilities_in : "".to_string(),
		}
	}
}

impl crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_from_store_response::WMLS_GetFromStoreResponse {
	/* Protocol 2100, MessageType : 10 */
	pub fn default_with_params(_result :i32,)
	-> crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_from_store_response::WMLS_GetFromStoreResponse {
		crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_from_store_response::WMLS_GetFromStoreResponse {
			result : _result,
			xmlout : "".to_string(),
			supp_msg_out : "".to_string(),
		}
	}
}

impl Default for crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_version::WMLS_GetVersion {
	/* Protocol 2100, MessageType : 11 */
	fn default()
	-> crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_version::WMLS_GetVersion {
		crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_version::WMLS_GetVersion {
		}
	}
}

impl Default for crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_version_response::WMLS_GetVersionResponse {
	/* Protocol 2100, MessageType : 12 */
	fn default()
	-> crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_version_response::WMLS_GetVersionResponse {
		crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_version_response::WMLS_GetVersionResponse {
			result : "".to_string(),
		}
	}
}

impl Default for crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_update_in_store::WMLS_UpdateInStore {
	/* Protocol 2100, MessageType : 13 */
	fn default()
	-> crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_update_in_store::WMLS_UpdateInStore {
		crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_update_in_store::WMLS_UpdateInStore {
			wmltype_in : "".to_string(),
			xmlin : "".to_string(),
			options_in : "".to_string(),
			capabilities_in : "".to_string(),
		}
	}
}

impl crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_update_in_store_response::WMLS_UpdateInStoreResponse {
	/* Protocol 2100, MessageType : 14 */
	pub fn default_with_params(_result :i32,)
	-> crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_update_in_store_response::WMLS_UpdateInStoreResponse {
		crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_update_in_store_response::WMLS_UpdateInStoreResponse {
			result : _result,
			supp_msg_out : "".to_string(),
		}
	}
}

pub fn get_message_schema(protocol: i32, message_type: i32) -> &'static str{
	match (protocol, message_type) {
		(0, 1) => crate::energistics::etp::v12::protocol::core::request_session::AVRO_SCHEMA,
		(0, 2) => crate::energistics::etp::v12::protocol::core::open_session::AVRO_SCHEMA,
		(0, 5) => crate::energistics::etp::v12::protocol::core::close_session::AVRO_SCHEMA,
		(0, 6) => crate::energistics::etp::v12::protocol::core::authorize::AVRO_SCHEMA,
		(0, 7) => crate::energistics::etp::v12::protocol::core::authorize_response::AVRO_SCHEMA,
		(0, 8) => crate::energistics::etp::v12::protocol::core::ping::AVRO_SCHEMA,
		(0, 9) => crate::energistics::etp::v12::protocol::core::pong::AVRO_SCHEMA,
		(0, 1000) => crate::energistics::etp::v12::protocol::core::protocol_exception::AVRO_SCHEMA,
		(0, 1001) => crate::energistics::etp::v12::protocol::core::acknowledge::AVRO_SCHEMA,
		(1, 1) => crate::energistics::etp::v12::protocol::channel_streaming::channel_metadata::AVRO_SCHEMA,
		(1, 2) => crate::energistics::etp::v12::protocol::channel_streaming::channel_data::AVRO_SCHEMA,
		(1, 3) => crate::energistics::etp::v12::protocol::channel_streaming::start_streaming::AVRO_SCHEMA,
		(1, 4) => crate::energistics::etp::v12::protocol::channel_streaming::stop_streaming::AVRO_SCHEMA,
		(1, 5) => crate::energistics::etp::v12::protocol::channel_streaming::truncate_channels::AVRO_SCHEMA,
		(2, 1) => crate::energistics::etp::v12::protocol::channel_data_frame::get_frame_metadata::AVRO_SCHEMA,
		(2, 2) => crate::energistics::etp::v12::protocol::channel_data_frame::get_frame_metadata_response::AVRO_SCHEMA,
		(2, 3) => crate::energistics::etp::v12::protocol::channel_data_frame::get_frame::AVRO_SCHEMA,
		(2, 4) => crate::energistics::etp::v12::protocol::channel_data_frame::get_frame_response_header::AVRO_SCHEMA,
		(2, 5) => crate::energistics::etp::v12::protocol::channel_data_frame::cancel_get_frame::AVRO_SCHEMA,
		(2, 6) => crate::energistics::etp::v12::protocol::channel_data_frame::get_frame_response_rows::AVRO_SCHEMA,
		(3, 1) => crate::energistics::etp::v12::protocol::discovery::get_resources::AVRO_SCHEMA,
		(3, 4) => crate::energistics::etp::v12::protocol::discovery::get_resources_response::AVRO_SCHEMA,
		(3, 5) => crate::energistics::etp::v12::protocol::discovery::get_deleted_resources::AVRO_SCHEMA,
		(3, 6) => crate::energistics::etp::v12::protocol::discovery::get_deleted_resources_response::AVRO_SCHEMA,
		(3, 7) => crate::energistics::etp::v12::protocol::discovery::get_resources_edges_response::AVRO_SCHEMA,
		(4, 1) => crate::energistics::etp::v12::protocol::store::get_data_objects::AVRO_SCHEMA,
		(4, 2) => crate::energistics::etp::v12::protocol::store::put_data_objects::AVRO_SCHEMA,
		(4, 3) => crate::energistics::etp::v12::protocol::store::delete_data_objects::AVRO_SCHEMA,
		(4, 4) => crate::energistics::etp::v12::protocol::store::get_data_objects_response::AVRO_SCHEMA,
		(4, 8) => crate::energistics::etp::v12::protocol::store::chunk::AVRO_SCHEMA,
		(4, 9) => crate::energistics::etp::v12::protocol::store::put_data_objects_response::AVRO_SCHEMA,
		(4, 10) => crate::energistics::etp::v12::protocol::store::delete_data_objects_response::AVRO_SCHEMA,
		(5, 2) => crate::energistics::etp::v12::protocol::store_notification::object_changed::AVRO_SCHEMA,
		(5, 3) => crate::energistics::etp::v12::protocol::store_notification::object_deleted::AVRO_SCHEMA,
		(5, 4) => crate::energistics::etp::v12::protocol::store_notification::unsubscribe_notifications::AVRO_SCHEMA,
		(5, 5) => crate::energistics::etp::v12::protocol::store_notification::object_access_revoked::AVRO_SCHEMA,
		(5, 6) => crate::energistics::etp::v12::protocol::store_notification::subscribe_notifications::AVRO_SCHEMA,
		(5, 7) => crate::energistics::etp::v12::protocol::store_notification::subscription_ended::AVRO_SCHEMA,
		(5, 8) => crate::energistics::etp::v12::protocol::store_notification::unsolicited_store_notifications::AVRO_SCHEMA,
		(5, 9) => crate::energistics::etp::v12::protocol::store_notification::chunk::AVRO_SCHEMA,
		(5, 10) => crate::energistics::etp::v12::protocol::store_notification::subscribe_notifications_response::AVRO_SCHEMA,
		(5, 11) => crate::energistics::etp::v12::protocol::store_notification::object_active_status_changed::AVRO_SCHEMA,
		(6, 1) => crate::energistics::etp::v12::protocol::growing_object::delete_parts::AVRO_SCHEMA,
		(6, 3) => crate::energistics::etp::v12::protocol::growing_object::get_parts::AVRO_SCHEMA,
		(6, 4) => crate::energistics::etp::v12::protocol::growing_object::get_parts_by_range::AVRO_SCHEMA,
		(6, 5) => crate::energistics::etp::v12::protocol::growing_object::put_parts::AVRO_SCHEMA,
		(6, 6) => crate::energistics::etp::v12::protocol::growing_object::get_parts_response::AVRO_SCHEMA,
		(6, 7) => crate::energistics::etp::v12::protocol::growing_object::replace_parts_by_range::AVRO_SCHEMA,
		(6, 8) => crate::energistics::etp::v12::protocol::growing_object::get_parts_metadata::AVRO_SCHEMA,
		(6, 9) => crate::energistics::etp::v12::protocol::growing_object::get_parts_metadata_response::AVRO_SCHEMA,
		(6, 10) => crate::energistics::etp::v12::protocol::growing_object::get_parts_by_range_response::AVRO_SCHEMA,
		(6, 11) => crate::energistics::etp::v12::protocol::growing_object::delete_parts_response::AVRO_SCHEMA,
		(6, 13) => crate::energistics::etp::v12::protocol::growing_object::put_parts_response::AVRO_SCHEMA,
		(6, 14) => crate::energistics::etp::v12::protocol::growing_object::get_growing_data_objects_header::AVRO_SCHEMA,
		(6, 15) => crate::energistics::etp::v12::protocol::growing_object::get_growing_data_objects_header_response::AVRO_SCHEMA,
		(6, 16) => crate::energistics::etp::v12::protocol::growing_object::put_growing_data_objects_header::AVRO_SCHEMA,
		(6, 17) => crate::energistics::etp::v12::protocol::growing_object::put_growing_data_objects_header_response::AVRO_SCHEMA,
		(6, 18) => crate::energistics::etp::v12::protocol::growing_object::replace_parts_by_range_response::AVRO_SCHEMA,
		(6, 19) => crate::energistics::etp::v12::protocol::growing_object::get_change_annotations::AVRO_SCHEMA,
		(6, 20) => crate::energistics::etp::v12::protocol::growing_object::get_change_annotations_response::AVRO_SCHEMA,
		(7, 2) => crate::energistics::etp::v12::protocol::growing_object_notification::parts_changed::AVRO_SCHEMA,
		(7, 3) => crate::energistics::etp::v12::protocol::growing_object_notification::parts_deleted::AVRO_SCHEMA,
		(7, 4) => crate::energistics::etp::v12::protocol::growing_object_notification::unsubscribe_part_notification::AVRO_SCHEMA,
		(7, 6) => crate::energistics::etp::v12::protocol::growing_object_notification::parts_replaced_by_range::AVRO_SCHEMA,
		(7, 7) => crate::energistics::etp::v12::protocol::growing_object_notification::subscribe_part_notifications::AVRO_SCHEMA,
		(7, 8) => crate::energistics::etp::v12::protocol::growing_object_notification::part_subscription_ended::AVRO_SCHEMA,
		(7, 9) => crate::energistics::etp::v12::protocol::growing_object_notification::unsolicited_part_notifications::AVRO_SCHEMA,
		(7, 10) => crate::energistics::etp::v12::protocol::growing_object_notification::subscribe_part_notifications_response::AVRO_SCHEMA,
		(9, 1) => crate::energistics::etp::v12::protocol::data_array::get_data_arrays_response::AVRO_SCHEMA,
		(9, 2) => crate::energistics::etp::v12::protocol::data_array::get_data_arrays::AVRO_SCHEMA,
		(9, 3) => crate::energistics::etp::v12::protocol::data_array::get_data_subarrays::AVRO_SCHEMA,
		(9, 4) => crate::energistics::etp::v12::protocol::data_array::put_data_arrays::AVRO_SCHEMA,
		(9, 5) => crate::energistics::etp::v12::protocol::data_array::put_data_subarrays::AVRO_SCHEMA,
		(9, 6) => crate::energistics::etp::v12::protocol::data_array::get_data_array_metadata::AVRO_SCHEMA,
		(9, 7) => crate::energistics::etp::v12::protocol::data_array::get_data_array_metadata_response::AVRO_SCHEMA,
		(9, 8) => crate::energistics::etp::v12::protocol::data_array::get_data_subarrays_response::AVRO_SCHEMA,
		(9, 9) => crate::energistics::etp::v12::protocol::data_array::put_uninitialized_data_arrays::AVRO_SCHEMA,
		(9, 10) => crate::energistics::etp::v12::protocol::data_array::put_data_arrays_response::AVRO_SCHEMA,
		(9, 11) => crate::energistics::etp::v12::protocol::data_array::put_data_subarrays_response::AVRO_SCHEMA,
		(9, 12) => crate::energistics::etp::v12::protocol::data_array::put_uninitialized_data_arrays_response::AVRO_SCHEMA,
		(13, 1) => crate::energistics::etp::v12::protocol::discovery_query::find_resources::AVRO_SCHEMA,
		(13, 2) => crate::energistics::etp::v12::protocol::discovery_query::find_resources_response::AVRO_SCHEMA,
		(14, 1) => crate::energistics::etp::v12::protocol::store_query::find_data_objects::AVRO_SCHEMA,
		(14, 2) => crate::energistics::etp::v12::protocol::store_query::find_data_objects_response::AVRO_SCHEMA,
		(14, 3) => crate::energistics::etp::v12::protocol::store_query::chunk::AVRO_SCHEMA,
		(16, 1) => crate::energistics::etp::v12::protocol::growing_object_query::find_parts::AVRO_SCHEMA,
		(16, 2) => crate::energistics::etp::v12::protocol::growing_object_query::find_parts_response::AVRO_SCHEMA,
		(18, 1) => crate::energistics::etp::v12::protocol::transaction::start_transaction::AVRO_SCHEMA,
		(18, 2) => crate::energistics::etp::v12::protocol::transaction::start_transaction_response::AVRO_SCHEMA,
		(18, 3) => crate::energistics::etp::v12::protocol::transaction::commit_transaction::AVRO_SCHEMA,
		(18, 4) => crate::energistics::etp::v12::protocol::transaction::rollback_transaction::AVRO_SCHEMA,
		(18, 5) => crate::energistics::etp::v12::protocol::transaction::commit_transaction_response::AVRO_SCHEMA,
		(18, 6) => crate::energistics::etp::v12::protocol::transaction::rollback_transaction_response::AVRO_SCHEMA,
		(21, 1) => crate::energistics::etp::v12::protocol::channel_subscribe::get_channel_metadata::AVRO_SCHEMA,
		(21, 2) => crate::energistics::etp::v12::protocol::channel_subscribe::get_channel_metadata_response::AVRO_SCHEMA,
		(21, 3) => crate::energistics::etp::v12::protocol::channel_subscribe::subscribe_channels::AVRO_SCHEMA,
		(21, 4) => crate::energistics::etp::v12::protocol::channel_subscribe::channel_data::AVRO_SCHEMA,
		(21, 6) => crate::energistics::etp::v12::protocol::channel_subscribe::range_replaced::AVRO_SCHEMA,
		(21, 7) => crate::energistics::etp::v12::protocol::channel_subscribe::unsubscribe_channels::AVRO_SCHEMA,
		(21, 8) => crate::energistics::etp::v12::protocol::channel_subscribe::subscriptions_stopped::AVRO_SCHEMA,
		(21, 9) => crate::energistics::etp::v12::protocol::channel_subscribe::get_ranges::AVRO_SCHEMA,
		(21, 10) => crate::energistics::etp::v12::protocol::channel_subscribe::get_ranges_response::AVRO_SCHEMA,
		(21, 11) => crate::energistics::etp::v12::protocol::channel_subscribe::cancel_get_ranges::AVRO_SCHEMA,
		(21, 12) => crate::energistics::etp::v12::protocol::channel_subscribe::subscribe_channels_response::AVRO_SCHEMA,
		(21, 13) => crate::energistics::etp::v12::protocol::channel_subscribe::channels_truncated::AVRO_SCHEMA,
		(21, 14) => crate::energistics::etp::v12::protocol::channel_subscribe::get_change_annotations::AVRO_SCHEMA,
		(21, 15) => crate::energistics::etp::v12::protocol::channel_subscribe::get_change_annotations_response::AVRO_SCHEMA,
		(22, 1) => crate::energistics::etp::v12::protocol::channel_data_load::open_channels::AVRO_SCHEMA,
		(22, 2) => crate::energistics::etp::v12::protocol::channel_data_load::open_channels_response::AVRO_SCHEMA,
		(22, 3) => crate::energistics::etp::v12::protocol::channel_data_load::close_channels::AVRO_SCHEMA,
		(22, 4) => crate::energistics::etp::v12::protocol::channel_data_load::channel_data::AVRO_SCHEMA,
		(22, 6) => crate::energistics::etp::v12::protocol::channel_data_load::replace_range::AVRO_SCHEMA,
		(22, 7) => crate::energistics::etp::v12::protocol::channel_data_load::channels_closed::AVRO_SCHEMA,
		(22, 8) => crate::energistics::etp::v12::protocol::channel_data_load::replace_range_response::AVRO_SCHEMA,
		(22, 9) => crate::energistics::etp::v12::protocol::channel_data_load::truncate_channels::AVRO_SCHEMA,
		(22, 10) => crate::energistics::etp::v12::protocol::channel_data_load::truncate_channels_response::AVRO_SCHEMA,
		(24, 1) => crate::energistics::etp::v12::protocol::dataspace::get_dataspaces::AVRO_SCHEMA,
		(24, 2) => crate::energistics::etp::v12::protocol::dataspace::get_dataspaces_response::AVRO_SCHEMA,
		(24, 3) => crate::energistics::etp::v12::protocol::dataspace::put_dataspaces::AVRO_SCHEMA,
		(24, 4) => crate::energistics::etp::v12::protocol::dataspace::delete_dataspaces::AVRO_SCHEMA,
		(24, 5) => crate::energistics::etp::v12::protocol::dataspace::delete_dataspaces_response::AVRO_SCHEMA,
		(24, 6) => crate::energistics::etp::v12::protocol::dataspace::put_dataspaces_response::AVRO_SCHEMA,
		(25, 1) => crate::energistics::etp::v12::protocol::supported_types::get_supported_types::AVRO_SCHEMA,
		(25, 2) => crate::energistics::etp::v12::protocol::supported_types::get_supported_types_response::AVRO_SCHEMA,
		(2100, 1) => crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_add_to_store::AVRO_SCHEMA,
		(2100, 2) => crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_add_to_store_response::AVRO_SCHEMA,
		(2100, 3) => crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_delete_from_store::AVRO_SCHEMA,
		(2100, 4) => crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_delete_from_store_response::AVRO_SCHEMA,
		(2100, 5) => crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_base_msg::AVRO_SCHEMA,
		(2100, 6) => crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_base_msg_response::AVRO_SCHEMA,
		(2100, 7) => crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_cap::AVRO_SCHEMA,
		(2100, 8) => crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_cap_response::AVRO_SCHEMA,
		(2100, 9) => crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_from_store::AVRO_SCHEMA,
		(2100, 10) => crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_from_store_response::AVRO_SCHEMA,
		(2100, 11) => crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_version::AVRO_SCHEMA,
		(2100, 12) => crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_get_version_response::AVRO_SCHEMA,
		(2100, 13) => crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_update_in_store::AVRO_SCHEMA,
		(2100, 14) => crate::energistics::etp::v12::private_protocols::witsml_soap::wmls_update_in_store_response::AVRO_SCHEMA,
		_ => "",
	}
}