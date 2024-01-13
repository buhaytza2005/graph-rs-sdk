// GENERATED CODE

use crate::api_default_imports::*;
use crate::sites::*;

resource_api_client!(TermStoreSetsApiClient, TermStoreSetsIdApiClient, ResourceIdentity::TermStoreSets);

impl TermStoreSetsApiClient {
	post!(
		doc: "Create termStore set", 
		name: create_sets,
		path: "/sets",
		body: true
	);
	get!(
		doc: "Get set", 
		name: list_sets,
		path: "/sets"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count_dbcc,
		path: "/sets/$count"
	);
}

impl TermStoreSetsIdApiClient {api_client_link!(terms, TermStoreSetsTermsApiClient);
api_client_link_id!(children_id, TermStoreSetsChildrenIdApiClient);
api_client_link!(children, TermStoreSetsChildrenApiClient);
api_client_link!(parent_group, TermStoreSetsParentGroupApiClient);
api_client_link_id!(term, TermStoreSetsTermsIdApiClient);

	delete!(
		doc: "Delete set", 
		name: delete_sets,
		path: "/sets/{{RID}}"
	);
	get!(
		doc: "Get set", 
		name: get_sets,
		path: "/sets/{{RID}}"
	);
	patch!(
		doc: "Update set", 
		name: update_sets,
		path: "/sets/{{RID}}",
		body: true
	);
}
