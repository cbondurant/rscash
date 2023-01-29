use druid::{Data, Lens};
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

#[derive(Debug, Deserialize, Serialize, Data, Clone, Lens)]
pub struct Account {
	pub name: String,
	pub id: GUID,
	#[serde(rename = "type")]
	pub kind: AccountType,
	commodity: Commodity,
	#[serde(rename = "parent")]
	pub parent_id: Option<GUID>,
	pub description: Option<String>,
}
