#[macro_use]
extern crate log;
#[macro_use]
extern crate collection_macros;
#[macro_use]
extern crate failure;

use std::{error::Error, sync::Arc};

use actix::{self, Addr, SyncArbiter};
use actix_web::{
    http::{header::CONTENT_TYPE, Method},
    middleware::{
        session::{CookieSessionBackend, SessionStorage},
        Logger,
    },
    server::HttpServer,
    App, HttpResponse,
};
use config::Config;
use diesel::pg::PgConnection;
use handlebars::Handlebars;
use r2d2_diesel::ConnectionManager;

pub mod db;
pub mod error;
pub mod routes;
pub mod types;

use self::db::{Db, Pool};

#[derive(Clone)]
pub struct AppConfig {
    db: Addr<Db>,
    templates: Arc<Handlebars>,
}

impl AppConfig {
    fn render<T: serde::Serialize>(&self, template: &str, data: &T) -> error::RenderResult {
        self.templates
            .render(template, data)
            .map(|s| HttpResponse::Ok().header(CONTENT_TYPE, "text/html").body(s))
            .map_err(|e| {
                error!("Unable to render template, {}", e);
                error::RenderError
            })
    }
}

fn db_pool(database_url: String) -> Result<Pool, Box<dyn Error>> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Ok(r2d2::Pool::builder().build(manager)?)
}

pub fn run(config: Config, database_url: String) -> Result<(), Box<dyn Error>> {
    let sys = actix::System::new("aardwolf-actix");

    let pool = db_pool(database_url)?;

    let db = SyncArbiter::start(3, move || Db::new(pool.clone()));

    let listen_address = format!(
        "{}:{}",
        config.get_str("Web.Listen.address")?,
        config.get_str("Web.Listen.port")?
    );

    let template_dir = config.get_str("Templates.dir")?;

    let mut templates = Handlebars::new();
    templates.register_templates_directory(".html.hbs", &template_dir)?;

    let templates = Arc::new(templates);

    HttpServer::new(move || {
        let state = AppConfig {
            db: db.clone(),
            templates: templates.clone(),
        };

        vec![
            App::with_state(state.clone())
                .prefix("/auth")
                .middleware(Logger::default())
                .middleware(SessionStorage::new(
                    CookieSessionBackend::signed(&[0; 32]).secure(false),
                ))
                .resource("/sign_up", |r| {
                    r.method(Method::GET)
                        .with(self::routes::auth::sign_up_form_with_error);
                    r.method(Method::POST).with(self::routes::auth::sign_up)
                })
                .resource("/sign_in", |r| {
                    r.method(Method::GET)
                        .with(self::routes::auth::sign_in_form_with_error);
                    r.method(Method::POST).with(self::routes::auth::sign_in)
                })
                .resource("/confirmation", |r| {
                    r.method(Method::GET).with(self::routes::auth::confirm)
                })
                .resource("/sign_out", |r| {
                    r.method(Method::DELETE).with(self::routes::auth::sign_out)
                }),
            App::with_state(state.clone())
                .middleware(Logger::default())
                .middleware(SessionStorage::new(
                    CookieSessionBackend::signed(&[0; 32]).secure(false),
                ))
                .resource("/", |r| {
                    r.method(Method::GET).with(self::routes::app::index)
                }),
        ]
    })
    .bind(&listen_address)?
    .run();

    sys.run();

    Ok(())
}
