// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(BookingBusinessesApiClient, BookingBusinessesIdApiClient, ResourceIdentity::BookingBusinesses);

impl BookingBusinessesApiClient {
	post!(
		doc: "Create bookingBusiness", 
		name: create_booking_businesses,
		path: "/bookingBusinesses",
		body: true
	);
	get!(
		doc: "List bookingBusinesses", 
		name: list_booking_businesses,
		path: "/bookingBusinesses"
	);
	get!(
		doc: "Get the number of the resource", 
		name: booking_businesses,
		path: "/bookingBusinesses/$count"
	);
}

impl BookingBusinessesIdApiClient {
	delete!(
		doc: "Delete bookingBusiness", 
		name: delete_booking_businesses,
		path: "/bookingBusinesses/{{RID}}"
	);
	get!(
		doc: "Get bookingBusiness", 
		name: get_booking_businesses,
		path: "/bookingBusinesses/{{RID}}"
	);
	patch!(
		doc: "Update bookingbusiness", 
		name: update_booking_businesses,
		path: "/bookingBusinesses/{{RID}}",
		body: true
	);
	post!(
		doc: "Create bookingAppointment", 
		name: create_appointments,
		path: "/bookingBusinesses/{{RID}}/appointments",
		body: true
	);
	get!(
		doc: "List appointments", 
		name: list_appointments,
		path: "/bookingBusinesses/{{RID}}/appointments"
	);
	get!(
		doc: "Get the number of the resource", 
		name: appointments,
		path: "/bookingBusinesses/{{RID}}/appointments/$count"
	);
	delete!(
		doc: "Delete bookingAppointment", 
		name: delete_appointments,
		path: "/bookingBusinesses/{{RID}}/appointments/{{id}}",
		params: booking_appointment_id
	);
	get!(
		doc: "Get bookingAppointment", 
		name: get_appointments,
		path: "/bookingBusinesses/{{RID}}/appointments/{{id}}",
		params: booking_appointment_id
	);
	patch!(
		doc: "Update bookingAppointment", 
		name: update_appointments,
		path: "/bookingBusinesses/{{RID}}/appointments/{{id}}",
		body: true,
		params: booking_appointment_id
	);
	post!(
		doc: "Invoke action cancel", 
		name: cancel,
		path: "/bookingBusinesses/{{RID}}/appointments/{{id}}/cancel",
		body: true,
		params: booking_appointment_id
	);
	post!(
		doc: "Create new navigation property to calendarView for solutions", 
		name: create_calendar_view,
		path: "/bookingBusinesses/{{RID}}/calendarView",
		body: true
	);
	get!(
		doc: "Get calendarView from solutions", 
		name: list_calendar_view,
		path: "/bookingBusinesses/{{RID}}/calendarView"
	);
	get!(
		doc: "Get the number of the resource", 
		name: calendar_view,
		path: "/bookingBusinesses/{{RID}}/calendarView/$count"
	);
	delete!(
		doc: "Delete navigation property calendarView for solutions", 
		name: delete_calendar_view,
		path: "/bookingBusinesses/{{RID}}/calendarView/{{id}}",
		params: booking_appointment_id
	);
	get!(
		doc: "Get calendarView from solutions", 
		name: get_calendar_view,
		path: "/bookingBusinesses/{{RID}}/calendarView/{{id}}",
		params: booking_appointment_id
	);
	patch!(
		doc: "Update the navigation property calendarView in solutions", 
		name: update_calendar_view,
		path: "/bookingBusinesses/{{RID}}/calendarView/{{id}}",
		body: true,
		params: booking_appointment_id
	);
	post!(
		doc: "Invoke action cancel", 
		name: cancel,
		path: "/bookingBusinesses/{{RID}}/calendarView/{{id}}/cancel",
		body: true,
		params: booking_appointment_id
	);
	post!(
		doc: "Create bookingCustomQuestion", 
		name: create_custom_questions,
		path: "/bookingBusinesses/{{RID}}/customQuestions",
		body: true
	);
	get!(
		doc: "List customQuestions", 
		name: list_custom_questions,
		path: "/bookingBusinesses/{{RID}}/customQuestions"
	);
	get!(
		doc: "Get the number of the resource", 
		name: custom_questions,
		path: "/bookingBusinesses/{{RID}}/customQuestions/$count"
	);
	delete!(
		doc: "Delete bookingCustomQuestion", 
		name: delete_custom_questions,
		path: "/bookingBusinesses/{{RID}}/customQuestions/{{id}}",
		params: booking_custom_question_id
	);
	get!(
		doc: "Get bookingCustomQuestion", 
		name: get_custom_questions,
		path: "/bookingBusinesses/{{RID}}/customQuestions/{{id}}",
		params: booking_custom_question_id
	);
	patch!(
		doc: "Update bookingCustomQuestion", 
		name: update_custom_questions,
		path: "/bookingBusinesses/{{RID}}/customQuestions/{{id}}",
		body: true,
		params: booking_custom_question_id
	);
	post!(
		doc: "Create bookingCustomer", 
		name: create_customers,
		path: "/bookingBusinesses/{{RID}}/customers",
		body: true
	);
	get!(
		doc: "List customers", 
		name: list_customers,
		path: "/bookingBusinesses/{{RID}}/customers"
	);
	get!(
		doc: "Get the number of the resource", 
		name: customers,
		path: "/bookingBusinesses/{{RID}}/customers/$count"
	);
	delete!(
		doc: "Delete bookingCustomer", 
		name: delete_customers,
		path: "/bookingBusinesses/{{RID}}/customers/{{id}}",
		params: booking_customer_base_id
	);
	get!(
		doc: "Get bookingCustomer", 
		name: get_customers,
		path: "/bookingBusinesses/{{RID}}/customers/{{id}}",
		params: booking_customer_base_id
	);
	patch!(
		doc: "Update bookingCustomer", 
		name: update_customers,
		path: "/bookingBusinesses/{{RID}}/customers/{{id}}",
		body: true,
		params: booking_customer_base_id
	);
	post!(
		doc: "Invoke action getStaffAvailability", 
		name: get_staff_availability,
		path: "/bookingBusinesses/{{RID}}/getStaffAvailability",
		body: true
	);
	post!(
		doc: "Invoke action publish", 
		name: publish,
		path: "/bookingBusinesses/{{RID}}/publish"
	);
	post!(
		doc: "Create bookingService", 
		name: create_services,
		path: "/bookingBusinesses/{{RID}}/services",
		body: true
	);
	get!(
		doc: "List services", 
		name: list_services,
		path: "/bookingBusinesses/{{RID}}/services"
	);
	get!(
		doc: "Get the number of the resource", 
		name: services,
		path: "/bookingBusinesses/{{RID}}/services/$count"
	);
	delete!(
		doc: "Delete bookingService", 
		name: delete_services,
		path: "/bookingBusinesses/{{RID}}/services/{{id}}",
		params: booking_service_id
	);
	get!(
		doc: "Get bookingService", 
		name: get_services,
		path: "/bookingBusinesses/{{RID}}/services/{{id}}",
		params: booking_service_id
	);
	patch!(
		doc: "Update bookingservice", 
		name: update_services,
		path: "/bookingBusinesses/{{RID}}/services/{{id}}",
		body: true,
		params: booking_service_id
	);
	post!(
		doc: "Create bookingStaffMember", 
		name: create_staff_members,
		path: "/bookingBusinesses/{{RID}}/staffMembers",
		body: true
	);
	get!(
		doc: "List staffMembers", 
		name: list_staff_members,
		path: "/bookingBusinesses/{{RID}}/staffMembers"
	);
	get!(
		doc: "Get the number of the resource", 
		name: staff_members,
		path: "/bookingBusinesses/{{RID}}/staffMembers/$count"
	);
	delete!(
		doc: "Delete bookingStaffMember", 
		name: delete_staff_members,
		path: "/bookingBusinesses/{{RID}}/staffMembers/{{id}}",
		params: booking_staff_member_base_id
	);
	get!(
		doc: "Get bookingStaffMember", 
		name: get_staff_members,
		path: "/bookingBusinesses/{{RID}}/staffMembers/{{id}}",
		params: booking_staff_member_base_id
	);
	patch!(
		doc: "Update bookingstaffmember", 
		name: update_staff_members,
		path: "/bookingBusinesses/{{RID}}/staffMembers/{{id}}",
		body: true,
		params: booking_staff_member_base_id
	);
	post!(
		doc: "Invoke action unpublish", 
		name: unpublish,
		path: "/bookingBusinesses/{{RID}}/unpublish"
	);
}
