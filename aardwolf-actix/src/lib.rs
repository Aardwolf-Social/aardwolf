#[macro_use]
extern crate log;

use std::error::Error;

use actix::{Addr, SyncArbiter};
use actix_web::{App, HttpRequest, server::HttpServer};
use config::Config;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

pub mod db;

use self::db::{Db, Pool};

pub struct State {
    db: Addr<Db>,
}

fn db_pool(database_url: String) -> Result<Pool, Box<dyn Error>> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Ok(r2d2::Pool::builder().build(manager)?)
}

fn index(_req: &HttpRequest<State>) -> &'static str {
    "Hello world!"
}

pub fn run(config: Config, database_url: String) -> Result<(), Box<dyn Error>> {
    let sys = actix::System::new("aardwolf-actix");

    let pool = db_pool(database_url)?;

    let db = SyncArbiter::start(3, move || {
        Db::new(pool.clone())
    });

    let listen_address = format!("{}:{}", config.get_str("Web.Listen.address")?, config.get_str("Web.Listen.port")?);

    HttpServer::new(move || {
        let state = State {
            db: db.clone(),
        };

        App::with_state(state)
            .resource("/", |r| r.f(index))
    }).bind(&listen_address)?.run();

    info!("listening on {}", listen_address);

    sys.run();

    Ok(())
}
