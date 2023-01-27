use druid::Data;
use serde::{Deserialize, Serialize};

use super::{commodity::Commodity, guid::GUID};

#[derive(Debug, Deserialize, Serialize, Data, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum AccountType {
	Expense,
	Root,
	Liability,
	Asset,
	Credit,
	Bank,
	Cash,
	Mutual,
	Income,
	Equity,
}

#[derive(Debug, Deserialize, Serialize, Data, Clone)]
pub struct Account {
	pub name: String,
	id: GUID,
	#[serde(rename = "type")]
	kind: AccountType,
	commodity: Commodity,
	parent_id: Option<GUID>,
	description: Option<String>,
}
