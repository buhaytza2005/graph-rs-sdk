// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(MemberOfApiClient, MemberOfIdApiClient, ResourceIdentity::MemberOf);

impl MemberOfApiClient {
	get!(
		doc: "Get memberOf from users", 
		name: list_member_of,
		path: "/memberOf"
	);
	get!(
		doc: "Get the number of the resource", 
		name: member_of,
		path: "/memberOf/$count"
	);
	get!(
		doc: "Get the items of type microsoft.graph.administrativeUnit in the microsoft.graph.directoryObject collection", 
		name: as_administrative_unit,
		path: "/memberOf/graph.administrativeUnit"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count,
		path: "/memberOf/graph.administrativeUnit/$count"
	);
	get!(
		doc: "Get the items of type microsoft.graph.directoryRole in the microsoft.graph.directoryObject collection", 
		name: as_directory_role,
		path: "/memberOf/graph.directoryRole"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count,
		path: "/memberOf/graph.directoryRole/$count"
	);
	get!(
		doc: "Get the items of type microsoft.graph.group in the microsoft.graph.directoryObject collection", 
		name: as_group,
		path: "/memberOf/graph.group"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count,
		path: "/memberOf/graph.group/$count"
	);
}

impl MemberOfIdApiClient {
	get!(
		doc: "Get memberOf from users", 
		name: get_member_of,
		path: "/memberOf/{{RID}}"
	);
	get!(
		doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.administrativeUnit", 
		name: as_administrative_unit,
		path: "/memberOf/{{RID}}/graph.administrativeUnit"
	);
	get!(
		doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.directoryRole", 
		name: as_directory_role,
		path: "/memberOf/{{RID}}/graph.directoryRole"
	);
	get!(
		doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.group", 
		name: as_group,
		path: "/memberOf/{{RID}}/graph.group"
	);
}
