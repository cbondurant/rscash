use druid::Data;
use serde::{Deserialize, Serialize};

use super::{currency::Quantity, date::Date, guid::GUID};

/// The current state of the split
/// if reconsiled, includes the time that was applied.
#[derive(Debug, Deserialize, Serialize, Data, Clone, PartialEq, Eq)]
pub enum SplitState {
	#[serde(rename = "y")]
	Reconsiled,
	#[serde(rename = "c")]
	Cleared,
	#[serde(rename = "n")]
	Pending,
}

#[derive(Debug, Deserialize, Serialize, Data, Clone)]
pub struct Split {
	pub id: GUID,
	#[serde(rename = "reconciled-state")]
	pub state: SplitState,
	#[serde(rename = "account")]
	pub account_id: GUID,
	pub value: Quantity,
	#[serde(rename = "reconcile-date")]
	pub reconcile_date: Option<Date>,
}
