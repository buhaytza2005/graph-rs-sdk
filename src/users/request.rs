// GENERATED CODE

use crate::api_default_imports::*;
use crate::users::*;
use crate::oauth2_permission_grants::*;
use crate::agreement_acceptances::*;
use crate::planner::*;
use crate::teams::*;
use crate::solutions::*;
use crate::chats::*;

resource_api_client!(UsersApiClient, UsersIdApiClient, ResourceIdentity::Users);

impl UsersApiClient {
	post!(
		doc: "Create user", 
		name: create_user,
		path: "/users",
		body: true
	);
	get!(
		doc: "List users", 
		name: list_user,
		path: "/users"
	);
	get!(
		doc: "Get the number of the resource", 
		name: users_get_count_ee_47,
		path: "/users/$count"
	);
	get!(
		doc: "Invoke function delta", 
		name: delta,
		path: "/users/delta()"
	);
	post!(
		doc: "Invoke action getAvailableExtensionProperties", 
		name: get_available_extension_properties,
		path: "/users/getAvailableExtensionProperties",
		body: true
	);
	post!(
		doc: "Invoke action getByIds", 
		name: get_by_ids,
		path: "/users/getByIds",
		body: true
	);
	post!(
		doc: "Invoke action validateProperties", 
		name: validate_properties,
		path: "/users/validateProperties",
		body: true
	);
}

impl UsersIdApiClient {api_client_link!(created_objects, CreatedObjectsApiClient);
api_client_link_id!(oauth2_permission_grant, Oauth2PermissionGrantsIdApiClient);
api_client_link!(outlook, OutlookApiClient);
api_client_link!(owned_objects, OwnedObjectsApiClient);
api_client_link!(solution, SolutionsIdApiClient);
api_client_link_id!(calendar_view, CalendarViewIdApiClient);
api_client_link!(insights, InsightsApiClient);
api_client_link_id!(extension, ExtensionsIdApiClient);
api_client_link_id!(license_detail, LicenseDetailsIdApiClient);
api_client_link_id!(mail_folder, MailFoldersIdApiClient);
api_client_link!(joined_teams, JoinedTeamsApiClient);
api_client_link_id!(device_management_troubleshooting_event, DeviceManagementTroubleshootingEventsIdApiClient);
api_client_link!(followed_sites, FollowedSitesApiClient);
api_client_link!(activities, ActivitiesApiClient);
api_client_link!(presence, PresenceApiClient);
api_client_link!(onenote, OnenoteApiClient);
api_client_link!(device_management_troubleshooting_events, DeviceManagementTroubleshootingEventsApiClient);
api_client_link!(extensions, ExtensionsApiClient);
api_client_link!(chats, ChatsApiClient);
api_client_link!(messages, UsersMessagesApiClient);
api_client_link!(default_calendar, DefaultCalendarApiClient);
api_client_link_id!(activity, ActivitiesIdApiClient);
api_client_link!(todo, TodoApiClient);
api_client_link!(mailbox_settings, MailboxSettingsApiClient);
api_client_link!(authentication, AuthenticationApiClient);
api_client_link_id!(transitive_member_of_id, TransitiveMemberOfIdApiClient);
api_client_link!(direct_reports, DirectReportsApiClient);
api_client_link_id!(owned_object, OwnedObjectsIdApiClient);
api_client_link_id!(channel, ChannelsIdApiClient);
api_client_link!(planner, PlannerApiClient);
api_client_link!(owned_devices, OwnedDevicesApiClient);
api_client_link_id!(member_of_id, MemberOfIdApiClient);
api_client_link_id!(direct_report, DirectReportsIdApiClient);
api_client_link_id!(managed_device, ManagedDevicesIdApiClient);
api_client_link!(oauth2_permission_grants, Oauth2PermissionGrantsApiClient);
api_client_link!(contacts, ContactsApiClient);
api_client_link!(transitive_member_of, TransitiveMemberOfApiClient);
api_client_link!(events, EventsApiClient);
api_client_link!(calendars, CalendarsApiClient);
api_client_link_id!(photo, PhotosIdApiClient);
api_client_link_id!(contact, ContactsIdApiClient);
api_client_link!(registered_devices, RegisteredDevicesApiClient);
api_client_link_id!(agreement_acceptance, AgreementAcceptancesIdApiClient);
api_client_link!(member_of, MemberOfApiClient);
api_client_link!(managed_app_registrations, ManagedAppRegistrationsApiClient);
api_client_link_id!(online_meeting, OnlineMeetingsIdApiClient);
api_client_link!(teamwork, TeamworkApiClient);
api_client_link!(drive, DefaultDriveApiClient);
api_client_link!(channels, ChannelsApiClient);
api_client_link!(scoped_role_member_of, ScopedRoleMemberOfApiClient);
api_client_link_id!(contact_folder, ContactFoldersIdApiClient);
api_client_link!(photos, PhotosApiClient);
api_client_link!(mail_folders, MailFoldersApiClient);
api_client_link!(schedule, ScheduleApiClient);
api_client_link_id!(chat, ChatsIdApiClient);
api_client_link_id!(owned_device, OwnedDevicesIdApiClient);
api_client_link!(agreement_acceptances, AgreementAcceptancesApiClient);
api_client_link!(inference_classification, InferenceClassificationApiClient);
api_client_link!(calendar_groups, CalendarGroupsApiClient);
api_client_link_id!(joined_team, JoinedTeamsIdApiClient);
api_client_link_id!(app_role_assignment, AppRoleAssignmentsIdApiClient);
api_client_link!(calendar_views, CalendarViewApiClient);
api_client_link!(contact_folders, ContactFoldersApiClient);
api_client_link_id!(scoped_role_member_of_id, ScopedRoleMemberOfIdApiClient);
api_client_link!(settings, SettingsApiClient);
api_client_link_id!(calendar_group, CalendarGroupsIdApiClient);
api_client_link_id!(message, UsersMessagesIdApiClient);
api_client_link!(app_role_assignments, AppRoleAssignmentsApiClient);
api_client_link_id!(managed_app_registration, ManagedAppRegistrationsIdApiClient);
api_client_link!(solutions, SolutionsApiClient);
api_client_link!(online_meetings, OnlineMeetingsApiClient);
api_client_link_id!(event, EventsIdApiClient);
api_client_link_id!(registered_device, RegisteredDevicesIdApiClient);
api_client_link!(managed_devices, ManagedDevicesApiClient);
api_client_link_id!(created_object, CreatedObjectsIdApiClient);
api_client_link_id!(calendar, CalendarsIdApiClient);
api_client_link!(license_details, LicenseDetailsApiClient);

	delete!(
		doc: "Delete user", 
		name: delete_user,
		path: "/users/{{RID}}"
	);
	get!(
		doc: "Get user", 
		name: get_user,
		path: "/users/{{RID}}"
	);
	patch!(
		doc: "Update user", 
		name: update_user,
		path: "/users/{{RID}}",
		body: true
	);
	get!(
		doc: "List agreementAcceptances", 
		name: list_agreement_acceptances,
		path: "/users/{{RID}}/agreementAcceptances"
	);
	get!(
		doc: "Get the number of the resource", 
		name: agreement_acceptances,
		path: "/users/{{RID}}/agreementAcceptances/$count"
	);
	get!(
		doc: "Get agreementAcceptances from users", 
		name: get_agreement_acceptances,
		path: "/users/{{RID}}/agreementAcceptances/{{id}}",
		params: agreement_acceptance_id
	);
	post!(
		doc: "Invoke action assignLicense", 
		name: assign_license,
		path: "/users/{{RID}}/assignLicense",
		body: true
	);
	post!(
		doc: "Invoke action changePassword", 
		name: change_password,
		path: "/users/{{RID}}/changePassword",
		body: true
	);
	post!(
		doc: "Invoke action checkMemberGroups", 
		name: check_member_groups,
		path: "/users/{{RID}}/checkMemberGroups",
		body: true
	);
	post!(
		doc: "Invoke action checkMemberObjects", 
		name: check_member_objects,
		path: "/users/{{RID}}/checkMemberObjects",
		body: true
	);
	get!(
		doc: "Get Drive", 
		name: get_drive,
		path: "/users/{{RID}}/drive"
	);
	get!(
		doc: "List available drives", 
		name: list_drives,
		path: "/users/{{RID}}/drives"
	);
	get!(
		doc: "Get the number of the resource", 
		name: drives,
		path: "/users/{{RID}}/drives/$count"
	);
	get!(
		doc: "Get drives from users", 
		name: get_drives,
		path: "/users/{{RID}}/drives/{{id}}",
		params: drive_id
	);
	delete!(
		doc: "Delete navigation property employeeExperience for users", 
		name: delete_employee_experience,
		path: "/users/{{RID}}/employeeExperience"
	);
	get!(
		doc: "Get employeeExperience from users", 
		name: get_employee_experience,
		path: "/users/{{RID}}/employeeExperience"
	);
	patch!(
		doc: "Update the navigation property employeeExperience in users", 
		name: update_employee_experience,
		path: "/users/{{RID}}/employeeExperience",
		body: true
	);
	get!(
		doc: "List learningCourseActivities", 
		name: list_learning_course_activities,
		path: "/users/{{RID}}/employeeExperience/learningCourseActivities"
	);
	get!(
		doc: "Get learningCourseActivity", 
		name: get_learning_course_activities,
		path: "/users/{{RID}}/employeeExperience/learningCourseActivities(externalcourseActivityId='{{id}}')",
		params: externalcourse_activity_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: learning_course_activities,
		path: "/users/{{RID}}/employeeExperience/learningCourseActivities/$count"
	);
	get!(
		doc: "Get learningCourseActivity", 
		name: get_learning_course_activities,
		path: "/users/{{RID}}/employeeExperience/learningCourseActivities/{{id}}",
		params: learning_course_activity_id
	);
	get!(
		doc: "Invoke function exportDeviceAndAppManagementData", 
		name: user,
		path: "/users/{{RID}}/exportDeviceAndAppManagementData()"
	);
	get!(
		doc: "Invoke function exportDeviceAndAppManagementData", 
		name: export_device_app_management,
		path: "/users/{{RID}}/exportDeviceAndAppManagementData(skip={{id}},top={{id2}})",
		params: skip, top
	);
	post!(
		doc: "Invoke action exportPersonalData", 
		name: export_personal_data,
		path: "/users/{{RID}}/exportPersonalData",
		body: true
	);
	post!(
		doc: "Invoke action findMeetingTimes", 
		name: find_meeting_times,
		path: "/users/{{RID}}/findMeetingTimes",
		body: true
	);
	post!(
		doc: "Invoke action getMailTips", 
		name: get_mail_tips,
		path: "/users/{{RID}}/getMailTips",
		body: true
	);
	get!(
		doc: "Invoke function getManagedAppDiagnosticStatuses", 
		name: get_managed_app_diagnostic_statuses,
		path: "/users/{{RID}}/getManagedAppDiagnosticStatuses()"
	);
	get!(
		doc: "Invoke function getManagedAppPolicies", 
		name: get_managed_app_policies,
		path: "/users/{{RID}}/getManagedAppPolicies()"
	);
	get!(
		doc: "Invoke function getManagedDevicesWithAppFailures", 
		name: get_managed_devices_with_app_failures,
		path: "/users/{{RID}}/getManagedDevicesWithAppFailures()"
	);
	post!(
		doc: "Invoke action getMemberGroups", 
		name: get_member_groups,
		path: "/users/{{RID}}/getMemberGroups",
		body: true
	);
	post!(
		doc: "Invoke action getMemberObjects", 
		name: get_member_objects,
		path: "/users/{{RID}}/getMemberObjects",
		body: true
	);
	get!(
		doc: "Get mailboxSettings property value", 
		name: get_mailbox_settings,
		path: "/users/{{RID}}/mailboxSettings"
	);
	patch!(
		doc: "Update property mailboxSettings value.", 
		name: update_mailbox_settings,
		path: "/users/{{RID}}/mailboxSettings",
		body: true
	);
	get!(
		doc: "List manager", 
		name: get_manager,
		path: "/users/{{RID}}/manager"
	);
	delete!(
		doc: "Remove manager", 
		name: delete_manager,
		path: "/users/{{RID}}/manager"
	);
	put!(
		doc: "Assign manager", 
		name: update_ref_manager,
		path: "/users/{{RID}}/manager/$ref",
		body: true
	);
	get!(
		doc: "List manager", 
		name: get_ref_manager,
		path: "/users/{{RID}}/manager/$ref"
	);
	delete!(
		doc: "Remove manager", 
		name: delete_ref_manager,
		path: "/users/{{RID}}/manager/$ref"
	);
	get!(
		doc: "List people", 
		name: list_people,
		path: "/users/{{RID}}/people"
	);
	get!(
		doc: "Get the number of the resource", 
		name: people,
		path: "/users/{{RID}}/people/$count"
	);
	get!(
		doc: "Get people from users", 
		name: get_people,
		path: "/users/{{RID}}/people/{{id}}",
		params: person_id
	);
	post!(
		doc: "Create new navigation property to permissionGrants for users", 
		name: create_permission_grants,
		path: "/users/{{RID}}/permissionGrants",
		body: true
	);
	get!(
		doc: "List permissionGrants of a user", 
		name: list_permission_grants,
		path: "/users/{{RID}}/permissionGrants"
	);
	get!(
		doc: "Get the number of the resource", 
		name: permission_grants,
		path: "/users/{{RID}}/permissionGrants/$count"
	);
	delete!(
		doc: "Delete navigation property permissionGrants for users", 
		name: delete_permission_grants,
		path: "/users/{{RID}}/permissionGrants/{{id}}",
		params: resource_specific_permission_grant_id
	);
	get!(
		doc: "Get permissionGrants from users", 
		name: get_permission_grants,
		path: "/users/{{RID}}/permissionGrants/{{id}}",
		params: resource_specific_permission_grant_id
	);
	patch!(
		doc: "Update the navigation property permissionGrants in users", 
		name: update_permission_grants,
		path: "/users/{{RID}}/permissionGrants/{{id}}",
		body: true,
		params: resource_specific_permission_grant_id
	);
	get!(
		doc: "Get profilePhoto", 
		name: get_photo,
		path: "/users/{{RID}}/photo"
	);
	patch!(
		doc: "Update the navigation property photo in users", 
		name: update_photo,
		path: "/users/{{RID}}/photo",
		body: true
	);
	get!(
		doc: "Get media content for the navigation property photo from users", 
		name: get_photo_content,
		path: "/users/{{RID}}/photo/$value"
	);
	put!(
		doc: "Update media content for the navigation property photo in users", 
		name: update_photo_content,
		path: "/users/{{RID}}/photo/$value",
		body: true
	);
	delete!(
		doc: "Delete navigation property planner for users", 
		name: delete_planner,
		path: "/users/{{RID}}/planner"
	);
	get!(
		doc: "Get planner from users", 
		name: get_planner,
		path: "/users/{{RID}}/planner"
	);
	patch!(
		doc: "Update the navigation property planner in users", 
		name: update_planner,
		path: "/users/{{RID}}/planner",
		body: true
	);
	get!(
		doc: "Invoke function reminderView", 
		name: reminder_view,
		path: "/users/{{RID}}/reminderView(StartDateTime='{{id}}',EndDateTime='{{id2}}')",
		params: start_date_time, end_date_time
	);
	post!(
		doc: "Invoke action removeAllDevicesFromManagement", 
		name: remove_all_devices_from_management,
		path: "/users/{{RID}}/removeAllDevicesFromManagement"
	);
	post!(
		doc: "Invoke action reprocessLicenseAssignment", 
		name: reprocess_license_assignment,
		path: "/users/{{RID}}/reprocessLicenseAssignment"
	);
	post!(
		doc: "Invoke action restore", 
		name: restore,
		path: "/users/{{RID}}/restore"
	);
	post!(
		doc: "Invoke action retryServiceProvisioning", 
		name: retry_service_provisioning,
		path: "/users/{{RID}}/retryServiceProvisioning"
	);
	post!(
		doc: "Invoke action revokeSignInSessions", 
		name: revoke_sign_in_sessions,
		path: "/users/{{RID}}/revokeSignInSessions"
	);
	post!(
		doc: "Invoke action sendMail", 
		name: send_mail,
		path: "/users/{{RID}}/sendMail",
		body: true
	);
	get!(
		doc: "Get serviceProvisioningErrors property value", 
		name: list_service_provisioning_errors,
		path: "/users/{{RID}}/serviceProvisioningErrors"
	);
	get!(
		doc: "Get the number of the resource", 
		name: get_count_bccc,
		path: "/users/{{RID}}/serviceProvisioningErrors/$count"
	);
	delete!(
		doc: "Delete navigation property settings for users", 
		name: delete_settings,
		path: "/users/{{RID}}/settings"
	);
	get!(
		doc: "Get settings", 
		name: get_settings,
		path: "/users/{{RID}}/settings"
	);
	patch!(
		doc: "Update userSettings", 
		name: update_settings,
		path: "/users/{{RID}}/settings",
		body: true
	);
	delete!(
		doc: "Delete navigation property shiftPreferences for users", 
		name: delete_shift_preferences,
		path: "/users/{{RID}}/settings/shiftPreferences"
	);
	get!(
		doc: "Get shiftPreferences", 
		name: get_shift_preferences,
		path: "/users/{{RID}}/settings/shiftPreferences"
	);
	patch!(
		doc: "Update shiftPreferences", 
		name: update_shift_preferences,
		path: "/users/{{RID}}/settings/shiftPreferences",
		body: true
	);
	post!(
		doc: "Invoke action translateExchangeIds", 
		name: translate_exchange_ids,
		path: "/users/{{RID}}/translateExchangeIds",
		body: true
	);
	post!(
		doc: "Invoke action wipeManagedAppRegistrationsByDeviceTag", 
		name: wipe_managed_app_registrations_by_device_tag,
		path: "/users/{{RID}}/wipeManagedAppRegistrationsByDeviceTag",
		body: true
	);
}
