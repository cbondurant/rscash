use druid::Data;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Data, Clone, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum CommodityType {
	Currency,
	Nasdaq,
	#[serde(rename = "template")]
	Template,
}

#[derive(Debug, Deserialize, Serialize, Data, Clone)]
pub struct Commodity {
	space: CommodityType,
}
