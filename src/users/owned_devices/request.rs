// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(OwnedDevicesApiClient, OwnedDevicesIdApiClient, ResourceIdentity::OwnedDevices);

impl OwnedDevicesApiClient {
	get!(
		doc: "Get ownedDevices from users", 
		name: list_owned_devices,
		path: "/ownedDevices"
	);
	get!(
		doc: "Get the number of the resource", 
		name: owned_devices,
		path: "/ownedDevices/$count"
	);
	get!(
		doc: "Get the items of type microsoft.graph.appRoleAssignment in the microsoft.graph.directoryObject collection", 
		name: as_app_role_assignment,
		path: "/ownedDevices/graph.appRoleAssignment"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count,
		path: "/ownedDevices/graph.appRoleAssignment/$count"
	);
	get!(
		doc: "Get the items of type microsoft.graph.device in the microsoft.graph.directoryObject collection", 
		name: as_device,
		path: "/ownedDevices/graph.device"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count,
		path: "/ownedDevices/graph.device/$count"
	);
	get!(
		doc: "Get the items of type microsoft.graph.endpoint in the microsoft.graph.directoryObject collection", 
		name: as_endpoint,
		path: "/ownedDevices/graph.endpoint"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count,
		path: "/ownedDevices/graph.endpoint/$count"
	);
}

impl OwnedDevicesIdApiClient {
	get!(
		doc: "Get ownedDevices from users", 
		name: get_owned_devices,
		path: "/ownedDevices/{{RID}}"
	);
	get!(
		doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.appRoleAssignment", 
		name: as_app_role_assignment,
		path: "/ownedDevices/{{RID}}/graph.appRoleAssignment"
	);
	get!(
		doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.device", 
		name: as_device,
		path: "/ownedDevices/{{RID}}/graph.device"
	);
	get!(
		doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.endpoint", 
		name: as_endpoint,
		path: "/ownedDevices/{{RID}}/graph.endpoint"
	);
}
