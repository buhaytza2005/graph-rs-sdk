// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(RegisteredDevicesApiClient, RegisteredDevicesIdApiClient, ResourceIdentity::RegisteredDevices);

impl RegisteredDevicesApiClient {
	get!(
		doc: "Get registeredDevices from users", 
		name: list_registered_devices,
		path: "/registeredDevices"
	);
	get!(
		doc: "Get the number of the resource", 
		name: registered_devices,
		path: "/registeredDevices/$count"
	);
	get!(
		doc: "Get the items of type microsoft.graph.appRoleAssignment in the microsoft.graph.directoryObject collection", 
		name: as_app_role_assignment,
		path: "/registeredDevices/graph.appRoleAssignment"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count,
		path: "/registeredDevices/graph.appRoleAssignment/$count"
	);
	get!(
		doc: "Get the items of type microsoft.graph.device in the microsoft.graph.directoryObject collection", 
		name: as_device,
		path: "/registeredDevices/graph.device"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count,
		path: "/registeredDevices/graph.device/$count"
	);
	get!(
		doc: "Get the items of type microsoft.graph.endpoint in the microsoft.graph.directoryObject collection", 
		name: as_endpoint,
		path: "/registeredDevices/graph.endpoint"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count,
		path: "/registeredDevices/graph.endpoint/$count"
	);
}

impl RegisteredDevicesIdApiClient {
	get!(
		doc: "Get registeredDevices from users", 
		name: get_registered_devices,
		path: "/registeredDevices/{{RID}}"
	);
	get!(
		doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.appRoleAssignment", 
		name: as_app_role_assignment,
		path: "/registeredDevices/{{RID}}/graph.appRoleAssignment"
	);
	get!(
		doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.device", 
		name: as_device,
		path: "/registeredDevices/{{RID}}/graph.device"
	);
	get!(
		doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.endpoint", 
		name: as_endpoint,
		path: "/registeredDevices/{{RID}}/graph.endpoint"
	);
}
