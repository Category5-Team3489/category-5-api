pub mod student;
pub mod attendance;
pub mod event;

use serde::{Deserialize, Serialize};

use self::{student::Student, event::Event};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbData {
	pub students: Vec<Student>,
    pub events: Vec<Event>,
}

impl DbData {
	pub fn new() -> Self {
        Self {
            students: Vec::new(),
            events: Vec::new(),
        }
    }
}