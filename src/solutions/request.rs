// GENERATED CODE

use crate::api_default_imports::*;
use crate::users::*;

resource_api_client!(SolutionsApiClient, ResourceIdentity::Solutions);

impl SolutionsApiClient {api_client_link!(booking_businesses, BookingBusinessesApiClient);
api_client_link_id!(booking_business, BookingBusinessesIdApiClient);

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
