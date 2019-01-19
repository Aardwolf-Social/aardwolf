use aardwolf_models::sql_types::{Mime, PostVisibility};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidatedPostCreationForm {
    pub(crate) media_type: Mime,
    pub(crate) visibility: PostVisibility,
    pub(crate) content: String,
    pub(crate) source: String,
}
