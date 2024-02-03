// GENERATED CODE

use crate::api_default_imports::*;
use crate::users::*;
use crate::teams::*;

resource_api_client!(JoinedTeamsApiClient, JoinedTeamsIdApiClient, ResourceIdentity::JoinedTeams);

impl JoinedTeamsApiClient {
	post!(
		doc: "Create new navigation property to joinedTeams for users", 
		name: create_joined_teams,
		path: "/joinedTeams",
		body: true
	);
	get!(
		doc: "List joinedTeams", 
		name: list_joined_teams,
		path: "/joinedTeams"
	);
	get!(
		doc: "Get the number of the resource", 
		name: joined_teams,
		path: "/joinedTeams/$count"
	);
	get!(
		doc: "Invoke function getAllMessages", 
		name: get_all_messages,
		path: "/joinedTeams/getAllMessages()"
	);
}

impl JoinedTeamsIdApiClient {api_client_link!(channels, ChannelsApiClient);
api_client_link!(tags, TeamsTagsApiClient);
api_client_link!(members, TeamsMembersApiClient);
api_client_link!(member, TeamsMembersIdApiClient);
api_client_link_id!(channel, ChannelsIdApiClient);
api_client_link!(primary_channel, PrimaryChannelApiClient);
api_client_link_id!(tag, TeamsTagsIdApiClient);
api_client_link!(schedule, ScheduleApiClient);

	delete!(
		doc: "Delete navigation property joinedTeams for users", 
		name: delete_joined_teams,
		path: "/joinedTeams/{{RID}}"
	);
	get!(
		doc: "Get joinedTeams from users", 
		name: get_joined_teams,
		path: "/joinedTeams/{{RID}}"
	);
	patch!(
		doc: "Update the navigation property joinedTeams in users", 
		name: update_joined_teams,
		path: "/joinedTeams/{{RID}}",
		body: true
	);
	get!(
		doc: "List allChannels", 
		name: list_all_channels,
		path: "/joinedTeams/{{RID}}/allChannels"
	);
	get!(
		doc: "Get the number of the resource", 
		name: all_channels,
		path: "/joinedTeams/{{RID}}/allChannels/$count"
	);
	get!(
		doc: "Get allChannels from users", 
		name: get_all_channels,
		path: "/joinedTeams/{{RID}}/allChannels/{{id}}",
		params: channel_id
	);
	post!(
		doc: "Invoke action archive", 
		name: archive,
		path: "/joinedTeams/{{RID}}/archive",
		body: true
	);
	post!(
		doc: "Invoke action clone", 
		name: clone,
		path: "/joinedTeams/{{RID}}/clone",
		body: true
	);
	post!(
		doc: "Invoke action completeMigration", 
		name: complete_migration,
		path: "/joinedTeams/{{RID}}/completeMigration"
	);
	get!(
		doc: "Get group from users", 
		name: get_group,
		path: "/joinedTeams/{{RID}}/group"
	);
	get!(
		doc: "Get serviceProvisioningErrors property value", 
		name: list_service_provisioning_errors,
		path: "/joinedTeams/{{RID}}/group/serviceProvisioningErrors"
	);
	get!(
		doc: "Get the number of the resource", 
		name: service_provisioning_errors,
		path: "/joinedTeams/{{RID}}/group/serviceProvisioningErrors/$count"
	);
	get!(
		doc: "List incomingChannels", 
		name: list_incoming_channels,
		path: "/joinedTeams/{{RID}}/incomingChannels"
	);
	get!(
		doc: "Get the number of the resource", 
		name: incoming_channels,
		path: "/joinedTeams/{{RID}}/incomingChannels/$count"
	);
	get!(
		doc: "Get incomingChannels from users", 
		name: get_incoming_channels,
		path: "/joinedTeams/{{RID}}/incomingChannels/{{id}}",
		params: channel_id
	);
	delete!(
		doc: "Remove channel", 
		name: delete_incoming_channels,
		path: "/joinedTeams/{{RID}}/incomingChannels/{{id}}",
		params: channel_id
	);
	post!(
		doc: "Add app to team", 
		name: create_installed_apps,
		path: "/joinedTeams/{{RID}}/installedApps",
		body: true
	);
	get!(
		doc: "List apps in team", 
		name: list_installed_apps,
		path: "/joinedTeams/{{RID}}/installedApps"
	);
	get!(
		doc: "Get the number of the resource", 
		name: installed_apps,
		path: "/joinedTeams/{{RID}}/installedApps/$count"
	);
	get!(
		doc: "Get installed app in team", 
		name: get_installed_apps,
		path: "/joinedTeams/{{RID}}/installedApps/{{id}}",
		params: teams_app_installation_id
	);
	delete!(
		doc: "Remove app from team", 
		name: delete_installed_apps,
		path: "/joinedTeams/{{RID}}/installedApps/{{id}}",
		params: teams_app_installation_id
	);
	patch!(
		doc: "Update the navigation property installedApps in users", 
		name: update_installed_apps,
		path: "/joinedTeams/{{RID}}/installedApps/{{id}}",
		body: true,
		params: teams_app_installation_id
	);
	get!(
		doc: "Get teamsApp from users", 
		name: get_teams_app,
		path: "/joinedTeams/{{RID}}/installedApps/{{id}}/teamsApp",
		params: teams_app_installation_id
	);
	get!(
		doc: "Get teamsAppDefinition from users", 
		name: get_teams_app_definition,
		path: "/joinedTeams/{{RID}}/installedApps/{{id}}/teamsAppDefinition",
		params: teams_app_installation_id
	);
	post!(
		doc: "Invoke action upgrade", 
		name: upgrade,
		path: "/joinedTeams/{{RID}}/installedApps/{{id}}/upgrade",
		body: true,
		params: teams_app_installation_id
	);
	post!(
		doc: "Add member to team", 
		name: create_members,
		path: "/joinedTeams/{{RID}}/members",
		body: true
	);
	get!(
		doc: "List members of team", 
		name: list_members,
		path: "/joinedTeams/{{RID}}/members"
	);
	get!(
		doc: "Get the number of the resource", 
		name: members,
		path: "/joinedTeams/{{RID}}/members/$count"
	);
	post!(
		doc: "Invoke action add", 
		name: add,
		path: "/joinedTeams/{{RID}}/members/add",
		body: true
	);
	get!(
		doc: "Get member of team", 
		name: get_members,
		path: "/joinedTeams/{{RID}}/members/{{id}}",
		params: conversation_member_id
	);
	delete!(
		doc: "Remove member from team", 
		name: delete_members,
		path: "/joinedTeams/{{RID}}/members/{{id}}",
		params: conversation_member_id
	);
	patch!(
		doc: "Update member in team", 
		name: update_members,
		path: "/joinedTeams/{{RID}}/members/{{id}}",
		body: true,
		params: conversation_member_id
	);
	post!(
		doc: "Create new navigation property to operations for users", 
		name: create_operations,
		path: "/joinedTeams/{{RID}}/operations",
		body: true
	);
	get!(
		doc: "Get operations from users", 
		name: list_operations,
		path: "/joinedTeams/{{RID}}/operations"
	);
	get!(
		doc: "Get the number of the resource", 
		name: operations,
		path: "/joinedTeams/{{RID}}/operations/$count"
	);
	delete!(
		doc: "Delete navigation property operations for users", 
		name: delete_operations,
		path: "/joinedTeams/{{RID}}/operations/{{id}}",
		params: teams_async_operation_id
	);
	get!(
		doc: "Get operations from users", 
		name: get_operations,
		path: "/joinedTeams/{{RID}}/operations/{{id}}",
		params: teams_async_operation_id
	);
	patch!(
		doc: "Update the navigation property operations in users", 
		name: update_operations,
		path: "/joinedTeams/{{RID}}/operations/{{id}}",
		body: true,
		params: teams_async_operation_id
	);
	post!(
		doc: "Create new navigation property to permissionGrants for users", 
		name: create_permission_grants,
		path: "/joinedTeams/{{RID}}/permissionGrants",
		body: true
	);
	get!(
		doc: "List permissionGrants of a team", 
		name: list_permission_grants,
		path: "/joinedTeams/{{RID}}/permissionGrants"
	);
	get!(
		doc: "Get the number of the resource", 
		name: permission_grants,
		path: "/joinedTeams/{{RID}}/permissionGrants/$count"
	);
	delete!(
		doc: "Delete navigation property permissionGrants for users", 
		name: delete_permission_grants,
		path: "/joinedTeams/{{RID}}/permissionGrants/{{id}}",
		params: resource_specific_permission_grant_id
	);
	get!(
		doc: "Get permissionGrants from users", 
		name: get_permission_grants,
		path: "/joinedTeams/{{RID}}/permissionGrants/{{id}}",
		params: resource_specific_permission_grant_id
	);
	patch!(
		doc: "Update the navigation property permissionGrants in users", 
		name: update_permission_grants,
		path: "/joinedTeams/{{RID}}/permissionGrants/{{id}}",
		body: true,
		params: resource_specific_permission_grant_id
	);
	get!(
		doc: "Get profilePhoto", 
		name: get_photo,
		path: "/joinedTeams/{{RID}}/photo"
	);
	patch!(
		doc: "Update the navigation property photo in users", 
		name: update_photo,
		path: "/joinedTeams/{{RID}}/photo",
		body: true
	);
	get!(
		doc: "Get media content for the navigation property photo from users", 
		name: get_photo_content,
		path: "/joinedTeams/{{RID}}/photo/$value"
	);
	put!(
		doc: "Update media content for the navigation property photo in users", 
		name: update_photo_content,
		path: "/joinedTeams/{{RID}}/photo/$value",
		body: true
	);
	post!(
		doc: "Invoke action sendActivityNotification", 
		name: send_activity_notification,
		path: "/joinedTeams/{{RID}}/sendActivityNotification",
		body: true
	);
	get!(
		doc: "Get template from users", 
		name: get_template,
		path: "/joinedTeams/{{RID}}/template"
	);
	post!(
		doc: "Invoke action unarchive", 
		name: unarchive,
		path: "/joinedTeams/{{RID}}/unarchive"
	);
}
