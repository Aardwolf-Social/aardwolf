use std::str::FromStr;

use rocket::http::Cookies;

pub fn from_cookie<I, E>(cookies: &mut Cookies, key: &str, err: E) -> Result<I, E>
where
    I: FromStr,
{
    println!("Cookies: {:?}", cookies);
    match cookies.get_private(key) {
        Some(cookie) => {
            let value = cookie.value();
            println!("Value: {}", value);
            Ok(value.parse::<I>().map_err(|_| err)?)
        },
        None => Err(err),
    }
}
