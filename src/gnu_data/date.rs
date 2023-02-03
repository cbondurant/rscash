use chrono::{DateTime, Local};
use druid::Data;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Date {
	pub date: DateTime<Local>,
}

impl Data for Date {
	fn same(&self, other: &Self) -> bool {
		self.date == other.date
	}
}
