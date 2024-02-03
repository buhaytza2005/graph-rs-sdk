// GENERATED CODE

use crate::api_default_imports::*;
use crate::users::*;
use crate::extended_properties::*;

resource_api_client!(ContactFoldersApiClient, ContactFoldersIdApiClient, ResourceIdentity::ContactFolders);

impl ContactFoldersApiClient {
	post!(
		doc: "Create ContactFolder", 
		name: create_contact_folders,
		path: "/contactFolders",
		body: true
	);
	get!(
		doc: "List contactFolders", 
		name: list_contact_folders,
		path: "/contactFolders"
	);
	get!(
		doc: "Get the number of the resource", 
		name: contact_folders,
		path: "/contactFolders/$count"
	);
	get!(
		doc: "Invoke function delta", 
		name: delta,
		path: "/contactFolders/delta()"
	);
}

impl ContactFoldersIdApiClient {api_client_link_id!(child_folder, ChildFoldersIdApiClient);
api_client_link!(extended_properties, ExtendedPropertiesApiClient);
api_client_link!(child_folders, ChildFoldersApiClient);
api_client_link_id!(contact, ContactsIdApiClient);
api_client_link!(contacts, ContactsApiClient);

	delete!(
		doc: "Delete contactFolder", 
		name: delete_contact_folders,
		path: "/contactFolders/{{RID}}"
	);
	get!(
		doc: "Get contactFolder", 
		name: get_contact_folders,
		path: "/contactFolders/{{RID}}"
	);
	patch!(
		doc: "Update contactfolder", 
		name: update_contact_folders,
		path: "/contactFolders/{{RID}}",
		body: true
	);
}
