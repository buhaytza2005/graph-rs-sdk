// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(DriveApiClient, ResourceIdentity::Drive);

impl DriveApiClient {
	post!(
		doc: "Add new entity to drives", 
		name: create_drive,
		path: "/drives",
		body: true
	);
	get!(
		doc: "Get entities from drives", 
		name: list_drive,
		path: "/drives"
	);
	delete!(
		doc: "Delete entity from drives", 
		name: delete_drive,
		path: "/drives/{{id}}",
		params: drive_id
	);
	get!(
		doc: "Get entity from drives by key", 
		name: get_drive,
		path: "/drives/{{id}}",
		params: drive_id
	);
	patch!(
		doc: "Update entity in drives", 
		name: update_drive,
		path: "/drives/{{id}}",
		body: true,
		params: drive_id
	);
	post!(
		doc: "Create new navigation property to bundles for drives", 
		name: create_bundles,
		path: "/drives/{{id}}/bundles",
		body: true,
		params: drive_id
	);
	get!(
		doc: "Get bundles from drives", 
		name: list_bundles,
		path: "/drives/{{id}}/bundles",
		params: drive_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: bundles,
		path: "/drives/{{id}}/bundles/$count",
		params: drive_id
	);
	get!(
		doc: "Get bundles from drives", 
		name: get_bundles,
		path: "/drives/{{id}}/bundles/{{id2}}",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get content for the navigation property bundles from drives", 
		name: get_bundles_content,
		path: "/drives/{{id}}/bundles/{{id2}}/content",
		params: drive_id, drive_item_id
	);
	put!(
		doc: "Update content for the navigation property bundles in drives", 
		name: update_bundles_content,
		path: "/drives/{{id}}/bundles/{{id2}}/content",
		body: true,
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get createdByUser from drives", 
		name: get_created_by_user,
		path: "/drives/{{id}}/createdByUser",
		params: drive_id
	);
	get!(
		doc: "Get mailboxSettings property value", 
		name: get_mailbox_settings,
		path: "/drives/{{id}}/createdByUser/mailboxSettings",
		params: drive_id
	);
	patch!(
		doc: "Update property mailboxSettings value.", 
		name: update_mailbox_settings,
		path: "/drives/{{id}}/createdByUser/mailboxSettings",
		body: true,
		params: drive_id
	);
	get!(
		doc: "Get serviceProvisioningErrors property value", 
		name: list_service_provisioning_errors,
		path: "/drives/{{id}}/createdByUser/serviceProvisioningErrors",
		params: drive_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: service_provisioning_errors,
		path: "/drives/{{id}}/createdByUser/serviceProvisioningErrors/$count",
		params: drive_id
	);
	get!(
		doc: "List followed items", 
		name: list_following,
		path: "/drives/{{id}}/following",
		params: drive_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: following,
		path: "/drives/{{id}}/following/$count",
		params: drive_id
	);
	get!(
		doc: "Get following from drives", 
		name: get_following,
		path: "/drives/{{id}}/following/{{id2}}",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get content for the navigation property following from drives", 
		name: get_following_content,
		path: "/drives/{{id}}/following/{{id2}}/content",
		params: drive_id, drive_item_id
	);
	put!(
		doc: "Update content for the navigation property following in drives", 
		name: update_following_content,
		path: "/drives/{{id}}/following/{{id2}}/content",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Create new navigation property to items for drives", 
		name: create_items,
		path: "/drives/{{id}}/items",
		body: true,
		params: drive_id
	);
	get!(
		doc: "Get items from drives", 
		name: list_items,
		path: "/drives/{{id}}/items",
		params: drive_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: items,
		path: "/drives/{{id}}/items/$count",
		params: drive_id
	);
	delete!(
		doc: "Delete a DriveItem", 
		name: delete_items,
		path: "/drives/{{id}}/items/{{id2}}",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get items from drives", 
		name: get_items,
		path: "/drives/{{id}}/items/{{id2}}",
		params: drive_id, drive_item_id
	);
	patch!(
		doc: "Move a DriveItem to a new folder", 
		name: update_items,
		path: "/drives/{{id}}/items/{{id2}}",
		body: true,
		params: drive_id, drive_item_id
	);
	delete!(
		doc: "Delete navigation property analytics for drives", 
		name: delete_analytics,
		path: "/drives/{{id}}/items/{{id2}}/analytics",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get analytics from drives", 
		name: get_analytics,
		path: "/drives/{{id}}/items/{{id2}}/analytics",
		params: drive_id, drive_item_id
	);
	patch!(
		doc: "Update the navigation property analytics in drives", 
		name: update_analytics,
		path: "/drives/{{id}}/items/{{id2}}/analytics",
		body: true,
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get itemAnalytics", 
		name: get_all_time,
		path: "/drives/{{id}}/items/{{id2}}/analytics/allTime",
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Create new navigation property to itemActivityStats for drives", 
		name: create_item_activity_stats,
		path: "/drives/{{id}}/items/{{id2}}/analytics/itemActivityStats",
		body: true,
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get itemActivityStats from drives", 
		name: list_item_activity_stats,
		path: "/drives/{{id}}/items/{{id2}}/analytics/itemActivityStats",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: item_activity_stats,
		path: "/drives/{{id}}/items/{{id2}}/analytics/itemActivityStats/$count",
		params: drive_id, drive_item_id
	);
	delete!(
		doc: "Delete navigation property itemActivityStats for drives", 
		name: delete_item_activity_stats,
		path: "/drives/{{id}}/items/{{id2}}/analytics/itemActivityStats/{{id3}}",
		params: drive_id, drive_item_id, item_activity_stat_id
	);
	get!(
		doc: "Get itemActivityStats from drives", 
		name: get_item_activity_stats,
		path: "/drives/{{id}}/items/{{id2}}/analytics/itemActivityStats/{{id3}}",
		params: drive_id, drive_item_id, item_activity_stat_id
	);
	patch!(
		doc: "Update the navigation property itemActivityStats in drives", 
		name: update_item_activity_stats,
		path: "/drives/{{id}}/items/{{id2}}/analytics/itemActivityStats/{{id3}}",
		body: true,
		params: drive_id, drive_item_id, item_activity_stat_id
	);
	post!(
		doc: "Create new navigation property to activities for drives", 
		name: create_activities,
		path: "/drives/{{id}}/items/{{id2}}/analytics/itemActivityStats/{{id3}}/activities",
		body: true,
		params: drive_id, drive_item_id, item_activity_stat_id
	);
	get!(
		doc: "Get activities from drives", 
		name: list_activities,
		path: "/drives/{{id}}/items/{{id2}}/analytics/itemActivityStats/{{id3}}/activities",
		params: drive_id, drive_item_id, item_activity_stat_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: activities,
		path: "/drives/{{id}}/items/{{id2}}/analytics/itemActivityStats/{{id3}}/activities/$count",
		params: drive_id, drive_item_id, item_activity_stat_id
	);
	delete!(
		doc: "Delete navigation property activities for drives", 
		name: delete_activities,
		path: "/drives/{{id}}/items/{{id2}}/analytics/itemActivityStats/{{id3}}/activities/{{id4}}",
		params: drive_id, drive_item_id, item_activity_stat_id, item_activity_id
	);
	get!(
		doc: "Get activities from drives", 
		name: get_activities,
		path: "/drives/{{id}}/items/{{id2}}/analytics/itemActivityStats/{{id3}}/activities/{{id4}}",
		params: drive_id, drive_item_id, item_activity_stat_id, item_activity_id
	);
	patch!(
		doc: "Update the navigation property activities in drives", 
		name: update_activities,
		path: "/drives/{{id}}/items/{{id2}}/analytics/itemActivityStats/{{id3}}/activities/{{id4}}",
		body: true,
		params: drive_id, drive_item_id, item_activity_stat_id, item_activity_id
	);
	get!(
		doc: "Get driveItem from drives", 
		name: get_drive_item,
		path: "/drives/{{id}}/items/{{id2}}/analytics/itemActivityStats/{{id3}}/activities/{{id4}}/driveItem",
		params: drive_id, drive_item_id, item_activity_stat_id, item_activity_id
	);
	get!(
		doc: "Get content for the navigation property driveItem from drives", 
		name: get_drive_item_content,
		path: "/drives/{{id}}/items/{{id2}}/analytics/itemActivityStats/{{id3}}/activities/{{id4}}/driveItem/content",
		params: drive_id, drive_item_id, item_activity_stat_id, item_activity_id
	);
	put!(
		doc: "Update content for the navigation property driveItem in drives", 
		name: update_drive_item_content,
		path: "/drives/{{id}}/items/{{id2}}/analytics/itemActivityStats/{{id3}}/activities/{{id4}}/driveItem/content",
		body: true,
		params: drive_id, drive_item_id, item_activity_stat_id, item_activity_id
	);
	get!(
		doc: "Get lastSevenDays from drives", 
		name: get_last_seven_days,
		path: "/drives/{{id}}/items/{{id2}}/analytics/lastSevenDays",
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action assignSensitivityLabel", 
		name: assign_sensitivity_label,
		path: "/drives/{{id}}/items/{{id2}}/assignSensitivityLabel",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action checkin", 
		name: checkin,
		path: "/drives/{{id}}/items/{{id2}}/checkin",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action checkout", 
		name: checkout,
		path: "/drives/{{id}}/items/{{id2}}/checkout",
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Create new navigation property to children for drives", 
		name: create_children,
		path: "/drives/{{id}}/items/{{id2}}/children",
		body: true,
		params: drive_id, drive_item_id
	);
	get!(
		doc: "List children of a driveItem", 
		name: list_children,
		path: "/drives/{{id}}/items/{{id2}}/children",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: children,
		path: "/drives/{{id}}/items/{{id2}}/children/$count",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get children from drives", 
		name: get_children,
		path: "/drives/{{id}}/items/{{id2}}/children/{{id3}}",
		params: drive_id, drive_item_id, drive_item_id_1
	);
	get!(
		doc: "Get content for the navigation property children from drives", 
		name: get_children_content,
		path: "/drives/{{id}}/items/{{id2}}/children/{{id3}}/content",
		params: drive_id, drive_item_id, drive_item_id_1
	);
	put!(
		doc: "Update content for the navigation property children in drives", 
		name: update_children_content,
		path: "/drives/{{id}}/items/{{id2}}/children/{{id3}}/content",
		body: true,
		params: drive_id, drive_item_id, drive_item_id_1
	);
	get!(
		doc: "Get content for the navigation property items from drives", 
		name: get_items_content,
		path: "/drives/{{id}}/items/{{id2}}/content",
		params: drive_id, drive_item_id
	);
	put!(
		doc: "Update content for the navigation property items in drives", 
		name: update_items_content,
		path: "/drives/{{id}}/items/{{id2}}/content",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action copy", 
		name: copy,
		path: "/drives/{{id}}/items/{{id2}}/copy",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action createLink", 
		name: create_link,
		path: "/drives/{{id}}/items/{{id2}}/createLink",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action createUploadSession", 
		name: create_upload_session,
		path: "/drives/{{id}}/items/{{id2}}/createUploadSession",
		body: true,
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get createdByUser from drives", 
		name: get_created_by_user,
		path: "/drives/{{id}}/items/{{id2}}/createdByUser",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get mailboxSettings property value", 
		name: get_mailbox_settings,
		path: "/drives/{{id}}/items/{{id2}}/createdByUser/mailboxSettings",
		params: drive_id, drive_item_id
	);
	patch!(
		doc: "Update property mailboxSettings value.", 
		name: update_mailbox_settings,
		path: "/drives/{{id}}/items/{{id2}}/createdByUser/mailboxSettings",
		body: true,
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get serviceProvisioningErrors property value", 
		name: list_service_provisioning_errors,
		path: "/drives/{{id}}/items/{{id2}}/createdByUser/serviceProvisioningErrors",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: service_provisioning_errors,
		path: "/drives/{{id}}/items/{{id2}}/createdByUser/serviceProvisioningErrors/$count",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Invoke function delta", 
		name: drive_item,
		path: "/drives/{{id}}/items/{{id2}}/delta()",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Invoke function delta", 
		name: drive_item,
		path: "/drives/{{id}}/items/{{id2}}/delta(token='{{id3}}')",
		params: token
	);
	post!(
		doc: "Invoke action extractSensitivityLabels", 
		name: extract_sensitivity_labels,
		path: "/drives/{{id}}/items/{{id2}}/extractSensitivityLabels",
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action follow", 
		name: follow,
		path: "/drives/{{id}}/items/{{id2}}/follow",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Invoke function getActivitiesByInterval", 
		name: drive_item,
		path: "/drives/{{id}}/items/{{id2}}/getActivitiesByInterval()",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Invoke function getActivitiesByInterval", 
		name: drive_item,
		path: "/drives/{{id}}/items/{{id2}}/getActivitiesByInterval(startDateTime='{{id3}}',endDateTime='{{id4}}',interval='{{id5}}')",
		params: start_date_time, end_date_time, interval
	);
	post!(
		doc: "Invoke action invite", 
		name: invite,
		path: "/drives/{{id}}/items/{{id2}}/invite",
		body: true,
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get lastModifiedByUser from drives", 
		name: get_last_modified_by_user,
		path: "/drives/{{id}}/items/{{id2}}/lastModifiedByUser",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get mailboxSettings property value", 
		name: get_mailbox_settings,
		path: "/drives/{{id}}/items/{{id2}}/lastModifiedByUser/mailboxSettings",
		params: drive_id, drive_item_id
	);
	patch!(
		doc: "Update property mailboxSettings value.", 
		name: update_mailbox_settings,
		path: "/drives/{{id}}/items/{{id2}}/lastModifiedByUser/mailboxSettings",
		body: true,
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get serviceProvisioningErrors property value", 
		name: list_service_provisioning_errors,
		path: "/drives/{{id}}/items/{{id2}}/lastModifiedByUser/serviceProvisioningErrors",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: service_provisioning_errors,
		path: "/drives/{{id}}/items/{{id2}}/lastModifiedByUser/serviceProvisioningErrors/$count",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get listItem from drives", 
		name: get_list_item,
		path: "/drives/{{id}}/items/{{id2}}/listItem",
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action permanentDelete", 
		name: permanent_delete,
		path: "/drives/{{id}}/items/{{id2}}/permanentDelete",
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Create new navigation property to permissions for drives", 
		name: create_permissions,
		path: "/drives/{{id}}/items/{{id2}}/permissions",
		body: true,
		params: drive_id, drive_item_id
	);
	get!(
		doc: "List sharing permissions on a driveItem", 
		name: list_permissions,
		path: "/drives/{{id}}/items/{{id2}}/permissions",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: permissions,
		path: "/drives/{{id}}/items/{{id2}}/permissions/$count",
		params: drive_id, drive_item_id
	);
	delete!(
		doc: "Delete a sharing permission from a file or folder", 
		name: delete_permissions,
		path: "/drives/{{id}}/items/{{id2}}/permissions/{{id3}}",
		params: drive_id, drive_item_id, permission_id
	);
	get!(
		doc: "Get sharing permission for a file or folder", 
		name: get_permissions,
		path: "/drives/{{id}}/items/{{id2}}/permissions/{{id3}}",
		params: drive_id, drive_item_id, permission_id
	);
	patch!(
		doc: "Update sharing permission", 
		name: update_permissions,
		path: "/drives/{{id}}/items/{{id2}}/permissions/{{id3}}",
		body: true,
		params: drive_id, drive_item_id, permission_id
	);
	post!(
		doc: "Invoke action grant", 
		name: grant,
		path: "/drives/{{id}}/items/{{id2}}/permissions/{{id3}}/grant",
		body: true,
		params: drive_id, drive_item_id, permission_id
	);
	post!(
		doc: "Invoke action preview", 
		name: preview,
		path: "/drives/{{id}}/items/{{id2}}/preview",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action restore", 
		name: restore,
		path: "/drives/{{id}}/items/{{id2}}/restore",
		body: true,
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get retentionLabel from drives", 
		name: get_retention_label,
		path: "/drives/{{id}}/items/{{id2}}/retentionLabel",
		params: drive_id, drive_item_id
	);
	delete!(
		doc: "driveItem: removeRetentionLabel", 
		name: delete_retention_label,
		path: "/drives/{{id}}/items/{{id2}}/retentionLabel",
		params: drive_id, drive_item_id
	);
	patch!(
		doc: "driveItem: setRetentionLabel", 
		name: update_retention_label,
		path: "/drives/{{id}}/items/{{id2}}/retentionLabel",
		body: true,
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Invoke function search", 
		name: search,
		path: "/drives/{{id}}/items/{{id2}}/search(q='{{id3}}')",
		params: q
	);
	post!(
		doc: "Create new navigation property to subscriptions for drives", 
		name: create_subscriptions,
		path: "/drives/{{id}}/items/{{id2}}/subscriptions",
		body: true,
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get subscriptions from drives", 
		name: list_subscriptions,
		path: "/drives/{{id}}/items/{{id2}}/subscriptions",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: subscriptions,
		path: "/drives/{{id}}/items/{{id2}}/subscriptions/$count",
		params: drive_id, drive_item_id
	);
	delete!(
		doc: "Delete navigation property subscriptions for drives", 
		name: delete_subscriptions,
		path: "/drives/{{id}}/items/{{id2}}/subscriptions/{{id3}}",
		params: drive_id, drive_item_id, subscription_id
	);
	get!(
		doc: "Get subscriptions from drives", 
		name: get_subscriptions,
		path: "/drives/{{id}}/items/{{id2}}/subscriptions/{{id3}}",
		params: drive_id, drive_item_id, subscription_id
	);
	patch!(
		doc: "Update the navigation property subscriptions in drives", 
		name: update_subscriptions,
		path: "/drives/{{id}}/items/{{id2}}/subscriptions/{{id3}}",
		body: true,
		params: drive_id, drive_item_id, subscription_id
	);
	post!(
		doc: "Invoke action reauthorize", 
		name: reauthorize,
		path: "/drives/{{id}}/items/{{id2}}/subscriptions/{{id3}}/reauthorize",
		params: drive_id, drive_item_id, subscription_id
	);
	post!(
		doc: "Create new navigation property to thumbnails for drives", 
		name: create_thumbnails,
		path: "/drives/{{id}}/items/{{id2}}/thumbnails",
		body: true,
		params: drive_id, drive_item_id
	);
	get!(
		doc: "List thumbnails for a DriveItem", 
		name: list_thumbnails,
		path: "/drives/{{id}}/items/{{id2}}/thumbnails",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: thumbnails,
		path: "/drives/{{id}}/items/{{id2}}/thumbnails/$count",
		params: drive_id, drive_item_id
	);
	delete!(
		doc: "Delete navigation property thumbnails for drives", 
		name: delete_thumbnails,
		path: "/drives/{{id}}/items/{{id2}}/thumbnails/{{id3}}",
		params: drive_id, drive_item_id, thumbnail_set_id
	);
	get!(
		doc: "Get thumbnails from drives", 
		name: get_thumbnails,
		path: "/drives/{{id}}/items/{{id2}}/thumbnails/{{id3}}",
		params: drive_id, drive_item_id, thumbnail_set_id
	);
	patch!(
		doc: "Update the navigation property thumbnails in drives", 
		name: update_thumbnails,
		path: "/drives/{{id}}/items/{{id2}}/thumbnails/{{id3}}",
		body: true,
		params: drive_id, drive_item_id, thumbnail_set_id
	);
	post!(
		doc: "Invoke action unfollow", 
		name: unfollow,
		path: "/drives/{{id}}/items/{{id2}}/unfollow",
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action validatePermission", 
		name: validate_permission,
		path: "/drives/{{id}}/items/{{id2}}/validatePermission",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Create new navigation property to versions for drives", 
		name: create_versions,
		path: "/drives/{{id}}/items/{{id2}}/versions",
		body: true,
		params: drive_id, drive_item_id
	);
	get!(
		doc: "List versions of a driveItem", 
		name: list_versions,
		path: "/drives/{{id}}/items/{{id2}}/versions",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: versions,
		path: "/drives/{{id}}/items/{{id2}}/versions/$count",
		params: drive_id, drive_item_id
	);
	delete!(
		doc: "Delete navigation property versions for drives", 
		name: delete_versions,
		path: "/drives/{{id}}/items/{{id2}}/versions/{{id3}}",
		params: drive_id, drive_item_id, drive_item_version_id
	);
	get!(
		doc: "Get a DriveItemVersion resource", 
		name: get_versions,
		path: "/drives/{{id}}/items/{{id2}}/versions/{{id3}}",
		params: drive_id, drive_item_id, drive_item_version_id
	);
	patch!(
		doc: "Update the navigation property versions in drives", 
		name: update_versions,
		path: "/drives/{{id}}/items/{{id2}}/versions/{{id3}}",
		body: true,
		params: drive_id, drive_item_id, drive_item_version_id
	);
	get!(
		doc: "Get content for the navigation property versions from drives", 
		name: get_versions_content,
		path: "/drives/{{id}}/items/{{id2}}/versions/{{id3}}/content",
		params: drive_id, drive_item_id, drive_item_version_id
	);
	put!(
		doc: "Update content for the navigation property versions in drives", 
		name: update_versions_content,
		path: "/drives/{{id}}/items/{{id2}}/versions/{{id3}}/content",
		body: true,
		params: drive_id, drive_item_id, drive_item_version_id
	);
	post!(
		doc: "Invoke action restoreVersion", 
		name: restore_version,
		path: "/drives/{{id}}/items/{{id2}}/versions/{{id3}}/restoreVersion",
		params: drive_id, drive_item_id, drive_item_version_id
	);
	delete!(
		doc: "Delete navigation property workbook for drives", 
		name: delete_workbook,
		path: "/drives/{{id}}/items/{{id2}}/workbook",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get workbook from drives", 
		name: get_workbook,
		path: "/drives/{{id}}/items/{{id2}}/workbook",
		params: drive_id, drive_item_id
	);
	patch!(
		doc: "Update the navigation property workbook in drives", 
		name: update_workbook,
		path: "/drives/{{id}}/items/{{id2}}/workbook",
		body: true,
		params: drive_id, drive_item_id
	);
	delete!(
		doc: "Delete navigation property application for drives", 
		name: delete_application,
		path: "/drives/{{id}}/items/{{id2}}/workbook/application",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get workbookApplication", 
		name: get_application,
		path: "/drives/{{id}}/items/{{id2}}/workbook/application",
		params: drive_id, drive_item_id
	);
	patch!(
		doc: "Update the navigation property application in drives", 
		name: update_application,
		path: "/drives/{{id}}/items/{{id2}}/workbook/application",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action calculate", 
		name: calculate,
		path: "/drives/{{id}}/items/{{id2}}/workbook/application/calculate",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action closeSession", 
		name: close_session,
		path: "/drives/{{id}}/items/{{id2}}/workbook/closeSession",
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Create new navigation property to comments for drives", 
		name: create_comments,
		path: "/drives/{{id}}/items/{{id2}}/workbook/comments",
		body: true,
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get comments from drives", 
		name: list_comments,
		path: "/drives/{{id}}/items/{{id2}}/workbook/comments",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: comments,
		path: "/drives/{{id}}/items/{{id2}}/workbook/comments/$count",
		params: drive_id, drive_item_id
	);
	delete!(
		doc: "Delete navigation property comments for drives", 
		name: delete_comments,
		path: "/drives/{{id}}/items/{{id2}}/workbook/comments/{{id3}}",
		params: drive_id, drive_item_id, workbook_comment_id
	);
	get!(
		doc: "Get comments from drives", 
		name: get_comments,
		path: "/drives/{{id}}/items/{{id2}}/workbook/comments/{{id3}}",
		params: drive_id, drive_item_id, workbook_comment_id
	);
	patch!(
		doc: "Update the navigation property comments in drives", 
		name: update_comments,
		path: "/drives/{{id}}/items/{{id2}}/workbook/comments/{{id3}}",
		body: true,
		params: drive_id, drive_item_id, workbook_comment_id
	);
	post!(
		doc: "Create new navigation property to replies for drives", 
		name: create_replies,
		path: "/drives/{{id}}/items/{{id2}}/workbook/comments/{{id3}}/replies",
		body: true,
		params: drive_id, drive_item_id, workbook_comment_id
	);
	get!(
		doc: "Get workbookCommentReply", 
		name: list_replies,
		path: "/drives/{{id}}/items/{{id2}}/workbook/comments/{{id3}}/replies",
		params: drive_id, drive_item_id, workbook_comment_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: replies,
		path: "/drives/{{id}}/items/{{id2}}/workbook/comments/{{id3}}/replies/$count",
		params: drive_id, drive_item_id, workbook_comment_id
	);
	delete!(
		doc: "Delete navigation property replies for drives", 
		name: delete_replies,
		path: "/drives/{{id}}/items/{{id2}}/workbook/comments/{{id3}}/replies/{{id4}}",
		params: drive_id, drive_item_id, workbook_comment_id, workbook_comment_reply_id
	);
	get!(
		doc: "Get workbookCommentReply", 
		name: get_replies,
		path: "/drives/{{id}}/items/{{id2}}/workbook/comments/{{id3}}/replies/{{id4}}",
		params: drive_id, drive_item_id, workbook_comment_id, workbook_comment_reply_id
	);
	patch!(
		doc: "Update the navigation property replies in drives", 
		name: update_replies,
		path: "/drives/{{id}}/items/{{id2}}/workbook/comments/{{id3}}/replies/{{id4}}",
		body: true,
		params: drive_id, drive_item_id, workbook_comment_id, workbook_comment_reply_id
	);
	post!(
		doc: "Invoke action createSession", 
		name: create_session,
		path: "/drives/{{id}}/items/{{id2}}/workbook/createSession",
		body: true,
		params: drive_id, drive_item_id
	);
	delete!(
		doc: "Delete navigation property functions for drives", 
		name: delete_functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get functions from drives", 
		name: get_functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions",
		params: drive_id, drive_item_id
	);
	patch!(
		doc: "Update the navigation property functions in drives", 
		name: update_functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action abs", 
		name: abs,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/abs",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action accrInt", 
		name: accr_int,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/accrInt",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action accrIntM", 
		name: accr_int_m,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/accrIntM",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action acos", 
		name: acos,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/acos",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action acosh", 
		name: acosh,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/acosh",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action acot", 
		name: acot,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/acot",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action acoth", 
		name: acoth,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/acoth",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action amorDegrc", 
		name: amor_degrc,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/amorDegrc",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action amorLinc", 
		name: amor_linc,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/amorLinc",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action and", 
		name: and,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/and",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action arabic", 
		name: arabic,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/arabic",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action areas", 
		name: areas,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/areas",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action asc", 
		name: asc,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/asc",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action asin", 
		name: asin,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/asin",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action asinh", 
		name: asinh,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/asinh",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action atan", 
		name: atan,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/atan",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action atan2", 
		name: functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/atan2",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action atanh", 
		name: atanh,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/atanh",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action aveDev", 
		name: ave_dev,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/aveDev",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action average", 
		name: average,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/average",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action averageA", 
		name: average_a,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/averageA",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action averageIf", 
		name: average_if,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/averageIf",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action averageIfs", 
		name: average_ifs,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/averageIfs",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action bahtText", 
		name: baht_text,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/bahtText",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action base", 
		name: base,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/base",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action besselI", 
		name: bessel_i,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/besselI",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action besselJ", 
		name: bessel_j,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/besselJ",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action besselK", 
		name: bessel_k,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/besselK",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action besselY", 
		name: bessel_y,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/besselY",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action beta_Dist", 
		name: beta_dist,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/beta_Dist",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action beta_Inv", 
		name: beta_inv,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/beta_Inv",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action bin2Dec", 
		name: functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/bin2Dec",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action bin2Hex", 
		name: functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/bin2Hex",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action bin2Oct", 
		name: functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/bin2Oct",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action binom_Dist", 
		name: binom_dist,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/binom_Dist",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action binom_Dist_Range", 
		name: binom_dist_range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/binom_Dist_Range",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action binom_Inv", 
		name: binom_inv,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/binom_Inv",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action bitand", 
		name: bitand,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/bitand",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action bitlshift", 
		name: bitlshift,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/bitlshift",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action bitor", 
		name: bitor,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/bitor",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action bitrshift", 
		name: bitrshift,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/bitrshift",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action bitxor", 
		name: bitxor,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/bitxor",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action ceiling_Math", 
		name: ceiling_math,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/ceiling_Math",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action ceiling_Precise", 
		name: ceiling_precise,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/ceiling_Precise",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action char", 
		name: char,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/char",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action chiSq_Dist", 
		name: chi_sq_dist,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/chiSq_Dist",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action chiSq_Dist_RT", 
		name: chi_sq_dist_rt,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/chiSq_Dist_RT",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action chiSq_Inv", 
		name: chi_sq_inv,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/chiSq_Inv",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action chiSq_Inv_RT", 
		name: chi_sq_inv_rt,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/chiSq_Inv_RT",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action choose", 
		name: choose,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/choose",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action clean", 
		name: clean,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/clean",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action code", 
		name: code,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/code",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action columns", 
		name: columns,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/columns",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action combin", 
		name: combin,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/combin",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action combina", 
		name: combina,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/combina",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action complex", 
		name: complex,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/complex",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action concatenate", 
		name: concatenate,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/concatenate",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action confidence_Norm", 
		name: confidence_norm,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/confidence_Norm",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action confidence_T", 
		name: confidence_t,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/confidence_T",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action convert", 
		name: convert,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/convert",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action cos", 
		name: cos,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/cos",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action cosh", 
		name: cosh,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/cosh",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action cot", 
		name: cot,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/cot",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action coth", 
		name: coth,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/coth",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action count", 
		name: count,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/count",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action countA", 
		name: count_a,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/countA",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action countBlank", 
		name: count_blank,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/countBlank",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action countIf", 
		name: count_if,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/countIf",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action countIfs", 
		name: count_ifs,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/countIfs",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action coupDayBs", 
		name: coup_day_bs,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/coupDayBs",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action coupDays", 
		name: coup_days,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/coupDays",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action coupDaysNc", 
		name: coup_days_nc,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/coupDaysNc",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action coupNcd", 
		name: coup_ncd,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/coupNcd",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action coupNum", 
		name: coup_num,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/coupNum",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action coupPcd", 
		name: coup_pcd,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/coupPcd",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action csc", 
		name: csc,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/csc",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action csch", 
		name: csch,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/csch",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action cumIPmt", 
		name: cum_i_pmt,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/cumIPmt",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action cumPrinc", 
		name: cum_princ,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/cumPrinc",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action date", 
		name: date,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/date",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action datevalue", 
		name: datevalue,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/datevalue",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action daverage", 
		name: daverage,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/daverage",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action day", 
		name: day,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/day",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action days", 
		name: days,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/days",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action days360", 
		name: functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/days360",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action db", 
		name: db,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/db",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action dbcs", 
		name: dbcs,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/dbcs",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action dcount", 
		name: dcount,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/dcount",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action dcountA", 
		name: dcount_a,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/dcountA",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action ddb", 
		name: ddb,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/ddb",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action dec2Bin", 
		name: functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/dec2Bin",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action dec2Hex", 
		name: functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/dec2Hex",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action dec2Oct", 
		name: functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/dec2Oct",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action decimal", 
		name: decimal,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/decimal",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action degrees", 
		name: degrees,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/degrees",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action delta", 
		name: delta,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/delta",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action devSq", 
		name: dev_sq,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/devSq",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action dget", 
		name: dget,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/dget",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action disc", 
		name: disc,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/disc",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action dmax", 
		name: dmax,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/dmax",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action dmin", 
		name: dmin,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/dmin",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action dollar", 
		name: dollar,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/dollar",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action dollarDe", 
		name: dollar_de,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/dollarDe",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action dollarFr", 
		name: dollar_fr,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/dollarFr",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action dproduct", 
		name: dproduct,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/dproduct",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action dstDev", 
		name: dst_dev,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/dstDev",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action dstDevP", 
		name: dst_dev_p,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/dstDevP",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action dsum", 
		name: dsum,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/dsum",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action duration", 
		name: duration,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/duration",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action dvar", 
		name: dvar,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/dvar",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action dvarP", 
		name: dvar_p,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/dvarP",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action ecma_Ceiling", 
		name: ecma_ceiling,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/ecma_Ceiling",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action edate", 
		name: edate,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/edate",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action effect", 
		name: effect,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/effect",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action eoMonth", 
		name: eo_month,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/eoMonth",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action erf", 
		name: erf,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/erf",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action erfC", 
		name: erf_c,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/erfC",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action erfC_Precise", 
		name: erf_c_precise,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/erfC_Precise",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action erf_Precise", 
		name: erf_precise,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/erf_Precise",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action error_Type", 
		name: error_type,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/error_Type",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action even", 
		name: even,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/even",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action exact", 
		name: exact,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/exact",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action exp", 
		name: exp,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/exp",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action expon_Dist", 
		name: expon_dist,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/expon_Dist",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action f_Dist", 
		name: f_dist,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/f_Dist",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action f_Dist_RT", 
		name: f_dist_rt,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/f_Dist_RT",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action f_Inv", 
		name: f_inv,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/f_Inv",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action f_Inv_RT", 
		name: f_inv_rt,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/f_Inv_RT",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action fact", 
		name: fact,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/fact",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action factDouble", 
		name: fact_double,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/factDouble",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action false", 
		name: false,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/false",
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action find", 
		name: find,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/find",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action findB", 
		name: find_b,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/findB",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action fisher", 
		name: fisher,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/fisher",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action fisherInv", 
		name: fisher_inv,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/fisherInv",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action fixed", 
		name: fixed,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/fixed",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action floor_Math", 
		name: floor_math,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/floor_Math",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action floor_Precise", 
		name: floor_precise,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/floor_Precise",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action fv", 
		name: fv,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/fv",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action fvschedule", 
		name: fvschedule,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/fvschedule",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action gamma", 
		name: gamma,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/gamma",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action gammaLn", 
		name: gamma_ln,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/gammaLn",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action gammaLn_Precise", 
		name: gamma_ln_precise,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/gammaLn_Precise",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action gamma_Dist", 
		name: gamma_dist,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/gamma_Dist",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action gamma_Inv", 
		name: gamma_inv,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/gamma_Inv",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action gauss", 
		name: gauss,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/gauss",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action gcd", 
		name: gcd,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/gcd",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action geStep", 
		name: ge_step,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/geStep",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action geoMean", 
		name: geo_mean,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/geoMean",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action harMean", 
		name: har_mean,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/harMean",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action hex2Bin", 
		name: functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/hex2Bin",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action hex2Dec", 
		name: functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/hex2Dec",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action hex2Oct", 
		name: functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/hex2Oct",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action hlookup", 
		name: hlookup,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/hlookup",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action hour", 
		name: hour,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/hour",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action hypGeom_Dist", 
		name: hyp_geom_dist,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/hypGeom_Dist",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action hyperlink", 
		name: hyperlink,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/hyperlink",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action if", 
		name: if,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/if",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imAbs", 
		name: im_abs,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imAbs",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imArgument", 
		name: im_argument,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imArgument",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imConjugate", 
		name: im_conjugate,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imConjugate",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imCos", 
		name: im_cos,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imCos",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imCosh", 
		name: im_cosh,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imCosh",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imCot", 
		name: im_cot,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imCot",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imCsc", 
		name: im_csc,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imCsc",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imCsch", 
		name: im_csch,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imCsch",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imDiv", 
		name: im_div,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imDiv",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imExp", 
		name: im_exp,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imExp",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imLn", 
		name: im_ln,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imLn",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imLog10", 
		name: functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imLog10",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imLog2", 
		name: functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imLog2",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imPower", 
		name: im_power,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imPower",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imProduct", 
		name: im_product,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imProduct",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imReal", 
		name: im_real,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imReal",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imSec", 
		name: im_sec,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imSec",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imSech", 
		name: im_sech,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imSech",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imSin", 
		name: im_sin,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imSin",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imSinh", 
		name: im_sinh,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imSinh",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imSqrt", 
		name: im_sqrt,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imSqrt",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imSub", 
		name: im_sub,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imSub",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imSum", 
		name: im_sum,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imSum",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imTan", 
		name: im_tan,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imTan",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action imaginary", 
		name: imaginary,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/imaginary",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action int", 
		name: int,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/int",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action intRate", 
		name: int_rate,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/intRate",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action ipmt", 
		name: ipmt,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/ipmt",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action irr", 
		name: irr,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/irr",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action isErr", 
		name: is_err,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/isErr",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action isError", 
		name: is_error,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/isError",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action isEven", 
		name: is_even,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/isEven",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action isFormula", 
		name: is_formula,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/isFormula",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action isLogical", 
		name: is_logical,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/isLogical",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action isNA", 
		name: is_na,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/isNA",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action isNonText", 
		name: is_non_text,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/isNonText",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action isNumber", 
		name: is_number,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/isNumber",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action isOdd", 
		name: is_odd,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/isOdd",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action isText", 
		name: is_text,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/isText",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action isoWeekNum", 
		name: iso_week_num,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/isoWeekNum",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action iso_Ceiling", 
		name: iso_ceiling,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/iso_Ceiling",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action ispmt", 
		name: ispmt,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/ispmt",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action isref", 
		name: isref,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/isref",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action kurt", 
		name: kurt,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/kurt",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action large", 
		name: large,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/large",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action lcm", 
		name: lcm,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/lcm",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action left", 
		name: left,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/left",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action leftb", 
		name: leftb,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/leftb",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action len", 
		name: len,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/len",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action lenb", 
		name: lenb,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/lenb",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action ln", 
		name: ln,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/ln",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action log", 
		name: log,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/log",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action log10", 
		name: functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/log10",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action logNorm_Dist", 
		name: log_norm_dist,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/logNorm_Dist",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action logNorm_Inv", 
		name: log_norm_inv,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/logNorm_Inv",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action lookup", 
		name: lookup,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/lookup",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action lower", 
		name: lower,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/lower",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action match", 
		name: match,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/match",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action max", 
		name: max,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/max",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action maxA", 
		name: max_a,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/maxA",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action mduration", 
		name: mduration,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/mduration",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action median", 
		name: median,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/median",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action mid", 
		name: mid,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/mid",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action midb", 
		name: midb,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/midb",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action min", 
		name: min,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/min",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action minA", 
		name: min_a,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/minA",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action minute", 
		name: minute,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/minute",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action mirr", 
		name: mirr,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/mirr",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action mod", 
		name: mod,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/mod",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action month", 
		name: month,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/month",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action mround", 
		name: mround,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/mround",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action multiNomial", 
		name: multi_nomial,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/multiNomial",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action n", 
		name: n,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/n",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action na", 
		name: na,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/na",
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action negBinom_Dist", 
		name: neg_binom_dist,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/negBinom_Dist",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action networkDays", 
		name: network_days,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/networkDays",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action networkDays_Intl", 
		name: network_days_intl,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/networkDays_Intl",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action nominal", 
		name: nominal,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/nominal",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action norm_Dist", 
		name: norm_dist,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/norm_Dist",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action norm_Inv", 
		name: norm_inv,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/norm_Inv",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action norm_S_Dist", 
		name: norm_s_dist,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/norm_S_Dist",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action norm_S_Inv", 
		name: norm_s_inv,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/norm_S_Inv",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action not", 
		name: not,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/not",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action now", 
		name: now,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/now",
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action nper", 
		name: nper,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/nper",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action npv", 
		name: npv,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/npv",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action numberValue", 
		name: number_value,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/numberValue",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action oct2Bin", 
		name: functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/oct2Bin",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action oct2Dec", 
		name: functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/oct2Dec",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action oct2Hex", 
		name: functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/oct2Hex",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action odd", 
		name: odd,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/odd",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action oddFPrice", 
		name: odd_f_price,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/oddFPrice",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action oddFYield", 
		name: odd_f_yield,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/oddFYield",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action oddLPrice", 
		name: odd_l_price,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/oddLPrice",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action oddLYield", 
		name: odd_l_yield,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/oddLYield",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action or", 
		name: or,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/or",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action pduration", 
		name: pduration,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/pduration",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action percentRank_Exc", 
		name: percent_rank_exc,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/percentRank_Exc",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action percentRank_Inc", 
		name: percent_rank_inc,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/percentRank_Inc",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action percentile_Exc", 
		name: percentile_exc,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/percentile_Exc",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action percentile_Inc", 
		name: percentile_inc,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/percentile_Inc",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action permut", 
		name: permut,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/permut",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action permutationa", 
		name: permutationa,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/permutationa",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action phi", 
		name: phi,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/phi",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action pi", 
		name: pi,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/pi",
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action pmt", 
		name: pmt,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/pmt",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action poisson_Dist", 
		name: poisson_dist,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/poisson_Dist",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action power", 
		name: power,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/power",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action ppmt", 
		name: ppmt,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/ppmt",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action price", 
		name: price,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/price",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action priceDisc", 
		name: price_disc,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/priceDisc",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action priceMat", 
		name: price_mat,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/priceMat",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action product", 
		name: product,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/product",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action proper", 
		name: proper,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/proper",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action pv", 
		name: pv,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/pv",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action quartile_Exc", 
		name: quartile_exc,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/quartile_Exc",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action quartile_Inc", 
		name: quartile_inc,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/quartile_Inc",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action quotient", 
		name: quotient,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/quotient",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action radians", 
		name: radians,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/radians",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action rand", 
		name: rand,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/rand",
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action randBetween", 
		name: rand_between,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/randBetween",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action rank_Avg", 
		name: rank_avg,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/rank_Avg",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action rank_Eq", 
		name: rank_eq,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/rank_Eq",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action rate", 
		name: rate,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/rate",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action received", 
		name: received,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/received",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action replace", 
		name: replace,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/replace",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action replaceB", 
		name: replace_b,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/replaceB",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action rept", 
		name: rept,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/rept",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action right", 
		name: right,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/right",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action rightb", 
		name: rightb,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/rightb",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action roman", 
		name: roman,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/roman",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action round", 
		name: round,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/round",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action roundDown", 
		name: round_down,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/roundDown",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action roundUp", 
		name: round_up,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/roundUp",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action rows", 
		name: rows,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/rows",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action rri", 
		name: rri,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/rri",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action sec", 
		name: sec,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/sec",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action sech", 
		name: sech,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/sech",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action second", 
		name: second,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/second",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action seriesSum", 
		name: series_sum,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/seriesSum",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action sheet", 
		name: sheet,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/sheet",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action sheets", 
		name: sheets,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/sheets",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action sign", 
		name: sign,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/sign",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action sin", 
		name: sin,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/sin",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action sinh", 
		name: sinh,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/sinh",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action skew", 
		name: skew,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/skew",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action skew_p", 
		name: skew_p,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/skew_p",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action sln", 
		name: sln,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/sln",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action small", 
		name: small,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/small",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action sqrt", 
		name: sqrt,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/sqrt",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action sqrtPi", 
		name: sqrt_pi,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/sqrtPi",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action stDevA", 
		name: st_dev_a,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/stDevA",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action stDevPA", 
		name: st_dev_pa,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/stDevPA",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action stDev_P", 
		name: st_dev_p,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/stDev_P",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action stDev_S", 
		name: st_dev_s,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/stDev_S",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action standardize", 
		name: standardize,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/standardize",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action substitute", 
		name: substitute,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/substitute",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action subtotal", 
		name: subtotal,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/subtotal",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action sum", 
		name: sum,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/sum",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action sumIf", 
		name: sum_if,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/sumIf",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action sumIfs", 
		name: sum_ifs,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/sumIfs",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action sumSq", 
		name: sum_sq,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/sumSq",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action syd", 
		name: syd,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/syd",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action t", 
		name: t,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/t",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action t_Dist", 
		name: t_dist,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/t_Dist",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action t_Dist_2T", 
		name: functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/t_Dist_2T",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action t_Dist_RT", 
		name: t_dist_rt,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/t_Dist_RT",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action t_Inv", 
		name: t_inv,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/t_Inv",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action t_Inv_2T", 
		name: functions,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/t_Inv_2T",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action tan", 
		name: tan,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/tan",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action tanh", 
		name: tanh,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/tanh",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action tbillEq", 
		name: tbill_eq,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/tbillEq",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action tbillPrice", 
		name: tbill_price,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/tbillPrice",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action tbillYield", 
		name: tbill_yield,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/tbillYield",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action text", 
		name: text,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/text",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action time", 
		name: time,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/time",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action timevalue", 
		name: timevalue,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/timevalue",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action today", 
		name: today,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/today",
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action trim", 
		name: trim,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/trim",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action trimMean", 
		name: trim_mean,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/trimMean",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action true", 
		name: true,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/true",
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action trunc", 
		name: trunc,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/trunc",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action type", 
		name: type,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/type",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action unichar", 
		name: unichar,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/unichar",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action unicode", 
		name: unicode,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/unicode",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action upper", 
		name: upper,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/upper",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action usdollar", 
		name: usdollar,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/usdollar",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action value", 
		name: value,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/value",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action varA", 
		name: var_a,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/varA",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action varPA", 
		name: var_pa,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/varPA",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action var_P", 
		name: var_p,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/var_P",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action var_S", 
		name: var_s,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/var_S",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action vdb", 
		name: vdb,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/vdb",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action vlookup", 
		name: vlookup,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/vlookup",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action weekNum", 
		name: week_num,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/weekNum",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action weekday", 
		name: weekday,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/weekday",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action weibull_Dist", 
		name: weibull_dist,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/weibull_Dist",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action workDay", 
		name: work_day,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/workDay",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action workDay_Intl", 
		name: work_day_intl,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/workDay_Intl",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action xirr", 
		name: xirr,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/xirr",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action xnpv", 
		name: xnpv,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/xnpv",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action xor", 
		name: xor,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/xor",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action year", 
		name: year,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/year",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action yearFrac", 
		name: year_frac,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/yearFrac",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action yield", 
		name: yield,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/yield",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action yieldDisc", 
		name: yield_disc,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/yieldDisc",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action yieldMat", 
		name: yield_mat,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/yieldMat",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action z_Test", 
		name: z_test,
		path: "/drives/{{id}}/items/{{id2}}/workbook/functions/z_Test",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Create new navigation property to names for drives", 
		name: create_names,
		path: "/drives/{{id}}/items/{{id2}}/workbook/names",
		body: true,
		params: drive_id, drive_item_id
	);
	get!(
		doc: "List NamedItemCollection", 
		name: list_names,
		path: "/drives/{{id}}/items/{{id2}}/workbook/names",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count_eada,
		path: "/drives/{{id}}/items/{{id2}}/workbook/names/$count",
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action add", 
		name: add,
		path: "/drives/{{id}}/items/{{id2}}/workbook/names/add",
		body: true,
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action addFormulaLocal", 
		name: add_formula_local,
		path: "/drives/{{id}}/items/{{id2}}/workbook/names/addFormulaLocal",
		body: true,
		params: drive_id, drive_item_id
	);
	delete!(
		doc: "Delete navigation property names for drives", 
		name: delete_names,
		path: "/drives/{{id}}/items/{{id2}}/workbook/names/{{id3}}",
		params: drive_id, drive_item_id, workbook_named_item_id
	);
	get!(
		doc: "Get NamedItem", 
		name: get_names,
		path: "/drives/{{id}}/items/{{id2}}/workbook/names/{{id3}}",
		params: drive_id, drive_item_id, workbook_named_item_id
	);
	patch!(
		doc: "Update nameditem", 
		name: update_names,
		path: "/drives/{{id}}/items/{{id2}}/workbook/names/{{id3}}",
		body: true,
		params: drive_id, drive_item_id, workbook_named_item_id
	);
	get!(
		doc: "Invoke function range", 
		name: range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/names/{{id3}}/range()",
		params: drive_id, drive_item_id, workbook_named_item_id
	);
	get!(
		doc: "Get worksheet from drives", 
		name: get_worksheet,
		path: "/drives/{{id}}/items/{{id2}}/workbook/names/{{id3}}/worksheet",
		params: drive_id, drive_item_id, workbook_named_item_id
	);
	post!(
		doc: "Create new navigation property to operations for drives", 
		name: create_operations,
		path: "/drives/{{id}}/items/{{id2}}/workbook/operations",
		body: true,
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get workbookOperation", 
		name: list_operations,
		path: "/drives/{{id}}/items/{{id2}}/workbook/operations",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: operations,
		path: "/drives/{{id}}/items/{{id2}}/workbook/operations/$count",
		params: drive_id, drive_item_id
	);
	delete!(
		doc: "Delete navigation property operations for drives", 
		name: delete_operations,
		path: "/drives/{{id}}/items/{{id2}}/workbook/operations/{{id3}}",
		params: drive_id, drive_item_id, workbook_operation_id
	);
	get!(
		doc: "Get workbookOperation", 
		name: get_operations,
		path: "/drives/{{id}}/items/{{id2}}/workbook/operations/{{id3}}",
		params: drive_id, drive_item_id, workbook_operation_id
	);
	patch!(
		doc: "Update the navigation property operations in drives", 
		name: update_operations,
		path: "/drives/{{id}}/items/{{id2}}/workbook/operations/{{id3}}",
		body: true,
		params: drive_id, drive_item_id, workbook_operation_id
	);
	post!(
		doc: "Invoke action refreshSession", 
		name: refresh_session,
		path: "/drives/{{id}}/items/{{id2}}/workbook/refreshSession",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Invoke function sessionInfoResource", 
		name: session_info_resource,
		path: "/drives/{{id}}/items/{{id2}}/workbook/sessionInfoResource(key='{{id3}}')",
		params: key
	);
	get!(
		doc: "Invoke function tableRowOperationResult", 
		name: table_row_operation_result,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tableRowOperationResult(key='{{id3}}')",
		params: key
	);
	post!(
		doc: "Create new navigation property to tables for drives", 
		name: create_tables,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables",
		body: true,
		params: drive_id, drive_item_id
	);
	get!(
		doc: "List tables", 
		name: list_tables,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables",
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action add", 
		name: add,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/add",
		body: true,
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Invoke function count", 
		name: count,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/count()",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Invoke function itemAt", 
		name: item_at,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/itemAt(index={{id3}})",
		params: index
	);
	get!(
		doc: "Get Table", 
		name: get_tables,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}",
		params: drive_id, drive_item_id, workbook_table_id
	);
	delete!(
		doc: "Table: delete", 
		name: delete_tables,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}",
		params: drive_id, drive_item_id, workbook_table_id
	);
	patch!(
		doc: "Update table", 
		name: update_tables,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}",
		body: true,
		params: drive_id, drive_item_id, workbook_table_id
	);
	post!(
		doc: "Invoke action clearFilters", 
		name: clear_filters,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/clearFilters",
		params: drive_id, drive_item_id, workbook_table_id
	);
	post!(
		doc: "Create TableColumn", 
		name: create_columns,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns",
		body: true,
		params: drive_id, drive_item_id, workbook_table_id
	);
	get!(
		doc: "List TableColumnCollection", 
		name: list_columns,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns",
		params: drive_id, drive_item_id, workbook_table_id
	);
	post!(
		doc: "Invoke action add", 
		name: add,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/add",
		body: true,
		params: drive_id, drive_item_id, workbook_table_id
	);
	get!(
		doc: "Invoke function count", 
		name: count,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/count()",
		params: drive_id, drive_item_id, workbook_table_id
	);
	get!(
		doc: "Invoke function itemAt", 
		name: item_at,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/itemAt(index={{id4}})",
		params: index
	);
	get!(
		doc: "Get TableColumn", 
		name: get_columns,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}",
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	delete!(
		doc: "TableColumn: delete", 
		name: delete_columns,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}",
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	patch!(
		doc: "Update tablecolumn", 
		name: update_columns,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}",
		body: true,
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	get!(
		doc: "Invoke function dataBodyRange", 
		name: data_body_range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}/dataBodyRange()",
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	delete!(
		doc: "Delete navigation property filter for drives", 
		name: delete_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}/filter",
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	get!(
		doc: "Get filter from drives", 
		name: get_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}/filter",
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	patch!(
		doc: "Update the navigation property filter in drives", 
		name: update_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}/filter",
		body: true,
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action apply", 
		name: apply,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}/filter/apply",
		body: true,
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action applyBottomItemsFilter", 
		name: apply_bottom_items_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}/filter/applyBottomItemsFilter",
		body: true,
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action applyBottomPercentFilter", 
		name: apply_bottom_percent_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}/filter/applyBottomPercentFilter",
		body: true,
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action applyCellColorFilter", 
		name: apply_cell_color_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}/filter/applyCellColorFilter",
		body: true,
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action applyCustomFilter", 
		name: apply_custom_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}/filter/applyCustomFilter",
		body: true,
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action applyDynamicFilter", 
		name: apply_dynamic_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}/filter/applyDynamicFilter",
		body: true,
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action applyFontColorFilter", 
		name: apply_font_color_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}/filter/applyFontColorFilter",
		body: true,
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action applyIconFilter", 
		name: apply_icon_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}/filter/applyIconFilter",
		body: true,
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action applyTopItemsFilter", 
		name: apply_top_items_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}/filter/applyTopItemsFilter",
		body: true,
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action applyTopPercentFilter", 
		name: apply_top_percent_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}/filter/applyTopPercentFilter",
		body: true,
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action applyValuesFilter", 
		name: apply_values_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}/filter/applyValuesFilter",
		body: true,
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action clear", 
		name: clear,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}/filter/clear",
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	get!(
		doc: "Invoke function headerRowRange", 
		name: header_row_range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}/headerRowRange()",
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	get!(
		doc: "Invoke function range", 
		name: range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}/range()",
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	get!(
		doc: "Invoke function totalRowRange", 
		name: total_row_range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/columns/{{id4}}/totalRowRange()",
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action convertToRange", 
		name: convert_to_range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/convertToRange",
		params: drive_id, drive_item_id, workbook_table_id
	);
	get!(
		doc: "Invoke function dataBodyRange", 
		name: data_body_range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/dataBodyRange()",
		params: drive_id, drive_item_id, workbook_table_id
	);
	get!(
		doc: "Invoke function headerRowRange", 
		name: header_row_range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/headerRowRange()",
		params: drive_id, drive_item_id, workbook_table_id
	);
	get!(
		doc: "Invoke function range", 
		name: range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/range()",
		params: drive_id, drive_item_id, workbook_table_id
	);
	post!(
		doc: "Invoke action reapplyFilters", 
		name: reapply_filters,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/reapplyFilters",
		params: drive_id, drive_item_id, workbook_table_id
	);
	post!(
		doc: "Create TableRow", 
		name: create_rows,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/rows",
		body: true,
		params: drive_id, drive_item_id, workbook_table_id
	);
	get!(
		doc: "List TableRowCollection", 
		name: list_rows,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/rows",
		params: drive_id, drive_item_id, workbook_table_id
	);
	post!(
		doc: "Invoke action add", 
		name: add,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/rows/add",
		body: true,
		params: drive_id, drive_item_id, workbook_table_id
	);
	get!(
		doc: "Invoke function count", 
		name: count,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/rows/count()",
		params: drive_id, drive_item_id, workbook_table_id
	);
	get!(
		doc: "Invoke function itemAt", 
		name: item_at,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/rows/itemAt(index={{id4}})",
		params: index
	);
	get!(
		doc: "Get TableRow", 
		name: get_rows,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/rows/{{id4}}",
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_row_id
	);
	delete!(
		doc: "TableRow: delete", 
		name: delete_rows,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/rows/{{id4}}",
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_row_id
	);
	patch!(
		doc: "Update tablerow", 
		name: update_rows,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/rows/{{id4}}",
		body: true,
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_row_id
	);
	get!(
		doc: "Invoke function range", 
		name: range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/rows/{{id4}}/range()",
		params: drive_id, drive_item_id, workbook_table_id, workbook_table_row_id
	);
	delete!(
		doc: "Delete navigation property sort for drives", 
		name: delete_sort,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/sort",
		params: drive_id, drive_item_id, workbook_table_id
	);
	get!(
		doc: "Get TableSort", 
		name: get_sort,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/sort",
		params: drive_id, drive_item_id, workbook_table_id
	);
	patch!(
		doc: "Update the navigation property sort in drives", 
		name: update_sort,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/sort",
		body: true,
		params: drive_id, drive_item_id, workbook_table_id
	);
	post!(
		doc: "Invoke action apply", 
		name: apply,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/sort/apply",
		body: true,
		params: drive_id, drive_item_id, workbook_table_id
	);
	post!(
		doc: "Invoke action clear", 
		name: clear,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/sort/clear",
		params: drive_id, drive_item_id, workbook_table_id
	);
	post!(
		doc: "Invoke action reapply", 
		name: reapply,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/sort/reapply",
		params: drive_id, drive_item_id, workbook_table_id
	);
	get!(
		doc: "Invoke function totalRowRange", 
		name: total_row_range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/totalRowRange()",
		params: drive_id, drive_item_id, workbook_table_id
	);
	get!(
		doc: "Get worksheet from drives", 
		name: get_worksheet,
		path: "/drives/{{id}}/items/{{id2}}/workbook/tables/{{id3}}/worksheet",
		params: drive_id, drive_item_id, workbook_table_id
	);
	post!(
		doc: "Create new navigation property to worksheets for drives", 
		name: create_worksheets,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets",
		body: true,
		params: drive_id, drive_item_id
	);
	get!(
		doc: "List WorksheetCollection", 
		name: list_worksheets,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: worksheets,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/$count",
		params: drive_id, drive_item_id
	);
	post!(
		doc: "Invoke action add", 
		name: add,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/add",
		body: true,
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get Worksheet", 
		name: get_worksheets,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}",
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	patch!(
		doc: "Update worksheet", 
		name: update_worksheets,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	delete!(
		doc: "Worksheet: delete", 
		name: delete_worksheets,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}",
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	get!(
		doc: "Invoke function cell", 
		name: cell,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/cell(row={{id4}},column={{id5}})",
		params: row, column
	);
	post!(
		doc: "Create Chart", 
		name: create_charts,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	get!(
		doc: "List charts", 
		name: list_charts,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts",
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	post!(
		doc: "Invoke action add", 
		name: add,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/add",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	get!(
		doc: "Invoke function count", 
		name: count,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/count()",
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	get!(
		doc: "Invoke function item", 
		name: item,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/item(name='{{id4}}')",
		params: name
	);
	get!(
		doc: "Invoke function itemAt", 
		name: item_at,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/itemAt(index={{id4}})",
		params: index
	);
	delete!(
		doc: "Chart: delete", 
		name: delete_charts,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get Chart", 
		name: get_charts,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update chart", 
		name: update_charts,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property axes for drives", 
		name: delete_axes,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get axes from drives", 
		name: get_axes,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property axes in drives", 
		name: update_axes,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property categoryAxis for drives", 
		name: delete_category_axis,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get categoryAxis from drives", 
		name: get_category_axis,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property categoryAxis in drives", 
		name: update_category_axis,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property format for drives", 
		name: delete_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get format from drives", 
		name: get_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property format in drives", 
		name: update_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/format",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property font for drives", 
		name: delete_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/format/font",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get ChartFont", 
		name: get_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/format/font",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update chartfont", 
		name: update_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/format/font",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property line for drives", 
		name: delete_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/format/line",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get ChartLineFormat", 
		name: get_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/format/line",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update chartlineformat", 
		name: update_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/format/line",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	post!(
		doc: "Invoke action clear", 
		name: clear,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/format/line/clear",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property majorGridlines for drives", 
		name: delete_major_gridlines,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/majorGridlines",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get majorGridlines from drives", 
		name: get_major_gridlines,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/majorGridlines",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property majorGridlines in drives", 
		name: update_major_gridlines,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/majorGridlines",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property format for drives", 
		name: delete_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/majorGridlines/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get format from drives", 
		name: get_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/majorGridlines/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property format in drives", 
		name: update_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/majorGridlines/format",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property line for drives", 
		name: delete_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/majorGridlines/format/line",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get line from drives", 
		name: get_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/majorGridlines/format/line",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property line in drives", 
		name: update_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/majorGridlines/format/line",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	post!(
		doc: "Invoke action clear", 
		name: clear,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/majorGridlines/format/line/clear",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property minorGridlines for drives", 
		name: delete_minor_gridlines,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/minorGridlines",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get ChartGridlines", 
		name: get_minor_gridlines,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/minorGridlines",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update chartgridlines", 
		name: update_minor_gridlines,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/minorGridlines",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property format for drives", 
		name: delete_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/minorGridlines/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get format from drives", 
		name: get_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/minorGridlines/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property format in drives", 
		name: update_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/minorGridlines/format",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property line for drives", 
		name: delete_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/minorGridlines/format/line",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get line from drives", 
		name: get_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/minorGridlines/format/line",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property line in drives", 
		name: update_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/minorGridlines/format/line",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	post!(
		doc: "Invoke action clear", 
		name: clear,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/minorGridlines/format/line/clear",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property title for drives", 
		name: delete_title,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/title",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get ChartAxisTitle", 
		name: get_title,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/title",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update chartaxistitle", 
		name: update_title,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/title",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property format for drives", 
		name: delete_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/title/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get format from drives", 
		name: get_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/title/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property format in drives", 
		name: update_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/title/format",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property font for drives", 
		name: delete_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/title/format/font",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get font from drives", 
		name: get_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/title/format/font",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property font in drives", 
		name: update_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/categoryAxis/title/format/font",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property seriesAxis for drives", 
		name: delete_series_axis,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get seriesAxis from drives", 
		name: get_series_axis,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property seriesAxis in drives", 
		name: update_series_axis,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property format for drives", 
		name: delete_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get format from drives", 
		name: get_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property format in drives", 
		name: update_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/format",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property font for drives", 
		name: delete_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/format/font",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get ChartFont", 
		name: get_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/format/font",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update chartfont", 
		name: update_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/format/font",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property line for drives", 
		name: delete_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/format/line",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get ChartLineFormat", 
		name: get_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/format/line",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update chartlineformat", 
		name: update_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/format/line",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	post!(
		doc: "Invoke action clear", 
		name: clear,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/format/line/clear",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property majorGridlines for drives", 
		name: delete_major_gridlines,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/majorGridlines",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get majorGridlines from drives", 
		name: get_major_gridlines,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/majorGridlines",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property majorGridlines in drives", 
		name: update_major_gridlines,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/majorGridlines",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property format for drives", 
		name: delete_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/majorGridlines/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get format from drives", 
		name: get_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/majorGridlines/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property format in drives", 
		name: update_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/majorGridlines/format",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property line for drives", 
		name: delete_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/majorGridlines/format/line",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get line from drives", 
		name: get_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/majorGridlines/format/line",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property line in drives", 
		name: update_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/majorGridlines/format/line",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	post!(
		doc: "Invoke action clear", 
		name: clear,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/majorGridlines/format/line/clear",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property minorGridlines for drives", 
		name: delete_minor_gridlines,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/minorGridlines",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get ChartGridlines", 
		name: get_minor_gridlines,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/minorGridlines",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update chartgridlines", 
		name: update_minor_gridlines,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/minorGridlines",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property format for drives", 
		name: delete_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/minorGridlines/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get format from drives", 
		name: get_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/minorGridlines/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property format in drives", 
		name: update_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/minorGridlines/format",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property line for drives", 
		name: delete_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/minorGridlines/format/line",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get line from drives", 
		name: get_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/minorGridlines/format/line",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property line in drives", 
		name: update_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/minorGridlines/format/line",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	post!(
		doc: "Invoke action clear", 
		name: clear,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/minorGridlines/format/line/clear",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property title for drives", 
		name: delete_title,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/title",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get ChartAxisTitle", 
		name: get_title,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/title",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update chartaxistitle", 
		name: update_title,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/title",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property format for drives", 
		name: delete_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/title/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get format from drives", 
		name: get_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/title/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property format in drives", 
		name: update_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/title/format",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property font for drives", 
		name: delete_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/title/format/font",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get font from drives", 
		name: get_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/title/format/font",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property font in drives", 
		name: update_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/seriesAxis/title/format/font",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property valueAxis for drives", 
		name: delete_value_axis,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get ChartAxis", 
		name: get_value_axis,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update chartaxis", 
		name: update_value_axis,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property format for drives", 
		name: delete_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get format from drives", 
		name: get_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property format in drives", 
		name: update_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/format",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property font for drives", 
		name: delete_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/format/font",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get ChartFont", 
		name: get_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/format/font",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update chartfont", 
		name: update_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/format/font",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property line for drives", 
		name: delete_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/format/line",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get ChartLineFormat", 
		name: get_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/format/line",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update chartlineformat", 
		name: update_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/format/line",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	post!(
		doc: "Invoke action clear", 
		name: clear,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/format/line/clear",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property majorGridlines for drives", 
		name: delete_major_gridlines,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/majorGridlines",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get majorGridlines from drives", 
		name: get_major_gridlines,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/majorGridlines",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property majorGridlines in drives", 
		name: update_major_gridlines,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/majorGridlines",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property format for drives", 
		name: delete_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/majorGridlines/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get format from drives", 
		name: get_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/majorGridlines/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property format in drives", 
		name: update_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/majorGridlines/format",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property line for drives", 
		name: delete_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/majorGridlines/format/line",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get line from drives", 
		name: get_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/majorGridlines/format/line",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property line in drives", 
		name: update_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/majorGridlines/format/line",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	post!(
		doc: "Invoke action clear", 
		name: clear,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/majorGridlines/format/line/clear",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property minorGridlines for drives", 
		name: delete_minor_gridlines,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/minorGridlines",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get ChartGridlines", 
		name: get_minor_gridlines,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/minorGridlines",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update chartgridlines", 
		name: update_minor_gridlines,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/minorGridlines",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property format for drives", 
		name: delete_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/minorGridlines/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get format from drives", 
		name: get_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/minorGridlines/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property format in drives", 
		name: update_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/minorGridlines/format",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property line for drives", 
		name: delete_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/minorGridlines/format/line",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get line from drives", 
		name: get_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/minorGridlines/format/line",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property line in drives", 
		name: update_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/minorGridlines/format/line",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	post!(
		doc: "Invoke action clear", 
		name: clear,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/minorGridlines/format/line/clear",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property title for drives", 
		name: delete_title,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/title",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get ChartAxisTitle", 
		name: get_title,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/title",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update chartaxistitle", 
		name: update_title,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/title",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property format for drives", 
		name: delete_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/title/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get format from drives", 
		name: get_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/title/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property format in drives", 
		name: update_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/title/format",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property font for drives", 
		name: delete_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/title/format/font",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get font from drives", 
		name: get_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/title/format/font",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property font in drives", 
		name: update_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/axes/valueAxis/title/format/font",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property dataLabels for drives", 
		name: delete_data_labels,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/dataLabels",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get ChartDataLabels", 
		name: get_data_labels,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/dataLabels",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update chartdatalabels", 
		name: update_data_labels,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/dataLabels",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property format for drives", 
		name: delete_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/dataLabels/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get format from drives", 
		name: get_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/dataLabels/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property format in drives", 
		name: update_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/dataLabels/format",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property fill for drives", 
		name: delete_fill,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/dataLabels/format/fill",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get fill from drives", 
		name: get_fill,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/dataLabels/format/fill",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property fill in drives", 
		name: update_fill,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/dataLabels/format/fill",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	post!(
		doc: "Invoke action clear", 
		name: clear,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/dataLabels/format/fill/clear",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	post!(
		doc: "Invoke action setSolidColor", 
		name: set_solid_color,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/dataLabels/format/fill/setSolidColor",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property font for drives", 
		name: delete_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/dataLabels/format/font",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get font from drives", 
		name: get_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/dataLabels/format/font",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property font in drives", 
		name: update_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/dataLabels/format/font",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property format for drives", 
		name: delete_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get format from drives", 
		name: get_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property format in drives", 
		name: update_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/format",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property fill for drives", 
		name: delete_fill,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/format/fill",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get fill from drives", 
		name: get_fill,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/format/fill",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property fill in drives", 
		name: update_fill,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/format/fill",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	post!(
		doc: "Invoke action clear", 
		name: clear,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/format/fill/clear",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	post!(
		doc: "Invoke action setSolidColor", 
		name: set_solid_color,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/format/fill/setSolidColor",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property font for drives", 
		name: delete_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/format/font",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get font from drives", 
		name: get_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/format/font",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property font in drives", 
		name: update_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/format/font",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Invoke function image", 
		name: workbook_chart,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/image()",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Invoke function image", 
		name: workbook_chart,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/image(width={{id5}})",
		params: width
	);
	get!(
		doc: "Invoke function image", 
		name: workbook_chart,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/image(width={{id5}},height={{id6}})",
		params: width, height
	);
	get!(
		doc: "Invoke function image", 
		name: workbook_chart,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/image(width={{id5}},height={{id6}},fittingMode='{{id7}}')",
		params: width, height, fitting_mode
	);
	delete!(
		doc: "Delete navigation property legend for drives", 
		name: delete_legend,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/legend",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get ChartLegend", 
		name: get_legend,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/legend",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update chartlegend", 
		name: update_legend,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/legend",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property format for drives", 
		name: delete_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/legend/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get format from drives", 
		name: get_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/legend/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property format in drives", 
		name: update_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/legend/format",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property fill for drives", 
		name: delete_fill,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/legend/format/fill",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get fill from drives", 
		name: get_fill,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/legend/format/fill",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property fill in drives", 
		name: update_fill,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/legend/format/fill",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	post!(
		doc: "Invoke action clear", 
		name: clear,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/legend/format/fill/clear",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	post!(
		doc: "Invoke action setSolidColor", 
		name: set_solid_color,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/legend/format/fill/setSolidColor",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property font for drives", 
		name: delete_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/legend/format/font",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get font from drives", 
		name: get_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/legend/format/font",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property font in drives", 
		name: update_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/legend/format/font",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	post!(
		doc: "Create ChartSeries", 
		name: create_series,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "List ChartSeriesCollection", 
		name: list_series,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Invoke function count", 
		name: count,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/count()",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Invoke function itemAt", 
		name: item_at,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/itemAt(index={{id5}})",
		params: index
	);
	delete!(
		doc: "Delete navigation property series for drives", 
		name: delete_series,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id
	);
	get!(
		doc: "Get ChartSeries", 
		name: get_series,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id
	);
	patch!(
		doc: "Update chartseries", 
		name: update_series,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id
	);
	delete!(
		doc: "Delete navigation property format for drives", 
		name: delete_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id
	);
	get!(
		doc: "Get format from drives", 
		name: get_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id
	);
	patch!(
		doc: "Update the navigation property format in drives", 
		name: update_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/format",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id
	);
	delete!(
		doc: "Delete navigation property fill for drives", 
		name: delete_fill,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/format/fill",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id
	);
	get!(
		doc: "Get fill from drives", 
		name: get_fill,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/format/fill",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id
	);
	patch!(
		doc: "Update the navigation property fill in drives", 
		name: update_fill,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/format/fill",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id
	);
	post!(
		doc: "Invoke action clear", 
		name: clear,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/format/fill/clear",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id
	);
	post!(
		doc: "Invoke action setSolidColor", 
		name: set_solid_color,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/format/fill/setSolidColor",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id
	);
	delete!(
		doc: "Delete navigation property line for drives", 
		name: delete_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/format/line",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id
	);
	get!(
		doc: "Get line from drives", 
		name: get_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/format/line",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id
	);
	patch!(
		doc: "Update the navigation property line in drives", 
		name: update_line,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/format/line",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id
	);
	post!(
		doc: "Invoke action clear", 
		name: clear,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/format/line/clear",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id
	);
	post!(
		doc: "Create ChartPoints", 
		name: create_points,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/points",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id
	);
	get!(
		doc: "List ChartPointsCollection", 
		name: list_points,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/points",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id
	);
	get!(
		doc: "Invoke function count", 
		name: count,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/points/count()",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id
	);
	get!(
		doc: "Invoke function itemAt", 
		name: item_at,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/points/itemAt(index={{id6}})",
		params: index
	);
	delete!(
		doc: "Delete navigation property points for drives", 
		name: delete_points,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/points/{{id6}}",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id, workbook_chart_point_id
	);
	get!(
		doc: "Get ChartPoint", 
		name: get_points,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/points/{{id6}}",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id, workbook_chart_point_id
	);
	patch!(
		doc: "Update the navigation property points in drives", 
		name: update_points,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/points/{{id6}}",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id, workbook_chart_point_id
	);
	delete!(
		doc: "Delete navigation property format for drives", 
		name: delete_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/points/{{id6}}/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id, workbook_chart_point_id
	);
	get!(
		doc: "Get format from drives", 
		name: get_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/points/{{id6}}/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id, workbook_chart_point_id
	);
	patch!(
		doc: "Update the navigation property format in drives", 
		name: update_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/points/{{id6}}/format",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id, workbook_chart_point_id
	);
	delete!(
		doc: "Delete navigation property fill for drives", 
		name: delete_fill,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/points/{{id6}}/format/fill",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id, workbook_chart_point_id
	);
	get!(
		doc: "Get fill from drives", 
		name: get_fill,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/points/{{id6}}/format/fill",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id, workbook_chart_point_id
	);
	patch!(
		doc: "Update the navigation property fill in drives", 
		name: update_fill,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/points/{{id6}}/format/fill",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id, workbook_chart_point_id
	);
	post!(
		doc: "Invoke action clear", 
		name: clear,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/points/{{id6}}/format/fill/clear",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id, workbook_chart_point_id
	);
	post!(
		doc: "Invoke action setSolidColor", 
		name: set_solid_color,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/series/{{id5}}/points/{{id6}}/format/fill/setSolidColor",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id, workbook_chart_series_id, workbook_chart_point_id
	);
	post!(
		doc: "Invoke action setData", 
		name: set_data,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/setData",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	post!(
		doc: "Invoke action setPosition", 
		name: set_position,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/setPosition",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property title for drives", 
		name: delete_title,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/title",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get ChartTitle", 
		name: get_title,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/title",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update charttitle", 
		name: update_title,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/title",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property format for drives", 
		name: delete_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/title/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get format from drives", 
		name: get_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/title/format",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property format in drives", 
		name: update_format,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/title/format",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property fill for drives", 
		name: delete_fill,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/title/format/fill",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get fill from drives", 
		name: get_fill,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/title/format/fill",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property fill in drives", 
		name: update_fill,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/title/format/fill",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	post!(
		doc: "Invoke action clear", 
		name: clear,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/title/format/fill/clear",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	post!(
		doc: "Invoke action setSolidColor", 
		name: set_solid_color,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/title/format/fill/setSolidColor",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	delete!(
		doc: "Delete navigation property font for drives", 
		name: delete_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/title/format/font",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get font from drives", 
		name: get_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/title/format/font",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	patch!(
		doc: "Update the navigation property font in drives", 
		name: update_font,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/title/format/font",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	get!(
		doc: "Get worksheet from drives", 
		name: get_worksheet,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/charts/{{id4}}/worksheet",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_chart_id
	);
	post!(
		doc: "Create new navigation property to names for drives", 
		name: create_names,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/names",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	get!(
		doc: "List names", 
		name: list_names,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/names",
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: names,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/names/$count",
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	post!(
		doc: "Invoke action add", 
		name: add,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/names/add",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	post!(
		doc: "Invoke action addFormulaLocal", 
		name: add_formula_local,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/names/addFormulaLocal",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	delete!(
		doc: "Delete navigation property names for drives", 
		name: delete_names,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/names/{{id4}}",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_named_item_id
	);
	get!(
		doc: "Get names from drives", 
		name: get_names,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/names/{{id4}}",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_named_item_id
	);
	patch!(
		doc: "Update the navigation property names in drives", 
		name: update_names,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/names/{{id4}}",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_named_item_id
	);
	get!(
		doc: "Invoke function range", 
		name: range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/names/{{id4}}/range()",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_named_item_id
	);
	get!(
		doc: "Get worksheet from drives", 
		name: get_worksheet,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/names/{{id4}}/worksheet",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_named_item_id
	);
	post!(
		doc: "Create new navigation property to pivotTables for drives", 
		name: create_pivot_tables,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/pivotTables",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	get!(
		doc: "List pivotTables", 
		name: list_pivot_tables,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/pivotTables",
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: pivot_tables,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/pivotTables/$count",
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	post!(
		doc: "Invoke action refreshAll", 
		name: refresh_all,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/pivotTables/refreshAll",
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	delete!(
		doc: "Delete navigation property pivotTables for drives", 
		name: delete_pivot_tables,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/pivotTables/{{id4}}",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_pivot_table_id
	);
	get!(
		doc: "Get workbookPivotTable", 
		name: get_pivot_tables,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/pivotTables/{{id4}}",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_pivot_table_id
	);
	patch!(
		doc: "Update the navigation property pivotTables in drives", 
		name: update_pivot_tables,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/pivotTables/{{id4}}",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_pivot_table_id
	);
	post!(
		doc: "Invoke action refresh", 
		name: refresh,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/pivotTables/{{id4}}/refresh",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_pivot_table_id
	);
	get!(
		doc: "Get worksheet from drives", 
		name: get_worksheet,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/pivotTables/{{id4}}/worksheet",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_pivot_table_id
	);
	delete!(
		doc: "Delete navigation property protection for drives", 
		name: delete_protection,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/protection",
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	get!(
		doc: "Get WorksheetProtection", 
		name: get_protection,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/protection",
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	patch!(
		doc: "Update the navigation property protection in drives", 
		name: update_protection,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/protection",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	post!(
		doc: "Invoke action protect", 
		name: protect,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/protection/protect",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	post!(
		doc: "Invoke action unprotect", 
		name: unprotect,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/protection/unprotect",
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	get!(
		doc: "Invoke function range", 
		name: workbook_worksheet,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/range()",
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	get!(
		doc: "Invoke function range", 
		name: workbook_worksheet,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/range(address='{{id4}}')",
		params: address
	);
	post!(
		doc: "Create new navigation property to tables for drives", 
		name: create_tables,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	get!(
		doc: "List tables", 
		name: list_tables,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables",
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	post!(
		doc: "Invoke action add", 
		name: add,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/add",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	get!(
		doc: "Invoke function count", 
		name: count,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/count()",
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	get!(
		doc: "Invoke function itemAt", 
		name: item_at,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/itemAt(index={{id4}})",
		params: index
	);
	delete!(
		doc: "Delete navigation property tables for drives", 
		name: delete_tables,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	get!(
		doc: "Get tables from drives", 
		name: get_tables,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	patch!(
		doc: "Update the navigation property tables in drives", 
		name: update_tables,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	post!(
		doc: "Invoke action clearFilters", 
		name: clear_filters,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/clearFilters",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	post!(
		doc: "Create TableColumn", 
		name: create_columns,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	get!(
		doc: "List TableColumnCollection", 
		name: list_columns,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	post!(
		doc: "Invoke action add", 
		name: add,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/add",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	get!(
		doc: "Invoke function count", 
		name: count,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/count()",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	get!(
		doc: "Invoke function itemAt", 
		name: item_at,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/itemAt(index={{id5}})",
		params: index
	);
	get!(
		doc: "Get TableColumn", 
		name: get_columns,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	delete!(
		doc: "TableColumn: delete", 
		name: delete_columns,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	patch!(
		doc: "Update tablecolumn", 
		name: update_columns,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	get!(
		doc: "Invoke function dataBodyRange", 
		name: data_body_range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}/dataBodyRange()",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	delete!(
		doc: "Delete navigation property filter for drives", 
		name: delete_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}/filter",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	get!(
		doc: "Get filter from drives", 
		name: get_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}/filter",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	patch!(
		doc: "Update the navigation property filter in drives", 
		name: update_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}/filter",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action apply", 
		name: apply,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}/filter/apply",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action applyBottomItemsFilter", 
		name: apply_bottom_items_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}/filter/applyBottomItemsFilter",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action applyBottomPercentFilter", 
		name: apply_bottom_percent_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}/filter/applyBottomPercentFilter",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action applyCellColorFilter", 
		name: apply_cell_color_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}/filter/applyCellColorFilter",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action applyCustomFilter", 
		name: apply_custom_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}/filter/applyCustomFilter",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action applyDynamicFilter", 
		name: apply_dynamic_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}/filter/applyDynamicFilter",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action applyFontColorFilter", 
		name: apply_font_color_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}/filter/applyFontColorFilter",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action applyIconFilter", 
		name: apply_icon_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}/filter/applyIconFilter",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action applyTopItemsFilter", 
		name: apply_top_items_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}/filter/applyTopItemsFilter",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action applyTopPercentFilter", 
		name: apply_top_percent_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}/filter/applyTopPercentFilter",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action applyValuesFilter", 
		name: apply_values_filter,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}/filter/applyValuesFilter",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action clear", 
		name: clear,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}/filter/clear",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	get!(
		doc: "Invoke function headerRowRange", 
		name: header_row_range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}/headerRowRange()",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	get!(
		doc: "Invoke function range", 
		name: range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}/range()",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	get!(
		doc: "Invoke function totalRowRange", 
		name: total_row_range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/columns/{{id5}}/totalRowRange()",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_column_id
	);
	post!(
		doc: "Invoke action convertToRange", 
		name: convert_to_range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/convertToRange",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	get!(
		doc: "Invoke function dataBodyRange", 
		name: data_body_range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/dataBodyRange()",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	get!(
		doc: "Invoke function headerRowRange", 
		name: header_row_range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/headerRowRange()",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	get!(
		doc: "Invoke function range", 
		name: range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/range()",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	post!(
		doc: "Invoke action reapplyFilters", 
		name: reapply_filters,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/reapplyFilters",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	post!(
		doc: "Create TableRow", 
		name: create_rows,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/rows",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	get!(
		doc: "List TableRowCollection", 
		name: list_rows,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/rows",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	post!(
		doc: "Invoke action add", 
		name: add,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/rows/add",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	get!(
		doc: "Invoke function count", 
		name: count,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/rows/count()",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	get!(
		doc: "Invoke function itemAt", 
		name: item_at,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/rows/itemAt(index={{id5}})",
		params: index
	);
	get!(
		doc: "Get TableRow", 
		name: get_rows,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/rows/{{id5}}",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_row_id
	);
	delete!(
		doc: "TableRow: delete", 
		name: delete_rows,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/rows/{{id5}}",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_row_id
	);
	patch!(
		doc: "Update tablerow", 
		name: update_rows,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/rows/{{id5}}",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_row_id
	);
	get!(
		doc: "Invoke function range", 
		name: range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/rows/{{id5}}/range()",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id, workbook_table_row_id
	);
	delete!(
		doc: "Delete navigation property sort for drives", 
		name: delete_sort,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/sort",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	get!(
		doc: "Get TableSort", 
		name: get_sort,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/sort",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	patch!(
		doc: "Update the navigation property sort in drives", 
		name: update_sort,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/sort",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	post!(
		doc: "Invoke action apply", 
		name: apply,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/sort/apply",
		body: true,
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	post!(
		doc: "Invoke action clear", 
		name: clear,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/sort/clear",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	post!(
		doc: "Invoke action reapply", 
		name: reapply,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/sort/reapply",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	get!(
		doc: "Invoke function totalRowRange", 
		name: total_row_range,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/totalRowRange()",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	get!(
		doc: "Get worksheet from drives", 
		name: get_worksheet,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/tables/{{id4}}/worksheet",
		params: drive_id, drive_item_id, workbook_worksheet_id, workbook_table_id
	);
	get!(
		doc: "Invoke function usedRange", 
		name: workbook_worksheet,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/usedRange()",
		params: drive_id, drive_item_id, workbook_worksheet_id
	);
	get!(
		doc: "Invoke function usedRange", 
		name: workbook_worksheet,
		path: "/drives/{{id}}/items/{{id2}}/workbook/worksheets/{{id3}}/usedRange(valuesOnly={{id4}})",
		params: values_only
	);
	get!(
		doc: "Get lastModifiedByUser from drives", 
		name: get_last_modified_by_user,
		path: "/drives/{{id}}/lastModifiedByUser",
		params: drive_id
	);
	get!(
		doc: "Get mailboxSettings property value", 
		name: get_mailbox_settings,
		path: "/drives/{{id}}/lastModifiedByUser/mailboxSettings",
		params: drive_id
	);
	patch!(
		doc: "Update property mailboxSettings value.", 
		name: update_mailbox_settings,
		path: "/drives/{{id}}/lastModifiedByUser/mailboxSettings",
		body: true,
		params: drive_id
	);
	get!(
		doc: "Get serviceProvisioningErrors property value", 
		name: list_service_provisioning_errors,
		path: "/drives/{{id}}/lastModifiedByUser/serviceProvisioningErrors",
		params: drive_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: service_provisioning_errors,
		path: "/drives/{{id}}/lastModifiedByUser/serviceProvisioningErrors/$count",
		params: drive_id
	);
	delete!(
		doc: "Delete navigation property list for drives", 
		name: delete_list,
		path: "/drives/{{id}}/list",
		params: drive_id
	);
	get!(
		doc: "Get list from drives", 
		name: get_list,
		path: "/drives/{{id}}/list",
		params: drive_id
	);
	patch!(
		doc: "Update the navigation property list in drives", 
		name: update_list,
		path: "/drives/{{id}}/list",
		body: true,
		params: drive_id
	);
	post!(
		doc: "Create a columnDefinition in a list", 
		name: create_columns,
		path: "/drives/{{id}}/list/columns",
		body: true,
		params: drive_id
	);
	get!(
		doc: "List columnDefinitions in a list", 
		name: list_columns,
		path: "/drives/{{id}}/list/columns",
		params: drive_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: columns,
		path: "/drives/{{id}}/list/columns/$count",
		params: drive_id
	);
	delete!(
		doc: "Delete navigation property columns for drives", 
		name: delete_columns,
		path: "/drives/{{id}}/list/columns/{{id2}}",
		params: drive_id, column_definition_id
	);
	get!(
		doc: "Get columns from drives", 
		name: get_columns,
		path: "/drives/{{id}}/list/columns/{{id2}}",
		params: drive_id, column_definition_id
	);
	patch!(
		doc: "Update the navigation property columns in drives", 
		name: update_columns,
		path: "/drives/{{id}}/list/columns/{{id2}}",
		body: true,
		params: drive_id, column_definition_id
	);
	get!(
		doc: "Get sourceColumn from drives", 
		name: get_source_column,
		path: "/drives/{{id}}/list/columns/{{id2}}/sourceColumn",
		params: drive_id, column_definition_id
	);
	post!(
		doc: "Create new navigation property to contentTypes for drives", 
		name: create_content_types,
		path: "/drives/{{id}}/list/contentTypes",
		body: true,
		params: drive_id
	);
	get!(
		doc: "List contentTypes in a list", 
		name: list_content_types,
		path: "/drives/{{id}}/list/contentTypes",
		params: drive_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: content_types,
		path: "/drives/{{id}}/list/contentTypes/$count",
		params: drive_id
	);
	post!(
		doc: "Invoke action addCopy", 
		name: add_copy,
		path: "/drives/{{id}}/list/contentTypes/addCopy",
		body: true,
		params: drive_id
	);
	post!(
		doc: "Invoke action addCopyFromContentTypeHub", 
		name: add_copy_from_content_type_hub,
		path: "/drives/{{id}}/list/contentTypes/addCopyFromContentTypeHub",
		body: true,
		params: drive_id
	);
	get!(
		doc: "Invoke function getCompatibleHubContentTypes", 
		name: get_compatible_hub_content_types,
		path: "/drives/{{id}}/list/contentTypes/getCompatibleHubContentTypes()",
		params: drive_id
	);
	delete!(
		doc: "Delete navigation property contentTypes for drives", 
		name: delete_content_types,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}",
		params: drive_id, content_type_id
	);
	get!(
		doc: "Get contentTypes from drives", 
		name: get_content_types,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}",
		params: drive_id, content_type_id
	);
	patch!(
		doc: "Update the navigation property contentTypes in drives", 
		name: update_content_types,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}",
		body: true,
		params: drive_id, content_type_id
	);
	post!(
		doc: "Invoke action associateWithHubSites", 
		name: associate_with_hub_sites,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/associateWithHubSites",
		body: true,
		params: drive_id, content_type_id
	);
	get!(
		doc: "Get base from drives", 
		name: get_base,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/base",
		params: drive_id, content_type_id
	);
	get!(
		doc: "Get baseTypes from drives", 
		name: list_base_types,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/baseTypes",
		params: drive_id, content_type_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: base_types,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/baseTypes/$count",
		params: drive_id, content_type_id
	);
	get!(
		doc: "Get baseTypes from drives", 
		name: get_base_types,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/baseTypes/{{id3}}",
		params: drive_id, content_type_id, content_type_id_1
	);
	post!(
		doc: "Create new navigation property to columnLinks for drives", 
		name: create_column_links,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/columnLinks",
		body: true,
		params: drive_id, content_type_id
	);
	get!(
		doc: "Get columnLinks from drives", 
		name: list_column_links,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/columnLinks",
		params: drive_id, content_type_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: column_links,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/columnLinks/$count",
		params: drive_id, content_type_id
	);
	delete!(
		doc: "Delete navigation property columnLinks for drives", 
		name: delete_column_links,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/columnLinks/{{id3}}",
		params: drive_id, content_type_id, column_link_id
	);
	get!(
		doc: "Get columnLinks from drives", 
		name: get_column_links,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/columnLinks/{{id3}}",
		params: drive_id, content_type_id, column_link_id
	);
	patch!(
		doc: "Update the navigation property columnLinks in drives", 
		name: update_column_links,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/columnLinks/{{id3}}",
		body: true,
		params: drive_id, content_type_id, column_link_id
	);
	get!(
		doc: "Get columnPositions from drives", 
		name: list_column_positions,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/columnPositions",
		params: drive_id, content_type_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: column_positions,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/columnPositions/$count",
		params: drive_id, content_type_id
	);
	get!(
		doc: "Get columnPositions from drives", 
		name: get_column_positions,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/columnPositions/{{id3}}",
		params: drive_id, content_type_id, column_definition_id
	);
	post!(
		doc: "Create a columnDefinition in a content type", 
		name: create_columns,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/columns",
		body: true,
		params: drive_id, content_type_id
	);
	get!(
		doc: "List columnDefinitions in a content type", 
		name: list_columns,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/columns",
		params: drive_id, content_type_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: columns,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/columns/$count",
		params: drive_id, content_type_id
	);
	delete!(
		doc: "Delete columnDefinition", 
		name: delete_columns,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/columns/{{id3}}",
		params: drive_id, content_type_id, column_definition_id
	);
	get!(
		doc: "Get columnDefinition", 
		name: get_columns,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/columns/{{id3}}",
		params: drive_id, content_type_id, column_definition_id
	);
	patch!(
		doc: "Update columnDefinition", 
		name: update_columns,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/columns/{{id3}}",
		body: true,
		params: drive_id, content_type_id, column_definition_id
	);
	get!(
		doc: "Get sourceColumn from drives", 
		name: get_source_column,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/columns/{{id3}}/sourceColumn",
		params: drive_id, content_type_id, column_definition_id
	);
	post!(
		doc: "Invoke action copyToDefaultContentLocation", 
		name: copy_to_default_content_location,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/copyToDefaultContentLocation",
		body: true,
		params: drive_id, content_type_id
	);
	get!(
		doc: "Invoke function isPublished", 
		name: is_published,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/isPublished()",
		params: drive_id, content_type_id
	);
	post!(
		doc: "Invoke action publish", 
		name: publish,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/publish",
		params: drive_id, content_type_id
	);
	post!(
		doc: "Invoke action unpublish", 
		name: unpublish,
		path: "/drives/{{id}}/list/contentTypes/{{id2}}/unpublish",
		params: drive_id, content_type_id
	);
	get!(
		doc: "Get createdByUser from drives", 
		name: get_created_by_user,
		path: "/drives/{{id}}/list/createdByUser",
		params: drive_id
	);
	get!(
		doc: "Get mailboxSettings property value", 
		name: get_mailbox_settings,
		path: "/drives/{{id}}/list/createdByUser/mailboxSettings",
		params: drive_id
	);
	patch!(
		doc: "Update property mailboxSettings value.", 
		name: update_mailbox_settings,
		path: "/drives/{{id}}/list/createdByUser/mailboxSettings",
		body: true,
		params: drive_id
	);
	get!(
		doc: "Get serviceProvisioningErrors property value", 
		name: list_service_provisioning_errors,
		path: "/drives/{{id}}/list/createdByUser/serviceProvisioningErrors",
		params: drive_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: service_provisioning_errors,
		path: "/drives/{{id}}/list/createdByUser/serviceProvisioningErrors/$count",
		params: drive_id
	);
	get!(
		doc: "Get drive from drives", 
		name: get_drive,
		path: "/drives/{{id}}/list/drive",
		params: drive_id
	);
	post!(
		doc: "Create a new item in a list", 
		name: create_items,
		path: "/drives/{{id}}/list/items",
		body: true,
		params: drive_id
	);
	get!(
		doc: "Enumerate items in a list", 
		name: list_items,
		path: "/drives/{{id}}/list/items",
		params: drive_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: items,
		path: "/drives/{{id}}/list/items/$count",
		params: drive_id
	);
	delete!(
		doc: "Delete an item from a list", 
		name: delete_items,
		path: "/drives/{{id}}/list/items/{{id2}}",
		params: drive_id, list_item_id
	);
	get!(
		doc: "Get listItem", 
		name: get_items,
		path: "/drives/{{id}}/list/items/{{id2}}",
		params: drive_id, list_item_id
	);
	patch!(
		doc: "Update the navigation property items in drives", 
		name: update_items,
		path: "/drives/{{id}}/list/items/{{id2}}",
		body: true,
		params: drive_id, list_item_id
	);
	get!(
		doc: "Get analytics from drives", 
		name: get_analytics,
		path: "/drives/{{id}}/list/items/{{id2}}/analytics",
		params: drive_id, list_item_id
	);
	post!(
		doc: "Invoke action createLink", 
		name: create_link,
		path: "/drives/{{id}}/list/items/{{id2}}/createLink",
		body: true,
		params: drive_id, list_item_id
	);
	get!(
		doc: "Get createdByUser from drives", 
		name: get_created_by_user,
		path: "/drives/{{id}}/list/items/{{id2}}/createdByUser",
		params: drive_id, list_item_id
	);
	get!(
		doc: "Get mailboxSettings property value", 
		name: get_mailbox_settings,
		path: "/drives/{{id}}/list/items/{{id2}}/createdByUser/mailboxSettings",
		params: drive_id, list_item_id
	);
	patch!(
		doc: "Update property mailboxSettings value.", 
		name: update_mailbox_settings,
		path: "/drives/{{id}}/list/items/{{id2}}/createdByUser/mailboxSettings",
		body: true,
		params: drive_id, list_item_id
	);
	get!(
		doc: "Get serviceProvisioningErrors property value", 
		name: list_service_provisioning_errors,
		path: "/drives/{{id}}/list/items/{{id2}}/createdByUser/serviceProvisioningErrors",
		params: drive_id, list_item_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: service_provisioning_errors,
		path: "/drives/{{id}}/list/items/{{id2}}/createdByUser/serviceProvisioningErrors/$count",
		params: drive_id, list_item_id
	);
	post!(
		doc: "Create documentSetVersion", 
		name: create_document_set_versions,
		path: "/drives/{{id}}/list/items/{{id2}}/documentSetVersions",
		body: true,
		params: drive_id, list_item_id
	);
	get!(
		doc: "List documentSetVersions", 
		name: list_document_set_versions,
		path: "/drives/{{id}}/list/items/{{id2}}/documentSetVersions",
		params: drive_id, list_item_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: document_set_versions,
		path: "/drives/{{id}}/list/items/{{id2}}/documentSetVersions/$count",
		params: drive_id, list_item_id
	);
	delete!(
		doc: "Delete documentSetVersion", 
		name: delete_document_set_versions,
		path: "/drives/{{id}}/list/items/{{id2}}/documentSetVersions/{{id3}}",
		params: drive_id, list_item_id, document_set_version_id
	);
	get!(
		doc: "Get documentSetVersion", 
		name: get_document_set_versions,
		path: "/drives/{{id}}/list/items/{{id2}}/documentSetVersions/{{id3}}",
		params: drive_id, list_item_id, document_set_version_id
	);
	patch!(
		doc: "Update the navigation property documentSetVersions in drives", 
		name: update_document_set_versions,
		path: "/drives/{{id}}/list/items/{{id2}}/documentSetVersions/{{id3}}",
		body: true,
		params: drive_id, list_item_id, document_set_version_id
	);
	delete!(
		doc: "Delete navigation property fields for drives", 
		name: delete_fields,
		path: "/drives/{{id}}/list/items/{{id2}}/documentSetVersions/{{id3}}/fields",
		params: drive_id, list_item_id, document_set_version_id
	);
	get!(
		doc: "Get fields from drives", 
		name: get_fields,
		path: "/drives/{{id}}/list/items/{{id2}}/documentSetVersions/{{id3}}/fields",
		params: drive_id, list_item_id, document_set_version_id
	);
	patch!(
		doc: "Update the navigation property fields in drives", 
		name: update_fields,
		path: "/drives/{{id}}/list/items/{{id2}}/documentSetVersions/{{id3}}/fields",
		body: true,
		params: drive_id, list_item_id, document_set_version_id
	);
	post!(
		doc: "Invoke action restore", 
		name: restore,
		path: "/drives/{{id}}/list/items/{{id2}}/documentSetVersions/{{id3}}/restore",
		params: drive_id, list_item_id, document_set_version_id
	);
	get!(
		doc: "Get driveItem from drives", 
		name: get_drive_item,
		path: "/drives/{{id}}/list/items/{{id2}}/driveItem",
		params: drive_id, list_item_id
	);
	get!(
		doc: "Get content for the navigation property driveItem from drives", 
		name: get_drive_item_content,
		path: "/drives/{{id}}/list/items/{{id2}}/driveItem/content",
		params: drive_id, list_item_id
	);
	put!(
		doc: "Update content for the navigation property driveItem in drives", 
		name: update_drive_item_content,
		path: "/drives/{{id}}/list/items/{{id2}}/driveItem/content",
		body: true,
		params: drive_id, list_item_id
	);
	delete!(
		doc: "Delete navigation property fields for drives", 
		name: delete_fields,
		path: "/drives/{{id}}/list/items/{{id2}}/fields",
		params: drive_id, list_item_id
	);
	get!(
		doc: "Get fields from drives", 
		name: get_fields,
		path: "/drives/{{id}}/list/items/{{id2}}/fields",
		params: drive_id, list_item_id
	);
	patch!(
		doc: "Update listItem", 
		name: update_fields,
		path: "/drives/{{id}}/list/items/{{id2}}/fields",
		body: true,
		params: drive_id, list_item_id
	);
	get!(
		doc: "Invoke function getActivitiesByInterval", 
		name: list_item,
		path: "/drives/{{id}}/list/items/{{id2}}/getActivitiesByInterval()",
		params: drive_id, list_item_id
	);
	get!(
		doc: "Invoke function getActivitiesByInterval", 
		name: list_item,
		path: "/drives/{{id}}/list/items/{{id2}}/getActivitiesByInterval(startDateTime='{{id3}}',endDateTime='{{id4}}',interval='{{id5}}')",
		params: start_date_time, end_date_time, interval
	);
	get!(
		doc: "Get lastModifiedByUser from drives", 
		name: get_last_modified_by_user,
		path: "/drives/{{id}}/list/items/{{id2}}/lastModifiedByUser",
		params: drive_id, list_item_id
	);
	get!(
		doc: "Get mailboxSettings property value", 
		name: get_mailbox_settings,
		path: "/drives/{{id}}/list/items/{{id2}}/lastModifiedByUser/mailboxSettings",
		params: drive_id, list_item_id
	);
	patch!(
		doc: "Update property mailboxSettings value.", 
		name: update_mailbox_settings,
		path: "/drives/{{id}}/list/items/{{id2}}/lastModifiedByUser/mailboxSettings",
		body: true,
		params: drive_id, list_item_id
	);
	get!(
		doc: "Get serviceProvisioningErrors property value", 
		name: list_service_provisioning_errors,
		path: "/drives/{{id}}/list/items/{{id2}}/lastModifiedByUser/serviceProvisioningErrors",
		params: drive_id, list_item_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: service_provisioning_errors,
		path: "/drives/{{id}}/list/items/{{id2}}/lastModifiedByUser/serviceProvisioningErrors/$count",
		params: drive_id, list_item_id
	);
	post!(
		doc: "Create new navigation property to versions for drives", 
		name: create_versions,
		path: "/drives/{{id}}/list/items/{{id2}}/versions",
		body: true,
		params: drive_id, list_item_id
	);
	get!(
		doc: "Listing versions of a ListItem", 
		name: list_versions,
		path: "/drives/{{id}}/list/items/{{id2}}/versions",
		params: drive_id, list_item_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: versions,
		path: "/drives/{{id}}/list/items/{{id2}}/versions/$count",
		params: drive_id, list_item_id
	);
	delete!(
		doc: "Delete navigation property versions for drives", 
		name: delete_versions,
		path: "/drives/{{id}}/list/items/{{id2}}/versions/{{id3}}",
		params: drive_id, list_item_id, list_item_version_id
	);
	get!(
		doc: "Get a ListItemVersion resource", 
		name: get_versions,
		path: "/drives/{{id}}/list/items/{{id2}}/versions/{{id3}}",
		params: drive_id, list_item_id, list_item_version_id
	);
	patch!(
		doc: "Update the navigation property versions in drives", 
		name: update_versions,
		path: "/drives/{{id}}/list/items/{{id2}}/versions/{{id3}}",
		body: true,
		params: drive_id, list_item_id, list_item_version_id
	);
	delete!(
		doc: "Delete navigation property fields for drives", 
		name: delete_fields,
		path: "/drives/{{id}}/list/items/{{id2}}/versions/{{id3}}/fields",
		params: drive_id, list_item_id, list_item_version_id
	);
	get!(
		doc: "Get fields from drives", 
		name: get_fields,
		path: "/drives/{{id}}/list/items/{{id2}}/versions/{{id3}}/fields",
		params: drive_id, list_item_id, list_item_version_id
	);
	patch!(
		doc: "Update the navigation property fields in drives", 
		name: update_fields,
		path: "/drives/{{id}}/list/items/{{id2}}/versions/{{id3}}/fields",
		body: true,
		params: drive_id, list_item_id, list_item_version_id
	);
	post!(
		doc: "Invoke action restoreVersion", 
		name: restore_version,
		path: "/drives/{{id}}/list/items/{{id2}}/versions/{{id3}}/restoreVersion",
		params: drive_id, list_item_id, list_item_version_id
	);
	get!(
		doc: "Get lastModifiedByUser from drives", 
		name: get_last_modified_by_user,
		path: "/drives/{{id}}/list/lastModifiedByUser",
		params: drive_id
	);
	get!(
		doc: "Get mailboxSettings property value", 
		name: get_mailbox_settings,
		path: "/drives/{{id}}/list/lastModifiedByUser/mailboxSettings",
		params: drive_id
	);
	patch!(
		doc: "Update property mailboxSettings value.", 
		name: update_mailbox_settings,
		path: "/drives/{{id}}/list/lastModifiedByUser/mailboxSettings",
		body: true,
		params: drive_id
	);
	get!(
		doc: "Get serviceProvisioningErrors property value", 
		name: list_service_provisioning_errors,
		path: "/drives/{{id}}/list/lastModifiedByUser/serviceProvisioningErrors",
		params: drive_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: service_provisioning_errors,
		path: "/drives/{{id}}/list/lastModifiedByUser/serviceProvisioningErrors/$count",
		params: drive_id
	);
	post!(
		doc: "Create new navigation property to operations for drives", 
		name: create_operations,
		path: "/drives/{{id}}/list/operations",
		body: true,
		params: drive_id
	);
	get!(
		doc: "Get operations from drives", 
		name: list_operations,
		path: "/drives/{{id}}/list/operations",
		params: drive_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: operations,
		path: "/drives/{{id}}/list/operations/$count",
		params: drive_id
	);
	delete!(
		doc: "Delete navigation property operations for drives", 
		name: delete_operations,
		path: "/drives/{{id}}/list/operations/{{id2}}",
		params: drive_id, rich_long_running_operation_id
	);
	get!(
		doc: "Get operations from drives", 
		name: get_operations,
		path: "/drives/{{id}}/list/operations/{{id2}}",
		params: drive_id, rich_long_running_operation_id
	);
	patch!(
		doc: "Update the navigation property operations in drives", 
		name: update_operations,
		path: "/drives/{{id}}/list/operations/{{id2}}",
		body: true,
		params: drive_id, rich_long_running_operation_id
	);
	post!(
		doc: "Create new navigation property to subscriptions for drives", 
		name: create_subscriptions,
		path: "/drives/{{id}}/list/subscriptions",
		body: true,
		params: drive_id
	);
	get!(
		doc: "Get subscriptions from drives", 
		name: list_subscriptions,
		path: "/drives/{{id}}/list/subscriptions",
		params: drive_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: subscriptions,
		path: "/drives/{{id}}/list/subscriptions/$count",
		params: drive_id
	);
	delete!(
		doc: "Delete navigation property subscriptions for drives", 
		name: delete_subscriptions,
		path: "/drives/{{id}}/list/subscriptions/{{id2}}",
		params: drive_id, subscription_id
	);
	get!(
		doc: "Get subscriptions from drives", 
		name: get_subscriptions,
		path: "/drives/{{id}}/list/subscriptions/{{id2}}",
		params: drive_id, subscription_id
	);
	patch!(
		doc: "Update the navigation property subscriptions in drives", 
		name: update_subscriptions,
		path: "/drives/{{id}}/list/subscriptions/{{id2}}",
		body: true,
		params: drive_id, subscription_id
	);
	post!(
		doc: "Invoke action reauthorize", 
		name: reauthorize,
		path: "/drives/{{id}}/list/subscriptions/{{id2}}/reauthorize",
		params: drive_id, subscription_id
	);
	get!(
		doc: "Invoke function recent", 
		name: recent,
		path: "/drives/{{id}}/recent()",
		params: drive_id
	);
	get!(
		doc: "Get a driveItem resource", 
		name: get_root,
		path: "/drives/{{id}}/root",
		params: drive_id
	);
	get!(
		doc: "Get content for the navigation property root from drives", 
		name: get_root_content,
		path: "/drives/{{id}}/root/content",
		params: drive_id
	);
	put!(
		doc: "Update content for the navigation property root in drives", 
		name: update_root_content,
		path: "/drives/{{id}}/root/content",
		body: true,
		params: drive_id
	);
	get!(
		doc: "Invoke function search", 
		name: search,
		path: "/drives/{{id}}/search(q='{{id2}}')",
		params: q
	);
	get!(
		doc: "Invoke function sharedWithMe", 
		name: shared_with_me,
		path: "/drives/{{id}}/sharedWithMe()",
		params: drive_id
	);
	get!(
		doc: "Get a special folder by name", 
		name: list_special,
		path: "/drives/{{id}}/special",
		params: drive_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: special,
		path: "/drives/{{id}}/special/$count",
		params: drive_id
	);
	get!(
		doc: "Get a special folder by name", 
		name: get_special,
		path: "/drives/{{id}}/special/{{id2}}",
		params: drive_id, drive_item_id
	);
	get!(
		doc: "Get content for the navigation property special from drives", 
		name: get_special_content,
		path: "/drives/{{id}}/special/{{id2}}/content",
		params: drive_id, drive_item_id
	);
	put!(
		doc: "Update content for the navigation property special in drives", 
		name: update_special_content,
		path: "/drives/{{id}}/special/{{id2}}/content",
		body: true,
		params: drive_id, drive_item_id
	);
}
