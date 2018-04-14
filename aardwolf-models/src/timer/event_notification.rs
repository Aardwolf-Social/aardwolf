use chrono::DateTime;
use chrono::offset::Utc;

use timer::event::Event;
use schema::event_notifications;
use timer::Timer;

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "event_notifications"]
pub struct EventNotification {
    id: i32,
    event_id: i32, // foreign key to Event
    timer_id: i32, // foreign key to Timer
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl EventNotification {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn event_id(&self) -> i32 {
        self.event_id
    }

    pub fn timer_id(&self) -> i32 {
        self.timer_id
    }
}

#[derive(Insertable)]
#[table_name = "event_notifications"]
pub struct NewEventNotification {
    event_id: i32,
    timer_id: i32,
}

impl NewEventNotification {
    pub fn new(event: &Event, timer: &Timer) -> Self {
        NewEventNotification {
            event_id: event.id(),
            timer_id: timer.id(),
        }
    }
}
