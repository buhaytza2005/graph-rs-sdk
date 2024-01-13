use graph_codegen::openapi::OpenApi;

fn main() {
    let open_api = OpenApi::default();

    open_api.debug_path_contains("worksheet");
}
