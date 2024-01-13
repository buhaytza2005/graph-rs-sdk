// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    TermStoreSetsTermsApiClient,
    TermStoreSetsTermsIdApiClient,
    ResourceIdentity::TermStoreSetsTerms
);

impl TermStoreSetsTermsApiClient {
    post!(
        doc: "Create new navigation property to terms for sites",
        name: create_terms,
        path: "/terms",
        body: true
    );
    get!(
        doc: "Get term",
        name: list_terms,
        path: "/terms"
    );
    get!(
        doc: "Get the number of the resource",
        name: terms,
        path: "/terms/$count"
    );
}

impl TermStoreSetsTermsIdApiClient {
    delete!(
        doc: "Delete term",
        name: delete_terms,
        path: "/terms/{{RID}}"
    );
    get!(
        doc: "Get term",
        name: get_terms,
        path: "/terms/{{RID}}"
    );
    patch!(
        doc: "Update term",
        name: update_terms,
        path: "/terms/{{RID}}",
        body: true
    );
    get!(
        doc: "Get set from sites",
        name: get_set,
        path: "/terms/{{RID}}/set"
    );
}
