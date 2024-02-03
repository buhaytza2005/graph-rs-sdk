// GENERATED CODE

use crate::api_default_imports::*;
use crate::users::*;
use crate::extended_properties::*;

resource_api_client!(MailFoldersApiClient, MailFoldersIdApiClient, ResourceIdentity::MailFolders);

impl MailFoldersApiClient {
	post!(
		doc: "Create MailFolder", 
		name: create_mail_folders,
		path: "/mailFolders",
		body: true
	);
	get!(
		doc: "Get mailFolders from users", 
		name: list_mail_folders,
		path: "/mailFolders"
	);
	get!(
		doc: "Get the number of the resource", 
		name: mail_folders,
		path: "/mailFolders/$count"
	);
	get!(
		doc: "Invoke function delta", 
		name: delta,
		path: "/mailFolders/delta()"
	);
}

impl MailFoldersIdApiClient {api_client_link_id!(messages_id, UsersMessagesIdApiClient);
api_client_link_id!(child_folder, ChildFoldersIdApiClient);
api_client_link!(extended_properties, ExtendedPropertiesApiClient);
api_client_link!(child_folders, ChildFoldersApiClient);
api_client_link!(messages, UsersMessagesApiClient);

	delete!(
		doc: "Delete mailFolder", 
		name: delete_mail_folders,
		path: "/mailFolders/{{RID}}"
	);
	get!(
		doc: "Get mailFolders from users", 
		name: get_mail_folders,
		path: "/mailFolders/{{RID}}"
	);
	patch!(
		doc: "Update mailSearchFolder", 
		name: update_mail_folders,
		path: "/mailFolders/{{RID}}",
		body: true
	);
	post!(
		doc: "Invoke action copy", 
		name: copy,
		path: "/mailFolders/{{RID}}/copy",
		body: true
	);
	post!(
		doc: "Create rule", 
		name: create_message_rules,
		path: "/mailFolders/{{RID}}/messageRules",
		body: true
	);
	get!(
		doc: "List rules", 
		name: list_message_rules,
		path: "/mailFolders/{{RID}}/messageRules"
	);
	get!(
		doc: "Get the number of the resource", 
		name: message_rules,
		path: "/mailFolders/{{RID}}/messageRules/$count"
	);
	delete!(
		doc: "Delete messageRule", 
		name: delete_message_rules,
		path: "/mailFolders/{{RID}}/messageRules/{{id}}",
		params: message_rule_id
	);
	get!(
		doc: "Get rule", 
		name: get_message_rules,
		path: "/mailFolders/{{RID}}/messageRules/{{id}}",
		params: message_rule_id
	);
	patch!(
		doc: "Update rule", 
		name: update_message_rules,
		path: "/mailFolders/{{RID}}/messageRules/{{id}}",
		body: true,
		params: message_rule_id
	);
	post!(
		doc: "Invoke action move", 
		name: move_mail_folder,
		path: "/mailFolders/{{RID}}/move",
		body: true
	);
}
