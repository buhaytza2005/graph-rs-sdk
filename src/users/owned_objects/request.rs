// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(OwnedObjectsApiClient, OwnedObjectsIdApiClient, ResourceIdentity::OwnedObjects);

impl OwnedObjectsApiClient {
	get!(
		doc: "Get ownedObjects from users", 
		name: list_owned_objects,
		path: "/ownedObjects"
	);
	get!(
		doc: "Get the number of the resource", 
		name: owned_objects,
		path: "/ownedObjects/$count"
	);
	get!(
		doc: "Get the items of type microsoft.graph.application in the microsoft.graph.directoryObject collection", 
		name: as_application,
		path: "/ownedObjects/graph.application"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count,
		path: "/ownedObjects/graph.application/$count"
	);
	get!(
		doc: "Get the items of type microsoft.graph.group in the microsoft.graph.directoryObject collection", 
		name: as_group,
		path: "/ownedObjects/graph.group"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count,
		path: "/ownedObjects/graph.group/$count"
	);
	get!(
		doc: "Get the items of type microsoft.graph.servicePrincipal in the microsoft.graph.directoryObject collection", 
		name: as_service_principal,
		path: "/ownedObjects/graph.servicePrincipal"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count,
		path: "/ownedObjects/graph.servicePrincipal/$count"
	);
}

impl OwnedObjectsIdApiClient {
	get!(
		doc: "Get ownedObjects from users", 
		name: get_owned_objects,
		path: "/ownedObjects/{{RID}}"
	);
	get!(
		doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.application", 
		name: as_application,
		path: "/ownedObjects/{{RID}}/graph.application"
	);
	get!(
		doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.group", 
		name: as_group,
		path: "/ownedObjects/{{RID}}/graph.group"
	);
	get!(
		doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.servicePrincipal", 
		name: as_service_principal,
		path: "/ownedObjects/{{RID}}/graph.servicePrincipal"
	);
}
