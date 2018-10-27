use crate::scope::Scope;

#[derive(Debug, Clone, PartialEq)]
pub struct App<'a> {
    pub client_name: &'a str,
    pub redirect_uris: &'a str,
    pub scopes: &'a Scope,
    pub website: &'a Option<String>,
}

#[derive(Default, Builder, Debug, Clone, PartialEq, Serialize)]
#[builder(setter(into))]
pub struct AppId {
    id: String,
    client_id: String,
    client_secret: String,
}
