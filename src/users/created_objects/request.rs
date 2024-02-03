// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(CreatedObjectsApiClient, CreatedObjectsIdApiClient, ResourceIdentity::CreatedObjects);

impl CreatedObjectsApiClient {
	get!(
		doc: "List createdObjects", 
		name: list_created_objects,
		path: "/createdObjects"
	);
	get!(
		doc: "Get the number of the resource", 
		name: created_objects,
		path: "/createdObjects/$count"
	);
	get!(
		doc: "Get the items of type microsoft.graph.servicePrincipal in the microsoft.graph.directoryObject collection", 
		name: as_service_principal,
		path: "/createdObjects/graph.servicePrincipal"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count,
		path: "/createdObjects/graph.servicePrincipal/$count"
	);
}

impl CreatedObjectsIdApiClient {
	get!(
		doc: "Get createdObjects from users", 
		name: get_created_objects,
		path: "/createdObjects/{{RID}}"
	);
	get!(
		doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.servicePrincipal", 
		name: as_service_principal,
		path: "/createdObjects/{{RID}}/graph.servicePrincipal"
	);
}
