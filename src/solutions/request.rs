// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(SolutionsApiClient, ResourceIdentity::Solutions);

impl SolutionsApiClient {
	get!(
		doc: "Get solutions", 
		name: get_solutions_root,
		path: "/solutions"
	);
	patch!(
		doc: "Update solutions", 
		name: update_solutions_root,
		path: "/solutions",
		body: true
	);
	post!(
		doc: "Create bookingBusiness", 
		name: create_booking_businesses,
		path: "/solutions/bookingBusinesses",
		body: true
	);
	get!(
		doc: "List bookingBusinesses", 
		name: list_booking_businesses,
		path: "/solutions/bookingBusinesses"
	);
	get!(
		doc: "Get the number of the resource", 
		name: booking_businesses,
		path: "/solutions/bookingBusinesses/$count"
	);
	delete!(
		doc: "Delete bookingBusiness", 
		name: delete_booking_businesses,
		path: "/solutions/bookingBusinesses/{{id}}",
		params: booking_business_id
	);
	get!(
		doc: "Get bookingBusiness", 
		name: get_booking_businesses,
		path: "/solutions/bookingBusinesses/{{id}}",
		params: booking_business_id
	);
	patch!(
		doc: "Update bookingbusiness", 
		name: update_booking_businesses,
		path: "/solutions/bookingBusinesses/{{id}}",
		body: true,
		params: booking_business_id
	);
	post!(
		doc: "Create bookingAppointment", 
		name: create_appointments,
		path: "/solutions/bookingBusinesses/{{id}}/appointments",
		body: true,
		params: booking_business_id
	);
	get!(
		doc: "List appointments", 
		name: list_appointments,
		path: "/solutions/bookingBusinesses/{{id}}/appointments",
		params: booking_business_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: appointments,
		path: "/solutions/bookingBusinesses/{{id}}/appointments/$count",
		params: booking_business_id
	);
	delete!(
		doc: "Delete bookingAppointment", 
		name: delete_appointments,
		path: "/solutions/bookingBusinesses/{{id}}/appointments/{{id2}}",
		params: booking_business_id, booking_appointment_id
	);
	get!(
		doc: "Get bookingAppointment", 
		name: get_appointments,
		path: "/solutions/bookingBusinesses/{{id}}/appointments/{{id2}}",
		params: booking_business_id, booking_appointment_id
	);
	patch!(
		doc: "Update bookingAppointment", 
		name: update_appointments,
		path: "/solutions/bookingBusinesses/{{id}}/appointments/{{id2}}",
		body: true,
		params: booking_business_id, booking_appointment_id
	);
	post!(
		doc: "Invoke action cancel", 
		name: cancel,
		path: "/solutions/bookingBusinesses/{{id}}/appointments/{{id2}}/cancel",
		body: true,
		params: booking_business_id, booking_appointment_id
	);
	post!(
		doc: "Create new navigation property to calendarView for solutions", 
		name: create_calendar_view,
		path: "/solutions/bookingBusinesses/{{id}}/calendarView",
		body: true,
		params: booking_business_id
	);
	get!(
		doc: "Get calendarView from solutions", 
		name: list_calendar_view,
		path: "/solutions/bookingBusinesses/{{id}}/calendarView",
		params: booking_business_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: calendar_view,
		path: "/solutions/bookingBusinesses/{{id}}/calendarView/$count",
		params: booking_business_id
	);
	delete!(
		doc: "Delete navigation property calendarView for solutions", 
		name: delete_calendar_view,
		path: "/solutions/bookingBusinesses/{{id}}/calendarView/{{id2}}",
		params: booking_business_id, booking_appointment_id
	);
	get!(
		doc: "Get calendarView from solutions", 
		name: get_calendar_view,
		path: "/solutions/bookingBusinesses/{{id}}/calendarView/{{id2}}",
		params: booking_business_id, booking_appointment_id
	);
	patch!(
		doc: "Update the navigation property calendarView in solutions", 
		name: update_calendar_view,
		path: "/solutions/bookingBusinesses/{{id}}/calendarView/{{id2}}",
		body: true,
		params: booking_business_id, booking_appointment_id
	);
	post!(
		doc: "Invoke action cancel", 
		name: cancel,
		path: "/solutions/bookingBusinesses/{{id}}/calendarView/{{id2}}/cancel",
		body: true,
		params: booking_business_id, booking_appointment_id
	);
	post!(
		doc: "Create bookingCustomQuestion", 
		name: create_custom_questions,
		path: "/solutions/bookingBusinesses/{{id}}/customQuestions",
		body: true,
		params: booking_business_id
	);
	get!(
		doc: "List customQuestions", 
		name: list_custom_questions,
		path: "/solutions/bookingBusinesses/{{id}}/customQuestions",
		params: booking_business_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: custom_questions,
		path: "/solutions/bookingBusinesses/{{id}}/customQuestions/$count",
		params: booking_business_id
	);
	delete!(
		doc: "Delete bookingCustomQuestion", 
		name: delete_custom_questions,
		path: "/solutions/bookingBusinesses/{{id}}/customQuestions/{{id2}}",
		params: booking_business_id, booking_custom_question_id
	);
	get!(
		doc: "Get bookingCustomQuestion", 
		name: get_custom_questions,
		path: "/solutions/bookingBusinesses/{{id}}/customQuestions/{{id2}}",
		params: booking_business_id, booking_custom_question_id
	);
	patch!(
		doc: "Update bookingCustomQuestion", 
		name: update_custom_questions,
		path: "/solutions/bookingBusinesses/{{id}}/customQuestions/{{id2}}",
		body: true,
		params: booking_business_id, booking_custom_question_id
	);
	post!(
		doc: "Create bookingCustomer", 
		name: create_customers,
		path: "/solutions/bookingBusinesses/{{id}}/customers",
		body: true,
		params: booking_business_id
	);
	get!(
		doc: "List customers", 
		name: list_customers,
		path: "/solutions/bookingBusinesses/{{id}}/customers",
		params: booking_business_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: customers,
		path: "/solutions/bookingBusinesses/{{id}}/customers/$count",
		params: booking_business_id
	);
	delete!(
		doc: "Delete bookingCustomer", 
		name: delete_customers,
		path: "/solutions/bookingBusinesses/{{id}}/customers/{{id2}}",
		params: booking_business_id, booking_customer_base_id
	);
	get!(
		doc: "Get bookingCustomer", 
		name: get_customers,
		path: "/solutions/bookingBusinesses/{{id}}/customers/{{id2}}",
		params: booking_business_id, booking_customer_base_id
	);
	patch!(
		doc: "Update bookingCustomer", 
		name: update_customers,
		path: "/solutions/bookingBusinesses/{{id}}/customers/{{id2}}",
		body: true,
		params: booking_business_id, booking_customer_base_id
	);
	post!(
		doc: "Invoke action getStaffAvailability", 
		name: get_staff_availability,
		path: "/solutions/bookingBusinesses/{{id}}/getStaffAvailability",
		body: true,
		params: booking_business_id
	);
	post!(
		doc: "Invoke action publish", 
		name: publish,
		path: "/solutions/bookingBusinesses/{{id}}/publish",
		params: booking_business_id
	);
	post!(
		doc: "Create bookingService", 
		name: create_services,
		path: "/solutions/bookingBusinesses/{{id}}/services",
		body: true,
		params: booking_business_id
	);
	get!(
		doc: "List services", 
		name: list_services,
		path: "/solutions/bookingBusinesses/{{id}}/services",
		params: booking_business_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: services,
		path: "/solutions/bookingBusinesses/{{id}}/services/$count",
		params: booking_business_id
	);
	delete!(
		doc: "Delete bookingService", 
		name: delete_services,
		path: "/solutions/bookingBusinesses/{{id}}/services/{{id2}}",
		params: booking_business_id, booking_service_id
	);
	get!(
		doc: "Get bookingService", 
		name: get_services,
		path: "/solutions/bookingBusinesses/{{id}}/services/{{id2}}",
		params: booking_business_id, booking_service_id
	);
	patch!(
		doc: "Update bookingservice", 
		name: update_services,
		path: "/solutions/bookingBusinesses/{{id}}/services/{{id2}}",
		body: true,
		params: booking_business_id, booking_service_id
	);
	post!(
		doc: "Create bookingStaffMember", 
		name: create_staff_members,
		path: "/solutions/bookingBusinesses/{{id}}/staffMembers",
		body: true,
		params: booking_business_id
	);
	get!(
		doc: "List staffMembers", 
		name: list_staff_members,
		path: "/solutions/bookingBusinesses/{{id}}/staffMembers",
		params: booking_business_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: staff_members,
		path: "/solutions/bookingBusinesses/{{id}}/staffMembers/$count",
		params: booking_business_id
	);
	delete!(
		doc: "Delete bookingStaffMember", 
		name: delete_staff_members,
		path: "/solutions/bookingBusinesses/{{id}}/staffMembers/{{id2}}",
		params: booking_business_id, booking_staff_member_base_id
	);
	get!(
		doc: "Get bookingStaffMember", 
		name: get_staff_members,
		path: "/solutions/bookingBusinesses/{{id}}/staffMembers/{{id2}}",
		params: booking_business_id, booking_staff_member_base_id
	);
	patch!(
		doc: "Update bookingstaffmember", 
		name: update_staff_members,
		path: "/solutions/bookingBusinesses/{{id}}/staffMembers/{{id2}}",
		body: true,
		params: booking_business_id, booking_staff_member_base_id
	);
	post!(
		doc: "Invoke action unpublish", 
		name: unpublish,
		path: "/solutions/bookingBusinesses/{{id}}/unpublish",
		params: booking_business_id
	);
	post!(
		doc: "Create new navigation property to bookingCurrencies for solutions", 
		name: create_booking_currencies,
		path: "/solutions/bookingCurrencies",
		body: true
	);
	get!(
		doc: "List bookingCurrencies", 
		name: list_booking_currencies,
		path: "/solutions/bookingCurrencies"
	);
	get!(
		doc: "Get the number of the resource", 
		name: booking_currencies,
		path: "/solutions/bookingCurrencies/$count"
	);
	delete!(
		doc: "Delete navigation property bookingCurrencies for solutions", 
		name: delete_booking_currencies,
		path: "/solutions/bookingCurrencies/{{id}}",
		params: booking_currency_id
	);
	get!(
		doc: "Get bookingCurrency", 
		name: get_booking_currencies,
		path: "/solutions/bookingCurrencies/{{id}}",
		params: booking_currency_id
	);
	patch!(
		doc: "Update the navigation property bookingCurrencies in solutions", 
		name: update_booking_currencies,
		path: "/solutions/bookingCurrencies/{{id}}",
		body: true,
		params: booking_currency_id
	);
	delete!(
		doc: "Delete navigation property virtualEvents for solutions", 
		name: delete_virtual_events,
		path: "/solutions/virtualEvents"
	);
	get!(
		doc: "Get virtualEvents from solutions", 
		name: get_virtual_events,
		path: "/solutions/virtualEvents"
	);
	patch!(
		doc: "Update the navigation property virtualEvents in solutions", 
		name: update_virtual_events,
		path: "/solutions/virtualEvents",
		body: true
	);
	post!(
		doc: "Create new navigation property to events for solutions", 
		name: create_events,
		path: "/solutions/virtualEvents/events",
		body: true
	);
	get!(
		doc: "Get events from solutions", 
		name: list_events,
		path: "/solutions/virtualEvents/events"
	);
	get!(
		doc: "Get the number of the resource", 
		name: events,
		path: "/solutions/virtualEvents/events/$count"
	);
	delete!(
		doc: "Delete navigation property events for solutions", 
		name: delete_events,
		path: "/solutions/virtualEvents/events/{{id}}",
		params: virtual_event_id
	);
	get!(
		doc: "Get events from solutions", 
		name: get_events,
		path: "/solutions/virtualEvents/events/{{id}}",
		params: virtual_event_id
	);
	patch!(
		doc: "Update the navigation property events in solutions", 
		name: update_events,
		path: "/solutions/virtualEvents/events/{{id}}",
		body: true,
		params: virtual_event_id
	);
	post!(
		doc: "Create new navigation property to sessions for solutions", 
		name: create_sessions,
		path: "/solutions/virtualEvents/events/{{id}}/sessions",
		body: true,
		params: virtual_event_id
	);
	get!(
		doc: "Get sessions from solutions", 
		name: list_sessions,
		path: "/solutions/virtualEvents/events/{{id}}/sessions",
		params: virtual_event_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: sessions,
		path: "/solutions/virtualEvents/events/{{id}}/sessions/$count",
		params: virtual_event_id
	);
	delete!(
		doc: "Delete navigation property sessions for solutions", 
		name: delete_sessions,
		path: "/solutions/virtualEvents/events/{{id}}/sessions/{{id2}}",
		params: virtual_event_id, virtual_event_session_id
	);
	get!(
		doc: "Get sessions from solutions", 
		name: get_sessions,
		path: "/solutions/virtualEvents/events/{{id}}/sessions/{{id2}}",
		params: virtual_event_id, virtual_event_session_id
	);
	patch!(
		doc: "Update the navigation property sessions in solutions", 
		name: update_sessions,
		path: "/solutions/virtualEvents/events/{{id}}/sessions/{{id2}}",
		body: true,
		params: virtual_event_id, virtual_event_session_id
	);
	post!(
		doc: "Create new navigation property to attendanceReports for solutions", 
		name: create_attendance_reports,
		path: "/solutions/virtualEvents/events/{{id}}/sessions/{{id2}}/attendanceReports",
		body: true,
		params: virtual_event_id, virtual_event_session_id
	);
	get!(
		doc: "Get attendanceReports from solutions", 
		name: list_attendance_reports,
		path: "/solutions/virtualEvents/events/{{id}}/sessions/{{id2}}/attendanceReports",
		params: virtual_event_id, virtual_event_session_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: attendance_reports,
		path: "/solutions/virtualEvents/events/{{id}}/sessions/{{id2}}/attendanceReports/$count",
		params: virtual_event_id, virtual_event_session_id
	);
	delete!(
		doc: "Delete navigation property attendanceReports for solutions", 
		name: delete_attendance_reports,
		path: "/solutions/virtualEvents/events/{{id}}/sessions/{{id2}}/attendanceReports/{{id3}}",
		params: virtual_event_id, virtual_event_session_id, meeting_attendance_report_id
	);
	get!(
		doc: "Get attendanceReports from solutions", 
		name: get_attendance_reports,
		path: "/solutions/virtualEvents/events/{{id}}/sessions/{{id2}}/attendanceReports/{{id3}}",
		params: virtual_event_id, virtual_event_session_id, meeting_attendance_report_id
	);
	patch!(
		doc: "Update the navigation property attendanceReports in solutions", 
		name: update_attendance_reports,
		path: "/solutions/virtualEvents/events/{{id}}/sessions/{{id2}}/attendanceReports/{{id3}}",
		body: true,
		params: virtual_event_id, virtual_event_session_id, meeting_attendance_report_id
	);
	post!(
		doc: "Create new navigation property to attendanceRecords for solutions", 
		name: create_attendance_records,
		path: "/solutions/virtualEvents/events/{{id}}/sessions/{{id2}}/attendanceReports/{{id3}}/attendanceRecords",
		body: true,
		params: virtual_event_id, virtual_event_session_id, meeting_attendance_report_id
	);
	get!(
		doc: "List attendanceRecords", 
		name: list_attendance_records,
		path: "/solutions/virtualEvents/events/{{id}}/sessions/{{id2}}/attendanceReports/{{id3}}/attendanceRecords",
		params: virtual_event_id, virtual_event_session_id, meeting_attendance_report_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: attendance_records,
		path: "/solutions/virtualEvents/events/{{id}}/sessions/{{id2}}/attendanceReports/{{id3}}/attendanceRecords/$count",
		params: virtual_event_id, virtual_event_session_id, meeting_attendance_report_id
	);
	delete!(
		doc: "Delete navigation property attendanceRecords for solutions", 
		name: delete_attendance_records,
		path: "/solutions/virtualEvents/events/{{id}}/sessions/{{id2}}/attendanceReports/{{id3}}/attendanceRecords/{{id4}}",
		params: virtual_event_id, virtual_event_session_id, meeting_attendance_report_id, attendance_record_id
	);
	get!(
		doc: "Get attendanceRecords from solutions", 
		name: get_attendance_records,
		path: "/solutions/virtualEvents/events/{{id}}/sessions/{{id2}}/attendanceReports/{{id3}}/attendanceRecords/{{id4}}",
		params: virtual_event_id, virtual_event_session_id, meeting_attendance_report_id, attendance_record_id
	);
	patch!(
		doc: "Update the navigation property attendanceRecords in solutions", 
		name: update_attendance_records,
		path: "/solutions/virtualEvents/events/{{id}}/sessions/{{id2}}/attendanceReports/{{id3}}/attendanceRecords/{{id4}}",
		body: true,
		params: virtual_event_id, virtual_event_session_id, meeting_attendance_report_id, attendance_record_id
	);
	post!(
		doc: "Create new navigation property to webinars for solutions", 
		name: create_webinars,
		path: "/solutions/virtualEvents/webinars",
		body: true
	);
	get!(
		doc: "List webinars", 
		name: list_webinars,
		path: "/solutions/virtualEvents/webinars"
	);
	get!(
		doc: "Get the number of the resource", 
		name: webinars,
		path: "/solutions/virtualEvents/webinars/$count"
	);
	get!(
		doc: "Invoke function getByUserIdAndRole", 
		name: get_by_user_id_and_role,
		path: "/solutions/virtualEvents/webinars/getByUserIdAndRole(userId='{{id}}',role='{{id2}}')",
		params: user_id, role
	);
	get!(
		doc: "Invoke function getByUserRole", 
		name: get_by_user_role,
		path: "/solutions/virtualEvents/webinars/getByUserRole(role='{{id}}')",
		params: role
	);
	delete!(
		doc: "Delete navigation property webinars for solutions", 
		name: delete_webinars,
		path: "/solutions/virtualEvents/webinars/{{id}}",
		params: virtual_event_webinar_id
	);
	get!(
		doc: "Get webinars from solutions", 
		name: get_webinars,
		path: "/solutions/virtualEvents/webinars/{{id}}",
		params: virtual_event_webinar_id
	);
	patch!(
		doc: "Update the navigation property webinars in solutions", 
		name: update_webinars,
		path: "/solutions/virtualEvents/webinars/{{id}}",
		body: true,
		params: virtual_event_webinar_id
	);
	post!(
		doc: "Create new navigation property to registrations for solutions", 
		name: create_registrations,
		path: "/solutions/virtualEvents/webinars/{{id}}/registrations",
		body: true,
		params: virtual_event_webinar_id
	);
	get!(
		doc: "List virtualEventRegistrations", 
		name: list_registrations,
		path: "/solutions/virtualEvents/webinars/{{id}}/registrations",
		params: virtual_event_webinar_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: registrations,
		path: "/solutions/virtualEvents/webinars/{{id}}/registrations/$count",
		params: virtual_event_webinar_id
	);
	delete!(
		doc: "Delete navigation property registrations for solutions", 
		name: delete_registrations,
		path: "/solutions/virtualEvents/webinars/{{id}}/registrations/{{id2}}",
		params: virtual_event_webinar_id, virtual_event_registration_id
	);
	get!(
		doc: "Get virtualEventRegistration", 
		name: get_registrations,
		path: "/solutions/virtualEvents/webinars/{{id}}/registrations/{{id2}}",
		params: virtual_event_webinar_id, virtual_event_registration_id
	);
	patch!(
		doc: "Update the navigation property registrations in solutions", 
		name: update_registrations,
		path: "/solutions/virtualEvents/webinars/{{id}}/registrations/{{id2}}",
		body: true,
		params: virtual_event_webinar_id, virtual_event_registration_id
	);
	post!(
		doc: "Create new navigation property to sessions for solutions", 
		name: create_sessions,
		path: "/solutions/virtualEvents/webinars/{{id}}/sessions",
		body: true,
		params: virtual_event_webinar_id
	);
	get!(
		doc: "Get sessions from solutions", 
		name: list_sessions,
		path: "/solutions/virtualEvents/webinars/{{id}}/sessions",
		params: virtual_event_webinar_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: sessions,
		path: "/solutions/virtualEvents/webinars/{{id}}/sessions/$count",
		params: virtual_event_webinar_id
	);
	delete!(
		doc: "Delete navigation property sessions for solutions", 
		name: delete_sessions,
		path: "/solutions/virtualEvents/webinars/{{id}}/sessions/{{id2}}",
		params: virtual_event_webinar_id, virtual_event_session_id
	);
	get!(
		doc: "Get sessions from solutions", 
		name: get_sessions,
		path: "/solutions/virtualEvents/webinars/{{id}}/sessions/{{id2}}",
		params: virtual_event_webinar_id, virtual_event_session_id
	);
	patch!(
		doc: "Update the navigation property sessions in solutions", 
		name: update_sessions,
		path: "/solutions/virtualEvents/webinars/{{id}}/sessions/{{id2}}",
		body: true,
		params: virtual_event_webinar_id, virtual_event_session_id
	);
	post!(
		doc: "Create new navigation property to attendanceReports for solutions", 
		name: create_attendance_reports,
		path: "/solutions/virtualEvents/webinars/{{id}}/sessions/{{id2}}/attendanceReports",
		body: true,
		params: virtual_event_webinar_id, virtual_event_session_id
	);
	get!(
		doc: "Get attendanceReports from solutions", 
		name: list_attendance_reports,
		path: "/solutions/virtualEvents/webinars/{{id}}/sessions/{{id2}}/attendanceReports",
		params: virtual_event_webinar_id, virtual_event_session_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: attendance_reports,
		path: "/solutions/virtualEvents/webinars/{{id}}/sessions/{{id2}}/attendanceReports/$count",
		params: virtual_event_webinar_id, virtual_event_session_id
	);
	delete!(
		doc: "Delete navigation property attendanceReports for solutions", 
		name: delete_attendance_reports,
		path: "/solutions/virtualEvents/webinars/{{id}}/sessions/{{id2}}/attendanceReports/{{id3}}",
		params: virtual_event_webinar_id, virtual_event_session_id, meeting_attendance_report_id
	);
	get!(
		doc: "Get attendanceReports from solutions", 
		name: get_attendance_reports,
		path: "/solutions/virtualEvents/webinars/{{id}}/sessions/{{id2}}/attendanceReports/{{id3}}",
		params: virtual_event_webinar_id, virtual_event_session_id, meeting_attendance_report_id
	);
	patch!(
		doc: "Update the navigation property attendanceReports in solutions", 
		name: update_attendance_reports,
		path: "/solutions/virtualEvents/webinars/{{id}}/sessions/{{id2}}/attendanceReports/{{id3}}",
		body: true,
		params: virtual_event_webinar_id, virtual_event_session_id, meeting_attendance_report_id
	);
	post!(
		doc: "Create new navigation property to attendanceRecords for solutions", 
		name: create_attendance_records,
		path: "/solutions/virtualEvents/webinars/{{id}}/sessions/{{id2}}/attendanceReports/{{id3}}/attendanceRecords",
		body: true,
		params: virtual_event_webinar_id, virtual_event_session_id, meeting_attendance_report_id
	);
	get!(
		doc: "List attendanceRecords", 
		name: list_attendance_records,
		path: "/solutions/virtualEvents/webinars/{{id}}/sessions/{{id2}}/attendanceReports/{{id3}}/attendanceRecords",
		params: virtual_event_webinar_id, virtual_event_session_id, meeting_attendance_report_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: attendance_records,
		path: "/solutions/virtualEvents/webinars/{{id}}/sessions/{{id2}}/attendanceReports/{{id3}}/attendanceRecords/$count",
		params: virtual_event_webinar_id, virtual_event_session_id, meeting_attendance_report_id
	);
	delete!(
		doc: "Delete navigation property attendanceRecords for solutions", 
		name: delete_attendance_records,
		path: "/solutions/virtualEvents/webinars/{{id}}/sessions/{{id2}}/attendanceReports/{{id3}}/attendanceRecords/{{id4}}",
		params: virtual_event_webinar_id, virtual_event_session_id, meeting_attendance_report_id, attendance_record_id
	);
	get!(
		doc: "Get attendanceRecords from solutions", 
		name: get_attendance_records,
		path: "/solutions/virtualEvents/webinars/{{id}}/sessions/{{id2}}/attendanceReports/{{id3}}/attendanceRecords/{{id4}}",
		params: virtual_event_webinar_id, virtual_event_session_id, meeting_attendance_report_id, attendance_record_id
	);
	patch!(
		doc: "Update the navigation property attendanceRecords in solutions", 
		name: update_attendance_records,
		path: "/solutions/virtualEvents/webinars/{{id}}/sessions/{{id2}}/attendanceReports/{{id3}}/attendanceRecords/{{id4}}",
		body: true,
		params: virtual_event_webinar_id, virtual_event_session_id, meeting_attendance_report_id, attendance_record_id
	);
}
