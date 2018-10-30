use failure::Fail;

pub enum AardwolfErrorKind {
    Redirect(String),
    BadRequest,
    RequiresAuthentication,
    RequiresPermission,
    NotFound,
    InternalServerError,
}

pub trait AardwolfError: Fail {
    fn name(&self) -> &str;
    fn kind(&self) -> AardwolfErrorKind;
    fn description(&self) -> String;
}

#[derive(Serialize, Deserialize)]
pub struct ErrorJson {
    name: String,
    description: String,
}

#[derive(Debug, Fail)]
#[fail(display = "{}", _0)]
pub struct AardwolfErrorWrapper<E>(E)
where
    E: AardwolfError;

impl<E> From<E> for AardwolfErrorWrapper<E>
where
    E: AardwolfError,
{
    fn from(e: E) -> Self {
        AardwolfErrorWrapper(e)
    }
}

#[cfg(feature = "rocket")]
mod rocket {
    use rocket::{
        http::{hyper::header::Location, Status},
        response::{Responder, Result},
        Request, Response,
    };
    use rocket_contrib::Json;

    use super::{AardwolfError, AardwolfErrorKind, AardwolfErrorWrapper, ErrorJson};

    impl<'r, E> Responder<'r> for AardwolfErrorWrapper<E>
    where
        E: AardwolfError,
    {
        fn respond_to(self, req: &Request) -> Result<'r> {
            let mut response_builder = Response::build();

            match self.0.kind() {
                AardwolfErrorKind::Redirect(s) => {
                    response_builder
                        .status(Status::SeeOther)
                        .header(Location(s));
                }
                AardwolfErrorKind::BadRequest => {
                    response_builder.status(Status::BadRequest);
                }
                AardwolfErrorKind::RequiresAuthentication => {
                    response_builder.status(Status::Unauthorized);
                }
                AardwolfErrorKind::RequiresPermission => {
                    response_builder.status(Status::Forbidden);
                }
                AardwolfErrorKind::NotFound => {
                    response_builder.status(Status::NotFound);
                }
                AardwolfErrorKind::InternalServerError => {
                    response_builder.status(Status::InternalServerError);
                }
            }

            let json = Json(ErrorJson {
                name: self.0.name().to_owned(),
                description: self.0.description(),
            })
            .respond_to(req)?;

            Ok(response_builder.join(json).finalize())
        }
    }
}

#[cfg(feature = "actix")]
mod actix {
    use actix_web::{error::ResponseError, http::header::LOCATION, HttpResponse};

    use super::{AardwolfError, AardwolfErrorKind, AardwolfErrorWrapper, ErrorJson};

    impl<E> ResponseError for AardwolfErrorWrapper<E>
    where
        E: AardwolfError,
    {
        fn error_response(&self) -> HttpResponse {
            let mut response_builder = match self.0.kind() {
                AardwolfErrorKind::Redirect(s) => {
                    let mut res = HttpResponse::SeeOther();
                    res.header(LOCATION, s);
                    res
                }
                AardwolfErrorKind::BadRequest => HttpResponse::BadRequest(),
                AardwolfErrorKind::RequiresAuthentication => HttpResponse::Unauthorized(),
                AardwolfErrorKind::RequiresPermission => HttpResponse::Forbidden(),
                AardwolfErrorKind::NotFound => HttpResponse::NotFound(),
                AardwolfErrorKind::InternalServerError => HttpResponse::InternalServerError(),
            };

            response_builder.json(ErrorJson {
                name: self.0.name().to_owned(),
                description: self.0.description(),
            })
        }
    }
}
