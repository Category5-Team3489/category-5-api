use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::crud::Crud;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    pub uuid: Uuid,
    pub name: String,
    pub info: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime
}

impl Event {
	pub fn new(name: String, info: String, start_time: NaiveDateTime, end_time: NaiveDateTime) -> Self {
		Event {
			uuid: Uuid::new_v4(),
			name,
            info,
            start_time,
            end_time
		}
	}
}

impl Crud<Event> for Event {}