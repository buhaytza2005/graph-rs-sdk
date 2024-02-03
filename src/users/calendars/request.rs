// GENERATED CODE

use crate::api_default_imports::*;
use crate::users::*;
use crate::extended_properties::*;

resource_api_client!(CalendarsApiClient, CalendarsIdApiClient, ResourceIdentity::Calendars);

impl CalendarsApiClient {
	post!(
		doc: "Create calendar", 
		name: create_calendars,
		path: "/calendars",
		body: true
	);
	get!(
		doc: "List calendars", 
		name: list_calendars,
		path: "/calendars"
	);
	get!(
		doc: "Get the number of the resource", 
		name: calendars,
		path: "/calendars/$count"
	);
}

impl CalendarsIdApiClient {api_client_link_id!(calendar_view, CalendarViewIdApiClient);
api_client_link!(extended_properties, ExtendedPropertiesApiClient);
api_client_link!(calendar_views, CalendarViewApiClient);
api_client_link_id!(event, EventsIdApiClient);
api_client_link!(events, EventsApiClient);

	delete!(
		doc: "Delete navigation property calendars for users", 
		name: delete_calendars,
		path: "/calendars/{{RID}}"
	);
	get!(
		doc: "Get calendars from users", 
		name: get_calendars,
		path: "/calendars/{{RID}}"
	);
	patch!(
		doc: "Update the navigation property calendars in users", 
		name: update_calendars,
		path: "/calendars/{{RID}}",
		body: true
	);
	get!(
		doc: "Invoke function allowedCalendarSharingRoles", 
		name: allowed_calendar_sharing_roles,
		path: "/calendars/{{RID}}/allowedCalendarSharingRoles(User='{{id}}')",
		params: user
	);
	post!(
		doc: "Create calendarPermission", 
		name: create_calendar_permissions,
		path: "/calendars/{{RID}}/calendarPermissions",
		body: true
	);
	get!(
		doc: "List calendarPermissions", 
		name: list_calendar_permissions,
		path: "/calendars/{{RID}}/calendarPermissions"
	);
	get!(
		doc: "Get the number of the resource", 
		name: calendar_permissions,
		path: "/calendars/{{RID}}/calendarPermissions/$count"
	);
	delete!(
		doc: "Delete calendarPermission", 
		name: delete_calendar_permissions,
		path: "/calendars/{{RID}}/calendarPermissions/{{id}}",
		params: calendar_permission_id
	);
	get!(
		doc: "Get calendarPermission", 
		name: get_calendar_permissions,
		path: "/calendars/{{RID}}/calendarPermissions/{{id}}",
		params: calendar_permission_id
	);
	patch!(
		doc: "Update calendarPermission", 
		name: update_calendar_permissions,
		path: "/calendars/{{RID}}/calendarPermissions/{{id}}",
		body: true,
		params: calendar_permission_id
	);
	post!(
		doc: "Create event", 
		name: create_events,
		path: "/calendars/{{RID}}/events",
		body: true
	);
	get!(
		doc: "List events", 
		name: list_events,
		path: "/calendars/{{RID}}/events"
	);
	get!(
		doc: "Get the number of the resource", 
		name: events,
		path: "/calendars/{{RID}}/events/$count"
	);
	get!(
		doc: "Invoke function delta", 
		name: delta,
		path: "/calendars/{{RID}}/events/delta()"
	);
	delete!(
		doc: "Delete navigation property events for users", 
		name: delete_events,
		path: "/calendars/{{RID}}/events/{{id}}",
		params: event_id
	);
	get!(
		doc: "Get events from users", 
		name: get_events,
		path: "/calendars/{{RID}}/events/{{id}}",
		params: event_id
	);
	patch!(
		doc: "Update event", 
		name: update_events,
		path: "/calendars/{{RID}}/events/{{id}}",
		body: true,
		params: event_id
	);
	post!(
		doc: "Invoke action accept", 
		name: accept,
		path: "/calendars/{{RID}}/events/{{id}}/accept",
		body: true,
		params: event_id
	);
	post!(
		doc: "Add attachment", 
		name: create_attachments,
		path: "/calendars/{{RID}}/events/{{id}}/attachments",
		body: true,
		params: event_id
	);
	get!(
		doc: "List attachments", 
		name: list_attachments,
		path: "/calendars/{{RID}}/events/{{id}}/attachments",
		params: event_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: attachments,
		path: "/calendars/{{RID}}/events/{{id}}/attachments/$count",
		params: event_id
	);
	post!(
		doc: "Invoke action createUploadSession", 
		name: create_upload_session,
		path: "/calendars/{{RID}}/events/{{id}}/attachments/createUploadSession",
		body: true,
		params: event_id
	);
	delete!(
		doc: "Delete attachment", 
		name: delete_attachments,
		path: "/calendars/{{RID}}/events/{{id}}/attachments/{{id2}}",
		params: event_id, attachment_id
	);
	get!(
		doc: "Get attachments from users", 
		name: get_attachments,
		path: "/calendars/{{RID}}/events/{{id}}/attachments/{{id2}}",
		params: event_id, attachment_id
	);
	get!(
		doc: "Get calendar from users", 
		name: get_calendar,
		path: "/calendars/{{RID}}/events/{{id}}/calendar",
		params: event_id
	);
	post!(
		doc: "Invoke action cancel", 
		name: cancel,
		path: "/calendars/{{RID}}/events/{{id}}/cancel",
		body: true,
		params: event_id
	);
	post!(
		doc: "Invoke action decline", 
		name: decline,
		path: "/calendars/{{RID}}/events/{{id}}/decline",
		body: true,
		params: event_id
	);
	post!(
		doc: "Invoke action dismissReminder", 
		name: dismiss_reminder,
		path: "/calendars/{{RID}}/events/{{id}}/dismissReminder",
		params: event_id
	);
	post!(
		doc: "Create open extension", 
		name: create_extensions,
		path: "/calendars/{{RID}}/events/{{id}}/extensions",
		body: true,
		params: event_id
	);
	get!(
		doc: "Get open extension", 
		name: list_extensions,
		path: "/calendars/{{RID}}/events/{{id}}/extensions",
		params: event_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: extensions,
		path: "/calendars/{{RID}}/events/{{id}}/extensions/$count",
		params: event_id
	);
	delete!(
		doc: "Delete navigation property extensions for users", 
		name: delete_extensions,
		path: "/calendars/{{RID}}/events/{{id}}/extensions/{{id2}}",
		params: event_id, extension_id
	);
	get!(
		doc: "Get open extension", 
		name: get_extensions,
		path: "/calendars/{{RID}}/events/{{id}}/extensions/{{id2}}",
		params: event_id, extension_id
	);
	patch!(
		doc: "Update the navigation property extensions in users", 
		name: update_extensions,
		path: "/calendars/{{RID}}/events/{{id}}/extensions/{{id2}}",
		body: true,
		params: event_id, extension_id
	);
	post!(
		doc: "Invoke action forward", 
		name: forward,
		path: "/calendars/{{RID}}/events/{{id}}/forward",
		body: true,
		params: event_id
	);
	post!(
		doc: "Invoke action snoozeReminder", 
		name: snooze_reminder,
		path: "/calendars/{{RID}}/events/{{id}}/snoozeReminder",
		body: true,
		params: event_id
	);
	post!(
		doc: "Invoke action tentativelyAccept", 
		name: tentatively_accept,
		path: "/calendars/{{RID}}/events/{{id}}/tentativelyAccept",
		body: true,
		params: event_id
	);
	post!(
		doc: "Invoke action getSchedule", 
		name: get_schedule,
		path: "/calendars/{{RID}}/getSchedule",
		body: true
	);
}
