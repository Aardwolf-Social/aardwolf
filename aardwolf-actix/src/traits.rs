use aardwolf_templates::Renderable;
use actix_web::{dev::HttpResponseBuilder, http::header::CONTENT_TYPE, HttpResponse};

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

impl<T> RenderableExt for T where T: Renderable {}

pub trait WithRucte {
    fn ructe<R>(&mut self, r: R) -> HttpResponse
    where
        R: Renderable;
}

impl WithRucte for HttpResponseBuilder {
    fn ructe<R>(&mut self, r: R) -> HttpResponse
    where
        R: Renderable,
    {
        let mut buf = Vec::new();

        match r.render(&mut buf) {
            Ok(_) => self.append_header((CONTENT_TYPE, "text/html")).body(buf),
            Err(e) => self
                .append_header((CONTENT_TYPE, "text/plain"))
                .body(format!("{}", e)),
        }
    }
}
