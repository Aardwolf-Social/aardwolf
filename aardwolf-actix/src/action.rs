use aardwolf_templates::Renderable;
use actix_web::{http::header::LOCATION, HttpResponse};

use crate::WithRucte;

pub trait RenderableExt: Renderable + Sized {
    fn ok(self) -> HttpResponse {
        HttpResponse::Ok().ructe(self)
    }

    fn created(self) -> HttpResponse {
        HttpResponse::Created().ructe(self)
    }

    fn not_found(self) -> HttpResponse {
        HttpResponse::NotFound().ructe(self)
    }
}

impl<T> RenderableExt for T where T: Renderable + Sized {}

pub fn redirect(to: &str) -> HttpResponse {
    HttpResponse::SeeOther().header(LOCATION, to).finish()
}
