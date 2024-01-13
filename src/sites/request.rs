// GENERATED CODE

use crate::api_default_imports::*;
use crate::default_drive::*;
use crate::users::*;
use crate::sites::*;
use crate::drives::*;

resource_api_client!(SitesApiClient, SitesIdApiClient, ResourceIdentity::Sites);

impl SitesApiClient {
	get!(
		doc: "Search for sites", 
		name: list_site,
		path: "/sites"
	);
	get!(
		doc: "Get the number of the resource", 
		name: sites_get_count_6254,
		path: "/sites/$count"
	);
	post!(
		doc: "Invoke action add", 
		name: add,
		path: "/sites/add",
		body: true
	);
	get!(
		doc: "Invoke function delta", 
		name: delta,
		path: "/sites/delta()"
	);
	get!(
		doc: "Invoke function getAllSites", 
		name: get_all_sites,
		path: "/sites/getAllSites()"
	);
	post!(
		doc: "Invoke action remove", 
		name: remove,
		path: "/sites/remove",
		body: true
	);
}

impl SitesIdApiClient {api_client_link!(term_store, TermStoreApiClient);
api_client_link_id!(content_type, SitesContentTypesIdApiClient);
api_client_link!(drive, DefaultDriveApiClient);
api_client_link!(lists, SitesListsApiClient);
api_client_link!(term_stores, TermStoresApiClient);
api_client_link_id!(term_stores_id, TermStoresIdApiClient);
api_client_link!(content_types, SitesContentTypesApiClient);
api_client_link_id!(list, SitesListsIdApiClient);
api_client_link!(items, SitesItemsApiClient);
api_client_link!(onenote, OnenoteApiClient);

	get!(
		doc: "Get a site resource", 
		name: get_site,
		path: "/sites/{{RID}}"
	);
	patch!(
		doc: "Update entity in sites", 
		name: update_site,
		path: "/sites/{{RID}}",
		body: true
	);
	delete!(
		doc: "Delete navigation property analytics for sites", 
		name: delete_analytics,
		path: "/sites/{{RID}}/analytics"
	);
	get!(
		doc: "Get analytics from sites", 
		name: get_analytics,
		path: "/sites/{{RID}}/analytics"
	);
	patch!(
		doc: "Update the navigation property analytics in sites", 
		name: update_analytics,
		path: "/sites/{{RID}}/analytics",
		body: true
	);
	get!(
		doc: "Get itemAnalytics", 
		name: get_all_time,
		path: "/sites/{{RID}}/analytics/allTime"
	);
	post!(
		doc: "Create new navigation property to itemActivityStats for sites", 
		name: create_item_activity_stats,
		path: "/sites/{{RID}}/analytics/itemActivityStats",
		body: true
	);
	get!(
		doc: "Get itemActivityStats from sites", 
		name: list_item_activity_stats,
		path: "/sites/{{RID}}/analytics/itemActivityStats"
	);
	get!(
		doc: "Get the number of the resource", 
		name: item_activity_stats,
		path: "/sites/{{RID}}/analytics/itemActivityStats/$count"
	);
	delete!(
		doc: "Delete navigation property itemActivityStats for sites", 
		name: delete_item_activity_stats,
		path: "/sites/{{RID}}/analytics/itemActivityStats/{{id}}",
		params: item_activity_stat_id
	);
	get!(
		doc: "Get itemActivityStats from sites", 
		name: get_item_activity_stats,
		path: "/sites/{{RID}}/analytics/itemActivityStats/{{id}}",
		params: item_activity_stat_id
	);
	patch!(
		doc: "Update the navigation property itemActivityStats in sites", 
		name: update_item_activity_stats,
		path: "/sites/{{RID}}/analytics/itemActivityStats/{{id}}",
		body: true,
		params: item_activity_stat_id
	);
	post!(
		doc: "Create new navigation property to activities for sites", 
		name: create_activities,
		path: "/sites/{{RID}}/analytics/itemActivityStats/{{id}}/activities",
		body: true,
		params: item_activity_stat_id
	);
	get!(
		doc: "Get activities from sites", 
		name: list_activities,
		path: "/sites/{{RID}}/analytics/itemActivityStats/{{id}}/activities",
		params: item_activity_stat_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: activities,
		path: "/sites/{{RID}}/analytics/itemActivityStats/{{id}}/activities/$count",
		params: item_activity_stat_id
	);
	delete!(
		doc: "Delete navigation property activities for sites", 
		name: delete_activities,
		path: "/sites/{{RID}}/analytics/itemActivityStats/{{id}}/activities/{{id2}}",
		params: item_activity_stat_id, item_activity_id
	);
	get!(
		doc: "Get activities from sites", 
		name: get_activities,
		path: "/sites/{{RID}}/analytics/itemActivityStats/{{id}}/activities/{{id2}}",
		params: item_activity_stat_id, item_activity_id
	);
	patch!(
		doc: "Update the navigation property activities in sites", 
		name: update_activities,
		path: "/sites/{{RID}}/analytics/itemActivityStats/{{id}}/activities/{{id2}}",
		body: true,
		params: item_activity_stat_id, item_activity_id
	);
	get!(
		doc: "Get driveItem from sites", 
		name: get_drive_item,
		path: "/sites/{{RID}}/analytics/itemActivityStats/{{id}}/activities/{{id2}}/driveItem",
		params: item_activity_stat_id, item_activity_id
	);
	get!(
		doc: "Get content for the navigation property driveItem from sites", 
		name: get_drive_item_content,
		path: "/sites/{{RID}}/analytics/itemActivityStats/{{id}}/activities/{{id2}}/driveItem/content",
		params: item_activity_stat_id, item_activity_id
	);
	put!(
		doc: "Update content for the navigation property driveItem in sites", 
		name: update_drive_item_content,
		path: "/sites/{{RID}}/analytics/itemActivityStats/{{id}}/activities/{{id2}}/driveItem/content",
		body: true,
		params: item_activity_stat_id, item_activity_id
	);
	get!(
		doc: "Get lastSevenDays from sites", 
		name: get_last_seven_days,
		path: "/sites/{{RID}}/analytics/lastSevenDays"
	);
	post!(
		doc: "Create a columnDefinition in a site", 
		name: create_columns,
		path: "/sites/{{RID}}/columns",
		body: true
	);
	get!(
		doc: "List columns in a site", 
		name: list_columns,
		path: "/sites/{{RID}}/columns"
	);
	get!(
		doc: "Get the number of the resource", 
		name: columns,
		path: "/sites/{{RID}}/columns/$count"
	);
	delete!(
		doc: "Delete navigation property columns for sites", 
		name: delete_columns,
		path: "/sites/{{RID}}/columns/{{id}}",
		params: column_definition_id
	);
	get!(
		doc: "Get columns from sites", 
		name: get_columns,
		path: "/sites/{{RID}}/columns/{{id}}",
		params: column_definition_id
	);
	patch!(
		doc: "Update the navigation property columns in sites", 
		name: update_columns,
		path: "/sites/{{RID}}/columns/{{id}}",
		body: true,
		params: column_definition_id
	);
	get!(
		doc: "Get sourceColumn from sites", 
		name: get_source_column,
		path: "/sites/{{RID}}/columns/{{id}}/sourceColumn",
		params: column_definition_id
	);
	get!(
		doc: "Get createdByUser from sites", 
		name: get_created_by_user,
		path: "/sites/{{RID}}/createdByUser"
	);
	get!(
		doc: "Get mailboxSettings property value", 
		name: get_mailbox_settings,
		path: "/sites/{{RID}}/createdByUser/mailboxSettings"
	);
	patch!(
		doc: "Update property mailboxSettings value.", 
		name: update_mailbox_settings,
		path: "/sites/{{RID}}/createdByUser/mailboxSettings",
		body: true
	);
	get!(
		doc: "Get serviceProvisioningErrors property value", 
		name: list_service_provisioning_errors,
		path: "/sites/{{RID}}/createdByUser/serviceProvisioningErrors"
	);
	get!(
		doc: "Get the number of the resource", 
		name: service_provisioning_errors,
		path: "/sites/{{RID}}/createdByUser/serviceProvisioningErrors/$count"
	);
	get!(
		doc: "Get drive from sites", 
		name: get_drive,
		path: "/sites/{{RID}}/drive"
	);
	get!(
		doc: "Get drives from sites", 
		name: list_drives,
		path: "/sites/{{RID}}/drives"
	);
	get!(
		doc: "Get the number of the resource", 
		name: drives,
		path: "/sites/{{RID}}/drives/$count"
	);
	get!(
		doc: "Get drives from sites", 
		name: get_drives,
		path: "/sites/{{RID}}/drives/{{id}}",
		params: drive_id
	);
	get!(
		doc: "Get externalColumns from sites", 
		name: list_external_columns,
		path: "/sites/{{RID}}/externalColumns"
	);
	get!(
		doc: "Get the number of the resource", 
		name: external_columns,
		path: "/sites/{{RID}}/externalColumns/$count"
	);
	get!(
		doc: "Get externalColumns from sites", 
		name: get_external_columns,
		path: "/sites/{{RID}}/externalColumns/{{id}}",
		params: column_definition_id
	);
	get!(
		doc: "Invoke function getActivitiesByInterval", 
		name: site,
		path: "/sites/{{RID}}/getActivitiesByInterval(startDateTime='{{id}}',endDateTime='{{id2}}',interval='{{id3}}')",
		params: start_date_time, end_date_time, interval
	);
	get!(
		doc: "Invoke function getApplicableContentTypesForList", 
		name: get_applicable_content_types_for_list,
		path: "/sites/{{RID}}/getApplicableContentTypesForList(listId='{{id}}')",
		params: list_id
	);
	get!(
		doc: "Invoke function getByPath", 
		name: get_by_path,
		path: "/sites/{{RID}}/getByPath(path='{{id}}')",
		params: path
	);
	get!(
		doc: "Get items from sites", 
		name: list_items,
		path: "/sites/{{RID}}/items"
	);
	get!(
		doc: "Get the number of the resource", 
		name: items,
		path: "/sites/{{RID}}/items/$count"
	);
	get!(
		doc: "Get items from sites", 
		name: get_items,
		path: "/sites/{{RID}}/items/{{id}}",
		params: base_item_id
	);
	get!(
		doc: "Get lastModifiedByUser from sites", 
		name: get_last_modified_by_user,
		path: "/sites/{{RID}}/lastModifiedByUser"
	);
	get!(
		doc: "Get mailboxSettings property value", 
		name: get_mailbox_settings,
		path: "/sites/{{RID}}/lastModifiedByUser/mailboxSettings"
	);
	patch!(
		doc: "Update property mailboxSettings value.", 
		name: update_mailbox_settings,
		path: "/sites/{{RID}}/lastModifiedByUser/mailboxSettings",
		body: true
	);
	get!(
		doc: "Get serviceProvisioningErrors property value", 
		name: list_service_provisioning_errors,
		path: "/sites/{{RID}}/lastModifiedByUser/serviceProvisioningErrors"
	);
	get!(
		doc: "Get the number of the resource", 
		name: service_provisioning_errors,
		path: "/sites/{{RID}}/lastModifiedByUser/serviceProvisioningErrors/$count"
	);
	post!(
		doc: "Create new navigation property to operations for sites", 
		name: create_operations,
		path: "/sites/{{RID}}/operations",
		body: true
	);
	get!(
		doc: "List operations on a site", 
		name: list_operations,
		path: "/sites/{{RID}}/operations"
	);
	get!(
		doc: "Get the number of the resource", 
		name: operations,
		path: "/sites/{{RID}}/operations/$count"
	);
	delete!(
		doc: "Delete navigation property operations for sites", 
		name: delete_operations,
		path: "/sites/{{RID}}/operations/{{id}}",
		params: rich_long_running_operation_id
	);
	get!(
		doc: "Get richLongRunningOperation", 
		name: get_operations,
		path: "/sites/{{RID}}/operations/{{id}}",
		params: rich_long_running_operation_id
	);
	patch!(
		doc: "Update the navigation property operations in sites", 
		name: update_operations,
		path: "/sites/{{RID}}/operations/{{id}}",
		body: true,
		params: rich_long_running_operation_id
	);
	post!(
		doc: "Create permission", 
		name: create_permissions,
		path: "/sites/{{RID}}/permissions",
		body: true
	);
	get!(
		doc: "List permissions", 
		name: list_permissions,
		path: "/sites/{{RID}}/permissions"
	);
	get!(
		doc: "Get the number of the resource", 
		name: permissions,
		path: "/sites/{{RID}}/permissions/$count"
	);
	delete!(
		doc: "Delete permission", 
		name: delete_permissions,
		path: "/sites/{{RID}}/permissions/{{id}}",
		params: permission_id
	);
	get!(
		doc: "Get permission", 
		name: get_permissions,
		path: "/sites/{{RID}}/permissions/{{id}}",
		params: permission_id
	);
	patch!(
		doc: "Update permission", 
		name: update_permissions,
		path: "/sites/{{RID}}/permissions/{{id}}",
		body: true,
		params: permission_id
	);
	post!(
		doc: "Invoke action grant", 
		name: grant,
		path: "/sites/{{RID}}/permissions/{{id}}/grant",
		body: true,
		params: permission_id
	);
	get!(
		doc: "List subsites for a site", 
		name: list_sites,
		path: "/sites/{{RID}}/sites"
	);
	get!(
		doc: "Get the number of the resource", 
		name: sites,
		path: "/sites/{{RID}}/sites/$count"
	);
	get!(
		doc: "Get sites from sites", 
		name: get_sites,
		path: "/sites/{{RID}}/sites/{{id}}",
		params: site_id_1
	);
}
