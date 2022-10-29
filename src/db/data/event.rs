use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::crud::Crud;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    pub uuid: Uuid,
    pub name: String,
    pub kind: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>
}

impl Event {
	pub fn new(name: String, kind: String, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Self {
		Event {
			uuid: Uuid::new_v4(),
			name,
            kind,
            start_time,
            end_time
		}
	}
}

impl Crud<Event> for Event {}