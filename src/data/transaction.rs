use druid::im::Vector;
use druid::Data;
use serde::{Deserialize, Serialize};

use super::{commodity::Commodity, date::Date, guid::GUID, split::Split};

#[derive(Debug, Deserialize, Serialize, Data, Clone)]
pub struct Splits {
	split: Vector<Split>,
}

#[derive(Debug, Deserialize, Serialize, Data, Clone)]
pub struct Transaction {
	id: GUID,
	currency: Commodity,
	#[serde(rename = "date-posted")]
	date_posted: Date,
	#[serde(rename = "date-entered")]
	date_entered: Date,
	description: String,
	splits: Splits,
}
