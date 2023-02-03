use druid::im::Vector;
use druid::Data;
use serde::{Deserialize, Serialize};

use super::{commodity::Commodity, date::Date, guid::GUID, split::Split};

#[derive(Debug, Deserialize, Serialize, Data, Clone)]
pub struct Splits {
	pub split: Vector<Split>,
}

#[derive(Debug, Deserialize, Serialize, Data, Clone)]
pub struct Transaction {
	pub id: GUID,
	pub currency: Commodity,
	#[serde(rename = "date-posted")]
	pub date_posted: Date,
	#[serde(rename = "date-entered")]
	pub date_entered: Date,
	pub description: String,
	pub splits: Splits,
}
