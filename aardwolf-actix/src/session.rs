use actix_session::Session;

pub fn from_session<I, E>(session: &Session, key: &str, err: E) -> Result<I, E>
where
    I: serde::de::DeserializeOwned,
{
    match session.get::<I>(key) {
        Ok(maybe_item) => match maybe_item {
            Some(item) => Ok(item),
            None => Err(err),
        },
        Err(_) => Err(err),
    }
}
