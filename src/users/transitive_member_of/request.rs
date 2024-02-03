// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(TransitiveMemberOfApiClient, TransitiveMemberOfIdApiClient, ResourceIdentity::TransitiveMemberOf);

impl TransitiveMemberOfApiClient {
	get!(
		doc: "Get transitiveMemberOf from users", 
		name: list_transitive_member_of,
		path: "/transitiveMemberOf"
	);
	get!(
		doc: "Get the number of the resource", 
		name: transitive_member_of,
		path: "/transitiveMemberOf/$count"
	);
	get!(
		doc: "Get the items of type microsoft.graph.administrativeUnit in the microsoft.graph.directoryObject collection", 
		name: as_administrative_unit,
		path: "/transitiveMemberOf/graph.administrativeUnit"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count,
		path: "/transitiveMemberOf/graph.administrativeUnit/$count"
	);
	get!(
		doc: "Get the items of type microsoft.graph.directoryRole in the microsoft.graph.directoryObject collection", 
		name: as_directory_role,
		path: "/transitiveMemberOf/graph.directoryRole"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count,
		path: "/transitiveMemberOf/graph.directoryRole/$count"
	);
	get!(
		doc: "Get the items of type microsoft.graph.group in the microsoft.graph.directoryObject collection", 
		name: as_group,
		path: "/transitiveMemberOf/graph.group"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count,
		path: "/transitiveMemberOf/graph.group/$count"
	);
}

impl TransitiveMemberOfIdApiClient {
	get!(
		doc: "Get transitiveMemberOf from users", 
		name: get_transitive_member_of,
		path: "/transitiveMemberOf/{{RID}}"
	);
	get!(
		doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.administrativeUnit", 
		name: as_administrative_unit,
		path: "/transitiveMemberOf/{{RID}}/graph.administrativeUnit"
	);
	get!(
		doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.directoryRole", 
		name: as_directory_role,
		path: "/transitiveMemberOf/{{RID}}/graph.directoryRole"
	);
	get!(
		doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.group", 
		name: as_group,
		path: "/transitiveMemberOf/{{RID}}/graph.group"
	);
}
