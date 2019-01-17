use std::str::FromStr;

use rocket::http::Cookies;

pub fn from_cookie<I, E>(cookies: &mut Cookies, key: &str, err: E) -> Result<I, E>
where
    I: FromStr,
{
    match cookies.get_private(key) {
        Some(cookie) => Ok(cookie.value().parse::<I>().map_err(|_| err)?),
        None => Err(err),
    }
}
