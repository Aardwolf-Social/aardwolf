use crate::scope::Scope;

#[derive(Debug, Clone, PartialEq)]
pub struct App {
    pub client_name: String,
    pub redirect_uris: String,
    pub scopes: Scope,
    pub website: Option<String>,
}

#[derive(Default, Builder, Debug, Clone, PartialEq, Serialize)]
#[builder(setter(into))]
pub struct AppId {
    id: String,
    client_id: String,
    client_secret: String,
}
