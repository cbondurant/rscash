use druid::Data;
use serde::{Deserialize, Serialize};

use crate::guid::GUID;

#[derive(Debug, Deserialize, Serialize, Data, Clone)]
pub struct Budget {
	id: GUID,
	name: String,
	description: String,
	#[serde(rename = "num-periods")]
	periods: u32,
	//#[serde(rename = "slots")]
	//accounts: HashMap<GUID, Vec<Quantity>>,
}
