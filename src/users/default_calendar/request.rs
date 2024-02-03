// GENERATED CODE

use crate::api_default_imports::*;
use crate::extended_properties::*;
use crate::users::*;

resource_api_client!(DefaultCalendarApiClient, ResourceIdentity::DefaultCalendar);

impl DefaultCalendarApiClient {api_client_link_id!(event, EventsIdApiClient);
api_client_link!(calendar_views, CalendarViewApiClient);
api_client_link_id!(calendar_view, CalendarViewIdApiClient);
api_client_link!(events, EventsApiClient);
api_client_link!(extended_properties, ExtendedPropertiesApiClient);

	delete!(
		doc: "Delete calendar", 
		name: delete_calendar,
		path: "/calendar"
	);
	get!(
		doc: "Get calendar", 
		name: get_calendar,
		path: "/calendar"
	);
	patch!(
		doc: "Update calendar", 
		name: update_calendar,
		path: "/calendar",
		body: true
	);
	get!(
		doc: "Invoke function allowedCalendarSharingRoles", 
		name: allowed_calendar_sharing_roles,
		path: "/calendar/allowedCalendarSharingRoles(User='{{id}}')",
		params: user
	);
	post!(
		doc: "Create calendarPermission", 
		name: create_calendar_permissions,
		path: "/calendar/calendarPermissions",
		body: true
	);
	get!(
		doc: "List calendarPermissions", 
		name: list_calendar_permissions,
		path: "/calendar/calendarPermissions"
	);
	get!(
		doc: "Get the number of the resource", 
		name: calendar_permissions,
		path: "/calendar/calendarPermissions/$count"
	);
	delete!(
		doc: "Delete calendarPermission", 
		name: delete_calendar_permissions,
		path: "/calendar/calendarPermissions/{{id}}",
		params: calendar_permission_id
	);
	get!(
		doc: "Get calendarPermission", 
		name: get_calendar_permissions,
		path: "/calendar/calendarPermissions/{{id}}",
		params: calendar_permission_id
	);
	patch!(
		doc: "Update calendarPermission", 
		name: update_calendar_permissions,
		path: "/calendar/calendarPermissions/{{id}}",
		body: true,
		params: calendar_permission_id
	);
	post!(
		doc: "Invoke action getSchedule", 
		name: get_schedule,
		path: "/calendar/getSchedule",
		body: true
	);
}
