use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::db::{data::attendance::Attendance, crud::Crud};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Student {
	pub uuid: Uuid,
	pub name: String,
	pub record: Vec<Attendance>,
    pub authorization_level: u8,
}

impl Student {
	pub fn new(name: String) -> Self {
		Student {
			uuid: Uuid::new_v4(),
			name,
			record: Vec::new(),
            authorization_level: 0,
		}
	}
}

impl Crud<Student> for Student {}