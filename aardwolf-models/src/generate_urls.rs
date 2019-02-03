use uuid::Uuid;

use crate::{base_actor::BaseActor, sql_types::Url};

pub trait GenerateUrls {
    fn activitypub_id(&self, uuid: &Uuid) -> String;
    fn profile_url(&self, uuid: &Uuid) -> Url;
    fn inbox_url(&self, uuid: &Uuid) -> Url;
    fn outbox_url(&self, uuid: &Uuid) -> Url;

    fn post_id(&self, base_actor: &BaseActor, uuid: &Uuid) -> String;
    fn post_url(&self, base_actor: &BaseActor, uuid: &Uuid) -> Url;
}
