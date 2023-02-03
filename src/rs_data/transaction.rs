use chrono::{DateTime, Local};
use druid::{im::Vector, Data};
use serde::{Deserialize, Serialize};

use crate::gnu_data::{guid::GUID, transaction::Transaction as GnuTransaction};

#[derive(Data, Debug, Deserialize, Serialize, Clone)]
pub struct Transaction {
	id: GUID,
	date_posted: Option<DateTime<Local>>,
	date_entered: DateTime<Local>,
	description: String,
}

impl From<GnuTransaction> for Transaction {
	fn from(value: GnuTransaction) -> Self {
		Self {
			id: value.id,
			date_posted: Some(value.date_posted.date),
			date_entered: value.date_entered.date,
			description: value.description,
		}
	}
}
