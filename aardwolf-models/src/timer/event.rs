use chrono::DateTime;
use chrono::offset::Utc;
use chrono_tz::Tz;

use base_actor::persona::Persona;
use schema::events;
use timer::Timer;

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "events"]
pub struct Event {
    id: i32,
    owner: i32,      // foreign key to Persona
    start_date: i32, // foreign key to Timer
    end_date: i32,   // foreign key to Timer
    timezone: Tz,
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
        self.timezone
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Insertable)]
#[table_name = "events"]
pub struct NewEvent {
    owner: i32,
    start_date: i32,
    end_date: i32,
    timezone: String,
    title: String,
    description: String,
}

impl NewEvent {
    pub fn new(
        owner: &Persona,
        start_date: &Timer,
        end_date: &Timer,
        timezone: Tz,
        title: String,
        description: String,
    ) -> Self {
        NewEvent {
            owner: owner.id(),
            start_date: start_date.id(),
            end_date: end_date.id(),
            timezone: timezone.name().to_owned(),
            title,
            description,
        }
    }
}
