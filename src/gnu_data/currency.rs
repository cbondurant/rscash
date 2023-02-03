use druid::Data;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Data, Clone)]
pub struct Currency {
	space: String,
	id: String,
}

#[derive(Debug, Deserialize, Serialize, Data, Clone)]
pub struct Quantity(pub String);
