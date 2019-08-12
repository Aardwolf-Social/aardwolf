#![feature(async_await)]

use std::{error::Error, fmt};

use aardwolf_models::{base_actor::BaseActor, generate_urls::GenerateUrls, sql_types::Url};
use aardwolf_templates::Renderable;
use actix::{self, Addr, SyncArbiter};
use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{
    dev::HttpResponseBuilder,
    http::header::CONTENT_TYPE,
    middleware::Logger,
    web::{get, post, resource, scope},
    App, HttpResponse, HttpServer,
};
use actix_web_async_compat::async_compat;
use config::Config;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use uuid::Uuid;

#[macro_use]
pub mod action;
pub mod db;
pub mod error;
pub mod routes;
mod session;
pub mod types;

pub use crate::session::from_session;

use self::db::{Db, Pool};

#[derive(Clone)]
pub struct UrlGenerator {
    domain: String,
    https: bool,
}

impl GenerateUrls for UrlGenerator {
    fn activitypub_id(&self, uuid: &Uuid) -> String {
        format!(
            "{}://{}/users/{}",
            if self.https { "https" } else { "http" },
            self.domain,
            uuid
        )
    }

    fn profile_url(&self, uuid: &Uuid) -> Url {
        format!(
            "{}://{}/users/{}/profile",
            if self.https { "https" } else { "http" },
            self.domain,
            uuid
        )
        .parse()
        .unwrap()
    }

    fn inbox_url(&self, uuid: &Uuid) -> Url {
        format!(
            "{}://{}/users/{}/inbox",
            if self.https { "https" } else { "http" },
            self.domain,
            uuid
        )
        .parse()
        .unwrap()
    }

    fn outbox_url(&self, uuid: &Uuid) -> Url {
        format!(
            "{}://{}/users/{}/outbox",
            if self.https { "https" } else { "http" },
            self.domain,
            uuid
        )
        .parse()
        .unwrap()
    }

    fn post_id(&self, _: &BaseActor, uuid: &Uuid) -> String {
        format!(
            "{}://{}/posts/{}",
            if self.https { "https" } else { "http" },
            self.domain,
            uuid
        )
    }

    fn post_url(&self, base_actor: &BaseActor, uuid: &Uuid) -> Url {
        self.post_id(base_actor, uuid).parse().unwrap()
    }
}

#[derive(Clone)]
pub struct AppConfig {
    db: Addr<Db>,
    generator: UrlGenerator,
}

impl fmt::Debug for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AppConfig")
    }
}

pub trait WithRucte {
    fn with_ructe<R>(&mut self, r: R) -> HttpResponse
    where
        R: Renderable;
}

impl WithRucte for HttpResponseBuilder {
    fn with_ructe<R>(&mut self, r: R) -> HttpResponse
    where
        R: Renderable,
    {
        let mut buf = Vec::new();

        match r.render(&mut buf) {
            Ok(_) => self.header(CONTENT_TYPE, "text/html").body(buf),
            Err(e) => self
                .header(CONTENT_TYPE, "text/plain")
                .body(format!("{}", e)),
        }
    }
}

fn db_pool(database_url: &str) -> Result<Pool, Box<dyn Error>> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Ok(r2d2::Pool::builder().build(manager)?)
}

#[cfg(debug_assertions)]
mod assets {
    use std::error::Error;

    use config::Config;

    #[derive(Clone, Debug)]
    pub struct Assets {
        web: String,
        images: String,
        emoji: String,
        themes: String,
        stylesheets: String,
    }

    impl Assets {
        pub fn from_config(config: &Config) -> Result<Self, Box<dyn Error>> {
            Ok(Assets {
                web: config.get_str("Assets.web")?,
                images: config.get_str("Assets.images")?,
                emoji: config.get_str("Assets.emoji")?,
                themes: config.get_str("Assets.themes")?,
                stylesheets: config.get_str("Assets.stylesheets")?,
            })
        }

        pub fn web(&self) -> &str {
            &self.web
        }

        pub fn images(&self) -> &str {
            &self.images
        }

        pub fn emoji(&self) -> &str {
            &self.emoji
        }

        pub fn themes(&self) -> &str {
            &self.themes
        }

        pub fn stylesheets(&self) -> &str {
            &self.stylesheets
        }
    }
}

pub fn run(config: &Config, database_url: &str) -> Result<(), Box<dyn Error>> {
    let sys = actix::System::new("aardwolf-actix");

    let pool = db_pool(database_url)?;

    let db = SyncArbiter::start(3, move || Db::new(pool.clone()));

    let listen_address = format!(
        "{}:{}",
        config.get_str("Web.Listen.address")?,
        config.get_str("Web.Listen.port")?
    );

    let url_generator = UrlGenerator {
        domain: config.get_str("Instance.domain")?,
        https: config.get_bool("Instance.https")?,
    };

    #[cfg(debug_assertions)]
    let assets = assets::Assets::from_config(&config)?;

    HttpServer::new(move || {
        let state = AppConfig {
            db: db.clone(),
            generator: url_generator.clone(),
        };

        let translations = aardwolf_templates::managed_state();

        App::new()
            .data(state.clone())
            .data(translations)
            .wrap(Logger::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(
                scope("/auth")
                    .service(
                        resource("/sign_up")
                            .route(get().to_async(self::routes::auth::sign_up_form))
                            .route(post().to_async(async_compat(self::routes::auth::sign_up))),
                    )
                    .service(
                        resource("/sign_in")
                            .route(get().to_async(self::routes::auth::sign_in_form))
                            .route(post().to_async(async_compat(self::routes::auth::sign_in))),
                    )
                    .service(
                        resource("/confirmation")
                            .route(get().to_async(async_compat(self::routes::auth::confirm))),
                    )
                    .service(
                        resource("/sign_out").route(get().to_async(self::routes::auth::sign_out)),
                    ),
            )
            .service(
                scope("/posts").service(
                    resource("/create")
                        .route(post().to_async(async_compat(self::routes::posts::create))),
                ),
            )
            .service(
                scope("/personas")
                    .service(
                        resource("/create")
                            .route(get().to_async(self::routes::personas::new))
                            .route(post().to_async(async_compat(self::routes::personas::create))),
                    )
                    .service(
                        resource("/delete")
                            .route(get().to_async(async_compat(self::routes::personas::delete))),
                    ),
            )
            .service(resource("/").route(get().to_async(self::routes::app::index)))
            .service(Files::new("/web", assets.web()))
            .service(Files::new("/images", assets.images()))
            .service(Files::new("/themes", assets.themes()))
            .service(Files::new("/emoji", assets.emoji()))
            .service(Files::new("/stylesheets", assets.stylesheets()))
    })
    .bind(&listen_address)?
    .start();

    sys.run()?;

    Ok(())
}
