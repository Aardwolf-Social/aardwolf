use std::{error::Error, fmt};

use aardwolf_models::{base_actor::GenerateUrls, sql_types::Url};
use aardwolf_templates::Renderable;
use actix::{self, Addr, SyncArbiter};
use actix_web::{
    dev::HttpResponseBuilder,
    fs::StaticFiles,
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
use r2d2_diesel::ConnectionManager;
use rocket_i18n::{Internationalized, Translations};
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
}

#[derive(Clone)]
pub struct AppConfig {
    db: Addr<Db>,
    translations: Translations,
    generator: UrlGenerator,
}

impl Internationalized for AppConfig {
    fn get(&self) -> Translations {
        self.translations.clone()
    }
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
            // TODO: domain and languages should be config'd
            translations: rocket_i18n::i18n("aardwolf", vec!["en", "pl"]),
            generator: url_generator.clone(),
        };

        vec![
            App::with_state(state.clone())
                .prefix("/auth")
                .middleware(Logger::default())
                .middleware(SessionStorage::new(
                    CookieSessionBackend::signed(&[0; 32]).secure(false),
                ))
                .resource("/sign_up", |r| {
                    r.method(Method::GET).with(self::routes::auth::sign_up_form);
                    r.method(Method::POST).with(self::routes::auth::sign_up)
                })
                .resource("/sign_in", |r| {
                    r.method(Method::GET).with(self::routes::auth::sign_in_form);
                    r.method(Method::POST).with(self::routes::auth::sign_in)
                })
                .resource("/confirmation", |r| {
                    r.method(Method::GET).with(self::routes::auth::confirm)
                })
                .resource("/sign_out", |r| {
                    r.method(Method::GET).with(self::routes::auth::sign_out)
                }),
            App::with_state(state.clone())
                .prefix("/personas")
                .middleware(Logger::default())
                .middleware(SessionStorage::new(
                    CookieSessionBackend::signed(&[0; 32]).secure(false),
                ))
                .resource("/create", |r| {
                    r.method(Method::GET).with(self::routes::personas::new);
                    r.method(Method::POST).with(self::routes::personas::create)
                })
                .resource("/delete", |r| {
                    r.method(Method::GET).with(self::routes::personas::delete)
                }),
            #[cfg(not(debug_assertions))]
            App::with_state(state.clone())
                .middleware(Logger::default())
                .middleware(SessionStorage::new(
                    CookieSessionBackend::signed(&[0; 32]).secure(false),
                ))
                .resource("/", |r| {
                    r.method(Method::GET).with(self::routes::app::index)
                }),
            #[cfg(debug_assertions)]
            App::with_state(state.clone())
                .middleware(Logger::default())
                .middleware(SessionStorage::new(
                    CookieSessionBackend::signed(&[0; 32]).secure(false),
                ))
                .resource("/", |r| {
                    r.method(Method::GET).with(self::routes::app::index)
                })
                .handler("/web", StaticFiles::new(assets.web()).unwrap())
                .handler("/images", StaticFiles::new(assets.images()).unwrap())
                .handler("/themes", StaticFiles::new(assets.themes()).unwrap())
                .handler("/emoji", StaticFiles::new(assets.emoji()).unwrap())
                .handler(
                    "/stylesheets",
                    StaticFiles::new(assets.stylesheets()).unwrap(),
                ),
        ]
    })
    .bind(&listen_address)?
    .run();

    sys.run();

    Ok(())
}
