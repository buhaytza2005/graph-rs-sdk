// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(DirectReportsApiClient, DirectReportsIdApiClient, ResourceIdentity::DirectReports);

impl DirectReportsApiClient {
	get!(
		doc: "Get directReports from users", 
		name: list_direct_reports,
		path: "/directReports"
	);
	get!(
		doc: "Get the number of the resource", 
		name: direct_reports,
		path: "/directReports/$count"
	);
	get!(
		doc: "Get the items of type microsoft.graph.orgContact in the microsoft.graph.directoryObject collection", 
		name: as_org_contact,
		path: "/directReports/graph.orgContact"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count,
		path: "/directReports/graph.orgContact/$count"
	);
	get!(
		doc: "Get the items of type microsoft.graph.user in the microsoft.graph.directoryObject collection", 
		name: as_user,
		path: "/directReports/graph.user"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count,
		path: "/directReports/graph.user/$count"
	);
}

impl DirectReportsIdApiClient {
	get!(
		doc: "Get directReports from users", 
		name: get_direct_reports,
		path: "/directReports/{{RID}}"
	);
	get!(
		doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.orgContact", 
		name: as_org_contact,
		path: "/directReports/{{RID}}/graph.orgContact"
	);
	get!(
		doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.user", 
		name: as_user,
		path: "/directReports/{{RID}}/graph.user"
	);
}
