// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    SitesItemsVersionsApiClient,
    SitesItemsVersionsIdApiClient,
    ResourceIdentity::SitesItemsVersions
);

impl SitesItemsVersionsApiClient {
    post!(
        doc: "Create new navigation property to versions for sites",
        name: create_versions,
        path: "/versions",
        body: true
    );
    get!(
        doc: "Listing versions of a ListItem",
        name: list_versions,
        path: "/versions"
    );
    get!(
        doc: "Get the number of the resource",
        name: versions,
        path: "/versions/$count"
    );
}

impl SitesItemsVersionsIdApiClient {
    delete!(
        doc: "Delete navigation property versions for sites",
        name: delete_versions,
        path: "/versions/{{RID}}"
    );
    get!(
        doc: "Get a ListItemVersion resource",
        name: get_versions,
        path: "/versions/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property versions in sites",
        name: update_versions,
        path: "/versions/{{RID}}",
        body: true
    );
    delete!(
        doc: "Delete navigation property fields for sites",
        name: delete_fields,
        path: "/versions/{{RID}}/fields"
    );
    get!(
        doc: "Get fields from sites",
        name: get_fields,
        path: "/versions/{{RID}}/fields"
    );
    patch!(
        doc: "Update the navigation property fields in sites",
        name: update_fields,
        path: "/versions/{{RID}}/fields",
        body: true
    );
    post!(
        doc: "Invoke action restoreVersion",
        name: restore_version,
        path: "/versions/{{RID}}/restoreVersion"
    );
}
