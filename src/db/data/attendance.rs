use chrono::{DateTime, Utc};
use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Attendance {
	pub event_uuid: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}