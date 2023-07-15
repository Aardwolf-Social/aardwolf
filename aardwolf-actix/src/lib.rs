use std::{error::Error, fmt, thread};

use aardwolf_models::{base_actor::BaseActor, generate_urls::GenerateUrls, sql_types::Url};
use actix::System;
use actix_files::Files;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    middleware::Logger,
    web::{get, post, resource, scope, Data},
    App, HttpServer,
    rt::Runtime
};

use config::Config;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use uuid::Uuid;

mod action;
mod error;
mod routes;
mod session;
mod traits;
mod types;

pub use crate::session::from_session;

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
    generator: UrlGenerator,
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl fmt::Debug for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AppConfig")
    }
}

fn db_pool(database_url: &str) -> Result<Pool<ConnectionManager<PgConnection>>, Box<dyn Error>> {
    let manager = ConnectionManager::new(database_url);
    let pool = Pool::builder().build(manager)?;
    Ok(pool)
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
                web: config.get_string("Assets.web")?,
                images: config.get_string("Assets.images")?,
                emoji: config.get_string("Assets.emoji")?,
                themes: config.get_string("Assets.themes")?,
                stylesheets: config.get_string("Assets.stylesheets")?,
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
    let pool = db_pool(database_url)?;

    let listen_address = format!(
        "{}:{}",
        config.get_string("Web.address")?,
        config.get_string("Web.port")?
    );

    let url_generator = UrlGenerator {
        domain: config.get_string("Instance.domain")?,
        https: config.get_bool("Instance.https")?,
    };

    // TODO: Allow key to be loaded from config file
    let secret_key = Key::generate();

    #[cfg(debug_assertions)]
    let assets = assets::Assets::from_config(&config)?;

    let sys = System::new();

    thread::spawn(move || {
        let _ = &Runtime::new().unwrap().block_on(async {
            HttpServer::new(move || {
                let state = AppConfig {
                    generator: url_generator.clone(),
                    pool: pool.clone(),
                };

                let translations = aardwolf_templates::managed_state();

                App::new()
                    .app_data(Data::new(state))
                    .app_data(Data::new(translations))
                    .wrap(Logger::default())
                    .wrap(SessionMiddleware::new(
                        CookieSessionStore::default(),
                        secret_key.clone(),
                    ))
                    .service(
                        scope("/auth")
                            .service(
                                resource("/sign_up")
                                    .route(get().to(self::routes::auth::sign_up_form))
                                    .route(post().to(self::routes::auth::sign_up)),
                            )
                            .service(
                                resource("/sign_in")
                                    .route(get().to(self::routes::auth::sign_in_form))
                                    .route(post().to(self::routes::auth::sign_in)),
                            )
                            .service(resource("/confirmation").route(get().to(self::routes::auth::confirm)))
                            .service(resource("/sign_out").route(get().to(self::routes::auth::sign_out))),
                    )
                    .service(
                        scope("/posts")
                            .service(resource("/create").route(post().to(self::routes::posts::create))),
                    )
                    .service(
                        scope("/personas")
                            .service(
                                resource("/create")
                                    .route(get().to(self::routes::personas::new))
                                    .route(post().to(self::routes::personas::create)),
                            )
                            .service(resource("/delete").route(get().to(self::routes::personas::delete))),
                    )
                    .service(resource("/").route(get().to(self::routes::app::index)))
                    .service(Files::new("/web", assets.web()))
                    .service(Files::new("/images", assets.images()))
                    .service(Files::new("/themes", assets.themes()))
                    .service(Files::new("/emoji", assets.emoji()))
                    .service(Files::new("/stylesheets", assets.stylesheets()))
            })
            .bind(&listen_address)?
            .run()
            .await
        });
    });

    Ok(sys.run()?)
}

