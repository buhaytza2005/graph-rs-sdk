// GENERATED CODE

use crate::api_default_imports::*;
use crate::sites::*;

resource_api_client!(SitesItemsApiClient, SitesItemsIdApiClient, ResourceIdentity::SitesItems);

impl SitesItemsApiClient {
	post!(
		doc: "Create a new item in a list", 
		name: create_items,
		path: "/items",
		body: true
	);
	get!(
		doc: "Enumerate items in a list", 
		name: list_items,
		path: "/items"
	);
}

impl SitesItemsIdApiClient {api_client_link_id!(version, SitesItemsVersionsIdApiClient);
api_client_link!(versions, SitesItemsVersionsApiClient);

	delete!(
		doc: "Delete an item from a list", 
		name: delete_items,
		path: "/items/{{RID}}"
	);
	get!(
		doc: "Get listItem", 
		name: get_items,
		path: "/items/{{RID}}"
	);
	patch!(
		doc: "Update the navigation property items in sites", 
		name: update_items,
		path: "/items/{{RID}}",
		body: true
	);
	get!(
		doc: "Get analytics from sites", 
		name: get_analytics,
		path: "/items/{{RID}}/analytics"
	);
	post!(
		doc: "Invoke action createLink", 
		name: create_link,
		path: "/items/{{RID}}/createLink",
		body: true
	);
	get!(
		doc: "Get createdByUser from sites", 
		name: get_created_by_user,
		path: "/items/{{RID}}/createdByUser"
	);
	get!(
		doc: "Get mailboxSettings property value", 
		name: get_mailbox_settings,
		path: "/items/{{RID}}/createdByUser/mailboxSettings"
	);
	patch!(
		doc: "Update property mailboxSettings value.", 
		name: update_mailbox_settings,
		path: "/items/{{RID}}/createdByUser/mailboxSettings",
		body: true
	);
	get!(
		doc: "Get serviceProvisioningErrors property value", 
		name: list_service_provisioning_errors,
		path: "/items/{{RID}}/createdByUser/serviceProvisioningErrors"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count_bafa,
		path: "/items/{{RID}}/createdByUser/serviceProvisioningErrors/$count"
	);
	post!(
		doc: "Create documentSetVersion", 
		name: create_document_set_versions,
		path: "/items/{{RID}}/documentSetVersions",
		body: true
	);
	get!(
		doc: "List documentSetVersions", 
		name: list_document_set_versions,
		path: "/items/{{RID}}/documentSetVersions"
	);
	get!(
		doc: "Get the number of the resource", 
		name: document_set_versions,
		path: "/items/{{RID}}/documentSetVersions/$count"
	);
	delete!(
		doc: "Delete documentSetVersion", 
		name: delete_document_set_versions,
		path: "/items/{{RID}}/documentSetVersions/{{id}}",
		params: document_set_version_id
	);
	get!(
		doc: "Get documentSetVersion", 
		name: get_document_set_versions,
		path: "/items/{{RID}}/documentSetVersions/{{id}}",
		params: document_set_version_id
	);
	patch!(
		doc: "Update the navigation property documentSetVersions in sites", 
		name: update_document_set_versions,
		path: "/items/{{RID}}/documentSetVersions/{{id}}",
		body: true,
		params: document_set_version_id
	);
	delete!(
		doc: "Delete navigation property fields for sites", 
		name: delete_fields,
		path: "/items/{{RID}}/documentSetVersions/{{id}}/fields",
		params: document_set_version_id
	);
	get!(
		doc: "Get fields from sites", 
		name: get_fields,
		path: "/items/{{RID}}/documentSetVersions/{{id}}/fields",
		params: document_set_version_id
	);
	patch!(
		doc: "Update the navigation property fields in sites", 
		name: update_fields,
		path: "/items/{{RID}}/documentSetVersions/{{id}}/fields",
		body: true,
		params: document_set_version_id
	);
	post!(
		doc: "Invoke action restore", 
		name: restore,
		path: "/items/{{RID}}/documentSetVersions/{{id}}/restore",
		params: document_set_version_id
	);
	get!(
		doc: "Get driveItem from sites", 
		name: get_drive_item,
		path: "/items/{{RID}}/driveItem"
	);
	get!(
		doc: "Get content for the navigation property driveItem from sites", 
		name: get_drive_item_content,
		path: "/items/{{RID}}/driveItem/content"
	);
	put!(
		doc: "Update content for the navigation property driveItem in sites", 
		name: update_drive_item_content,
		path: "/items/{{RID}}/driveItem/content",
		body: true
	);
	delete!(
		doc: "Delete navigation property fields for sites", 
		name: delete_fields,
		path: "/items/{{RID}}/fields"
	);
	get!(
		doc: "Get fields from sites", 
		name: get_fields,
		path: "/items/{{RID}}/fields"
	);
	patch!(
		doc: "Update listItem", 
		name: update_fields,
		path: "/items/{{RID}}/fields",
		body: true
	);
	get!(
		doc: "Invoke function getActivitiesByInterval", 
		name: get_activities_by_interval,
		path: "/items/{{RID}}/getActivitiesByInterval(startDateTime='{{id}}',endDateTime='{{id2}}',interval='{{id3}}')",
		params: start_date_time, end_date_time, interval
	);
	get!(
		doc: "Get lastModifiedByUser from sites", 
		name: get_last_modified_by_user,
		path: "/items/{{RID}}/lastModifiedByUser"
	);
	get!(
		doc: "Get mailboxSettings property value", 
		name: get_mailbox_settings,
		path: "/items/{{RID}}/lastModifiedByUser/mailboxSettings"
	);
	patch!(
		doc: "Update property mailboxSettings value.", 
		name: update_mailbox_settings,
		path: "/items/{{RID}}/lastModifiedByUser/mailboxSettings",
		body: true
	);
	get!(
		doc: "Get serviceProvisioningErrors property value", 
		name: list_service_provisioning_errors,
		path: "/items/{{RID}}/lastModifiedByUser/serviceProvisioningErrors"
	);
	get!(
		doc: "Get the number of the resource", 
		name: service_provisioning_errors,
		path: "/items/{{RID}}/lastModifiedByUser/serviceProvisioningErrors/$count"
	);
}
