use druid::{im::Vector, Data};
use serde::{Deserialize, Serialize};

use super::{
	commodity::Commodity,
	currency::{Currency, Quantity},
	date::Date,
	guid::GUID,
};

#[derive(Debug, Deserialize, Serialize, Data, Clone, PartialEq, Eq)]
pub enum PriceSource {
	#[serde(rename = "Finance::Quote")]
	FinanceQuote,
	UserTransaction,
	#[serde(rename = "user:price-editor")]
	UserPriceEditor,
	#[serde(rename = "user:split-register")]
	UserSplitRegister,
	#[serde(rename = "user:price")]
	UserPrice,
}

#[derive(Debug, Deserialize, Serialize, Data, Clone)]
pub struct Price {
	id: GUID,
	commodity: Commodity,
	currency: Currency,
	time: Date,
	source: PriceSource,
	value: Quantity,
}

#[derive(Debug, Deserialize, Serialize, Data, Clone)]
pub struct PriceDB {
	#[serde(rename = "price")]
	prices: Vector<Price>,
}
