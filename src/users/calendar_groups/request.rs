// GENERATED CODE

use crate::api_default_imports::*;
use crate::users::*;

resource_api_client!(CalendarGroupsApiClient, CalendarGroupsIdApiClient, ResourceIdentity::CalendarGroups);

impl CalendarGroupsApiClient {
	post!(
		doc: "Create CalendarGroup", 
		name: create_calendar_groups,
		path: "/calendarGroups",
		body: true
	);
	get!(
		doc: "List calendarGroups", 
		name: list_calendar_groups,
		path: "/calendarGroups"
	);
	get!(
		doc: "Get the number of the resource", 
		name: calendar_groups,
		path: "/calendarGroups/$count"
	);
}

impl CalendarGroupsIdApiClient {api_client_link!(calendars, CalendarsApiClient);
api_client_link_id!(calendar, CalendarsIdApiClient);

	delete!(
		doc: "Delete calendarGroup", 
		name: delete_calendar_groups,
		path: "/calendarGroups/{{RID}}"
	);
	get!(
		doc: "Get calendarGroup", 
		name: get_calendar_groups,
		path: "/calendarGroups/{{RID}}"
	);
	patch!(
		doc: "Update calendargroup", 
		name: update_calendar_groups,
		path: "/calendarGroups/{{RID}}",
		body: true
	);
	post!(
		doc: "Create Calendar", 
		name: create_calendars,
		path: "/calendarGroups/{{RID}}/calendars",
		body: true
	);
	get!(
		doc: "List calendars", 
		name: list_calendars,
		path: "/calendarGroups/{{RID}}/calendars"
	);
	get!(
		doc: "Get the number of the resource", 
		name: calendars,
		path: "/calendarGroups/{{RID}}/calendars/$count"
	);
}
