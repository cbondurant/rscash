use std::cell::RefCell;
use std::sync::Arc;

use druid::Data;
use serde::{Deserialize, Serialize};

use super::account::Account;

#[derive(Debug, Data, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ReconsileState {
	Reconsiled,
	Cleared,
	Pending,
}

#[derive(Debug, Data, Serialize, Deserialize, Clone)]
pub struct Split {
	state: ReconsileState,
	account: Arc<RefCell<Account>>,
	//value: Quantity,
}
