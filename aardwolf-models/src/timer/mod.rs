use chrono::DateTime;
use chrono::offset::Utc;

pub mod event;
pub mod event_notification;

use schema::timers;

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "timers"]
pub struct Timer {
    id: i32,
    fire_time: DateTime<Utc>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Timer {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn fire_time(&self) -> DateTime<Utc> {
        self.fire_time
    }
}

#[derive(Insertable)]
#[table_name = "timers"]
pub struct NewTimer {
    fire_time: DateTime<Utc>,
}

impl NewTimer {
    pub fn new(fire_time: DateTime<Utc>) -> Self {
        NewTimer { fire_time }
    }
}
