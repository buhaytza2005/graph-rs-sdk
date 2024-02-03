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
}
