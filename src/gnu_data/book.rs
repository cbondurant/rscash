use super::{
	account::Account, budget::Budget, commodity::Commodity, guid::GUID, price::PriceDB,
	transaction::Transaction,
};
use druid::{im::Vector, Data, Lens};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Data, Lens)]
pub struct Book {
	pub id: GUID,
	#[serde(rename = "commodity")]
	pub commodities: Vector<Commodity>,
	pub pricedb: PriceDB,
	#[serde(rename = "account")]
	pub accounts: Vector<Account>,
	#[serde(rename = "transaction")]
	pub transactions: Vector<Transaction>,
	#[serde(rename = "budget")]
	pub budgets: Vector<Budget>,
}
