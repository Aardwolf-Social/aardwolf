use chrono::{offset::Utc, DateTime};
use chrono_tz::Tz;
use diesel::{self, pg::PgConnection};
use thiserror::Error;

use crate::{base_actor::persona::Persona, schema::events, sql_types::Timezone, timer::Timer};

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[diesel(table_name = events)]
pub struct Event {
    id: i32,
    owner: i32,      // foreign key to Persona
    start_date: i32, // foreign key to Timer
    end_date: i32,   // foreign key to Timer
    timezone: Timezone,
    title: String,
    description: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Event {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn owner(&self) -> i32 {
        self.owner
    }

    pub fn start_date(&self) -> i32 {
        self.start_date
    }

    pub fn end_date(&self) -> i32 {
        self.end_date
    }

    pub fn timezone(&self) -> Tz {
        self.timezone.0
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Insertable)]
#[diesel(table_name = events)]
pub struct NewEvent {
    owner: i32,
    start_date: i32,
    end_date: i32,
    timezone: Timezone,
    title: String,
    description: String,
}

impl NewEvent {
    pub fn insert(self, conn: &mut PgConnection) -> Result<Event, diesel::result::Error> {
        use diesel::prelude::*;

        diesel::insert_into(events::table)
            .values(&self)
            .get_result(conn)
    }

    pub fn new(
        owner: &Persona,
        start_date: &Timer,
        end_date: &Timer,
        timezone: Tz,
        title: String,
        description: String,
    ) -> Result<Self, EventCreationError> {
        if start_date.fire_time() > end_date.fire_time() {
            return Err(EventCreationError);
        }

        Ok(NewEvent {
            owner: owner.id(),
            start_date: start_date.id(),
            end_date: end_date.id(),
            timezone: timezone.into(),
            title,
            description,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("Start time must be before end time")]
pub struct EventCreationError;

#[cfg(test)]
mod tests {
    use chrono_tz::Tz;

    use super::NewEvent;
    use crate::test_helper::*;

    #[test]
    fn create_event() {
        with_connection(|conn| {
            let t1 = make_timer(conn)?;
            let t2 = make_timer(conn)?;
            let (start, end) = if t1.fire_time() < t2.fire_time() {
                (t1, t2)
            } else {
                (t2, t1)
            };

            let owner_base = make_base_actor(conn)?;
            let owner = make_persona(conn, &owner_base)?;
            let _ = make_event(conn, &owner, &start, &end)?;

            Ok(())
        })
    }

    #[test]
    fn dont_create_event_with_invalid_times() {
        with_connection(|conn| {
            let t1 = make_timer(conn)?;
            let t2 = make_timer(conn)?;
            let (start, end) = if t1.fire_time() < t2.fire_time() {
                (t1, t2)
            } else {
                (t2, t1)
            };

            let owner_base = make_base_actor(conn)?;
            let owner = make_persona(conn, &owner_base)?;

            let new_event =
                NewEvent::new(&owner, &end, &start, Tz::UTC, gen_string(), gen_string());

            assert!(
                new_event.is_err(),
                "Should not have created event with invalid start and end times"
            );

            Ok(())
        })
    }
}
