use druid::Data;
use serde::{Deserialize, Serialize};

use crate::gnu_data::guid::GUID;

use super::transaction::Transaction;

#[derive(Data, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransactionFilter {
	Account(GUID),
}

impl TransactionFilter {
	pub fn transaction_filtered(&self, transaction: &Transaction) -> bool {
		match self {
			TransactionFilter::Account(guid) => {
				for split in transaction.splits.iter() {
					if split.account_id == *guid {
						return true;
					}
				}
				return false;
			}
		}
	}
}

#[derive(Data, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Page {
	Accounts { selected_page: GUID },
	Transactions(TransactionFilter),
}
