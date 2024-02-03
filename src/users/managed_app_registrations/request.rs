// GENERATED CODE

use crate::api_default_imports::*;
use crate::device_app_management::*;

resource_api_client!(ManagedAppRegistrationsApiClient, ManagedAppRegistrationsIdApiClient, ResourceIdentity::ManagedAppRegistrations);

impl ManagedAppRegistrationsApiClient {api_client_link!(intended_policies, ManagedAppRegistrationsIntendedPoliciesApiClient);
api_client_link_id!(intended_policies_id, ManagedAppRegistrationsIntendedPoliciesIdApiClient);
api_client_link_id!(applied_policies_id, ManagedAppRegistrationsAppliedPoliciesIdApiClient);
api_client_link!(applied_policies, ManagedAppRegistrationsAppliedPoliciesApiClient);

	get!(
		doc: "Get managedAppRegistrations from users", 
		name: list_managed_app_registrations,
		path: "/managedAppRegistrations"
	);
	get!(
		doc: "Get the number of the resource", 
		name: managed_app_registrations,
		path: "/managedAppRegistrations/$count"
	);
}

impl ManagedAppRegistrationsIdApiClient {api_client_link_id!(intended_policies_id, ManagedAppRegistrationsIntendedPoliciesIdApiClient);
api_client_link!(intended_policies, ManagedAppRegistrationsIntendedPoliciesApiClient);
api_client_link_id!(applied_policies_id, ManagedAppRegistrationsAppliedPoliciesIdApiClient);
api_client_link!(applied_policies, ManagedAppRegistrationsAppliedPoliciesApiClient);

	get!(
		doc: "Get managedAppRegistrations from users", 
		name: get_managed_app_registrations,
		path: "/managedAppRegistrations/{{RID}}"
	);
}
