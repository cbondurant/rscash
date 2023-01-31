use druid::Data;
use serde::{Deserialize, Serialize};

use crate::gnu_data::guid::GUID;

#[derive(Data, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransactionFilter {
	Account(GUID),
}

#[derive(Data, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Page {
	Accounts { selected_page: GUID },
	Transactions { filter: TransactionFilter },
}
