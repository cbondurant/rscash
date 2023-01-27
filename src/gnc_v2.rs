use crate::book::Book;
use serde::{Deserialize, Serialize};

use druid::im::Vector;
use druid::Data;

#[derive(Debug, Data, Deserialize, Serialize, Clone)]
pub struct GCNv2 {
	#[serde(rename = "book")]
	pub books: Vector<Book>,
}
