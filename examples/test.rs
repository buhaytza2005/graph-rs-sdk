use graph_codegen::macros::OpenApiParser;
use graph_codegen::openapi::OpenApi;
use graph_codegen::settings::get_write_configuration;
use graph_core::resource::ResourceIdentity;

fn main() {

    OpenApi::write(get_write_configuration(ResourceIdentity::Solutions));
    //OpenApi::write(get_write_configuration(ResourceIdentity::Users));
    let open_api = OpenApi::default();

    open_api.debug_path_contains("solutions")
}
