// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(AuthenticationApiClient, ResourceIdentity::Authentication);

impl AuthenticationApiClient {
	delete!(
		doc: "Delete navigation property authentication for users", 
		name: delete_authentication,
		path: "/authentication"
	);
	get!(
		doc: "Get authentication from users", 
		name: get_authentication,
		path: "/authentication"
	);
	patch!(
		doc: "Update the navigation property authentication in users", 
		name: update_authentication,
		path: "/authentication",
		body: true
	);
	post!(
		doc: "Create emailMethod", 
		name: create_email_methods,
		path: "/authentication/emailMethods",
		body: true
	);
	get!(
		doc: "List emailMethods", 
		name: list_email_methods,
		path: "/authentication/emailMethods"
	);
	get!(
		doc: "Get the number of the resource", 
		name: email_methods,
		path: "/authentication/emailMethods/$count"
	);
	delete!(
		doc: "Delete emailAuthenticationMethod", 
		name: delete_email_methods,
		path: "/authentication/emailMethods/{{id}}",
		params: email_authentication_method_id
	);
	get!(
		doc: "Get emailAuthenticationMethod", 
		name: get_email_methods,
		path: "/authentication/emailMethods/{{id}}",
		params: email_authentication_method_id
	);
	patch!(
		doc: "Update emailAuthenticationMethod", 
		name: update_email_methods,
		path: "/authentication/emailMethods/{{id}}",
		body: true,
		params: email_authentication_method_id
	);
	get!(
		doc: "List fido2AuthenticationMethod", 
		name: authentication,
		path: "/authentication/fido2Methods"
	);
	get!(
		doc: "Get the number of the resource", 
		name: fido_2_methods,
		path: "/authentication/fido2Methods/$count"
	);
	delete!(
		doc: "Delete fido2AuthenticationMethod", 
		name: delete_fido_2_authentication,
		path: "/authentication/fido2Methods/{{id}}",
		params: fido_2_authentication_method_id
	);
	get!(
		doc: "Get fido2AuthenticationMethod", 
		name: get_fido_2_authentication,
		path: "/authentication/fido2Methods/{{id}}",
		params: fido_2_authentication_method_id
	);
	post!(
		doc: "Create new navigation property to methods for users", 
		name: create_methods,
		path: "/authentication/methods",
		body: true
	);
	get!(
		doc: "List methods", 
		name: list_methods,
		path: "/authentication/methods"
	);
	get!(
		doc: "Get the number of the resource", 
		name: methods,
		path: "/authentication/methods/$count"
	);
	get!(
		doc: "Get authenticationMethod", 
		name: get_methods,
		path: "/authentication/methods/{{id}}",
		params: authentication_method_id
	);
	patch!(
		doc: "Update the navigation property methods in users", 
		name: update_methods,
		path: "/authentication/methods/{{id}}",
		body: true,
		params: authentication_method_id
	);
	post!(
		doc: "Invoke action resetPassword", 
		name: reset_password,
		path: "/authentication/methods/{{id}}/resetPassword",
		body: true,
		params: authentication_method_id
	);
	get!(
		doc: "List microsoftAuthenticatorAuthenticationMethods", 
		name: list_microsoft_authenticator_methods,
		path: "/authentication/microsoftAuthenticatorMethods"
	);
	get!(
		doc: "Get the number of the resource", 
		name: microsoft_authenticator_methods,
		path: "/authentication/microsoftAuthenticatorMethods/$count"
	);
	delete!(
		doc: "Delete microsoftAuthenticatorAuthenticationMethod", 
		name: delete_microsoft_authenticator_methods,
		path: "/authentication/microsoftAuthenticatorMethods/{{id}}",
		params: microsoft_authenticator_authentication_method_id
	);
	get!(
		doc: "Get microsoftAuthenticatorAuthenticationMethod", 
		name: get_microsoft_authenticator_methods,
		path: "/authentication/microsoftAuthenticatorMethods/{{id}}",
		params: microsoft_authenticator_authentication_method_id
	);
	get!(
		doc: "Get device from users", 
		name: get_device,
		path: "/authentication/microsoftAuthenticatorMethods/{{id}}/device",
		params: microsoft_authenticator_authentication_method_id
	);
	post!(
		doc: "Create new navigation property to operations for users", 
		name: create_operations,
		path: "/authentication/operations",
		body: true
	);
	get!(
		doc: "Get longRunningOperation", 
		name: list_operations,
		path: "/authentication/operations"
	);
	get!(
		doc: "Get the number of the resource", 
		name: operations,
		path: "/authentication/operations/$count"
	);
	delete!(
		doc: "Delete navigation property operations for users", 
		name: delete_operations,
		path: "/authentication/operations/{{id}}",
		params: long_running_operation_id
	);
	get!(
		doc: "Get longRunningOperation", 
		name: get_operations,
		path: "/authentication/operations/{{id}}",
		params: long_running_operation_id
	);
	patch!(
		doc: "Update the navigation property operations in users", 
		name: update_operations,
		path: "/authentication/operations/{{id}}",
		body: true,
		params: long_running_operation_id
	);
	post!(
		doc: "Create new navigation property to passwordMethods for users", 
		name: create_password_methods,
		path: "/authentication/passwordMethods",
		body: true
	);
	get!(
		doc: "List passwordMethods", 
		name: list_password_methods,
		path: "/authentication/passwordMethods"
	);
	get!(
		doc: "Get the number of the resource", 
		name: password_methods,
		path: "/authentication/passwordMethods/$count"
	);
	get!(
		doc: "Get passwordAuthenticationMethod", 
		name: get_password_methods,
		path: "/authentication/passwordMethods/{{id}}",
		params: password_authentication_method_id
	);
	post!(
		doc: "Create phoneMethod", 
		name: create_phone_methods,
		path: "/authentication/phoneMethods",
		body: true
	);
	get!(
		doc: "List phoneMethods", 
		name: list_phone_methods,
		path: "/authentication/phoneMethods"
	);
	get!(
		doc: "Get the number of the resource", 
		name: phone_methods,
		path: "/authentication/phoneMethods/$count"
	);
	delete!(
		doc: "Delete phoneAuthenticationMethod", 
		name: delete_phone_methods,
		path: "/authentication/phoneMethods/{{id}}",
		params: phone_authentication_method_id
	);
	get!(
		doc: "Get phoneAuthenticationMethod", 
		name: get_phone_methods,
		path: "/authentication/phoneMethods/{{id}}",
		params: phone_authentication_method_id
	);
	patch!(
		doc: "Update phoneAuthenticationMethod", 
		name: update_phone_methods,
		path: "/authentication/phoneMethods/{{id}}",
		body: true,
		params: phone_authentication_method_id
	);
	post!(
		doc: "Invoke action disableSmsSignIn", 
		name: disable_sms_sign_in,
		path: "/authentication/phoneMethods/{{id}}/disableSmsSignIn",
		params: phone_authentication_method_id
	);
	post!(
		doc: "Invoke action enableSmsSignIn", 
		name: enable_sms_sign_in,
		path: "/authentication/phoneMethods/{{id}}/enableSmsSignIn",
		params: phone_authentication_method_id
	);
	get!(
		doc: "List softwareOathMethods", 
		name: list_software_oath_methods,
		path: "/authentication/softwareOathMethods"
	);
	get!(
		doc: "Get the number of the resource", 
		name: software_oath_methods,
		path: "/authentication/softwareOathMethods/$count"
	);
	delete!(
		doc: "Delete softwareOathAuthenticationMethod", 
		name: delete_software_oath_methods,
		path: "/authentication/softwareOathMethods/{{id}}",
		params: software_oath_authentication_method_id
	);
	get!(
		doc: "Get softwareOathAuthenticationMethod", 
		name: get_software_oath_methods,
		path: "/authentication/softwareOathMethods/{{id}}",
		params: software_oath_authentication_method_id
	);
	post!(
		doc: "Create temporaryAccessPassMethod", 
		name: create_temporary_access_pass_methods,
		path: "/authentication/temporaryAccessPassMethods",
		body: true
	);
	get!(
		doc: "List temporaryAccessPassMethods", 
		name: list_temporary_access_pass_methods,
		path: "/authentication/temporaryAccessPassMethods"
	);
	get!(
		doc: "Get the number of the resource", 
		name: temporary_access_pass_methods,
		path: "/authentication/temporaryAccessPassMethods/$count"
	);
	delete!(
		doc: "Delete temporaryAccessPassAuthenticationMethod", 
		name: delete_temporary_access_pass_methods,
		path: "/authentication/temporaryAccessPassMethods/{{id}}",
		params: temporary_access_pass_authentication_method_id
	);
	get!(
		doc: "Get temporaryAccessPassAuthenticationMethod", 
		name: get_temporary_access_pass_methods,
		path: "/authentication/temporaryAccessPassMethods/{{id}}",
		params: temporary_access_pass_authentication_method_id
	);
	get!(
		doc: "List windowsHelloForBusinessAuthenticationMethods", 
		name: list_windows_hello_for_business_methods,
		path: "/authentication/windowsHelloForBusinessMethods"
	);
	get!(
		doc: "Get the number of the resource", 
		name: windows_hello_for_business_methods,
		path: "/authentication/windowsHelloForBusinessMethods/$count"
	);
	delete!(
		doc: "Delete windowsHelloForBusinessAuthenticationMethod", 
		name: delete_windows_hello_for_business_methods,
		path: "/authentication/windowsHelloForBusinessMethods/{{id}}",
		params: windows_hello_for_business_authentication_method_id
	);
	get!(
		doc: "Get windowsHelloForBusinessAuthenticationMethod", 
		name: get_windows_hello_for_business_methods,
		path: "/authentication/windowsHelloForBusinessMethods/{{id}}",
		params: windows_hello_for_business_authentication_method_id
	);
	get!(
		doc: "Get device from users", 
		name: get_windows_hello_for_business_methods_device,
		path: "/authentication/windowsHelloForBusinessMethods/{{id}}/device",
		params: windows_hello_for_business_authentication_method_id
	);
}
