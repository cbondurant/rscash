use druid::Data;
use serde::{Deserialize, Serialize};

use crate::{currency::Quantity, date::Date, guid::GUID};

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
	id: GUID,
	#[serde(rename = "reconciled-state")]
	state: SplitState,
	#[serde(rename = "account")]
	account_id: GUID,
	value: Quantity,
	#[serde(rename = "reconcile-date")]
	reconcile_date: Option<Date>,
}
