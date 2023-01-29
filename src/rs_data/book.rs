use std::cell::RefCell;

use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

use super::account::Account;
use crate::gnu_data::{book::Book as GnuBook, guid::GUID};

#[derive(Debug, Deserialize, Serialize, Clone, Data, Lens)]
pub struct Book {
	pub root_account: Account,
}

impl From<GnuBook> for Book {
	fn from(value: GnuBook) -> Self {
		let mut account_list: Vec<(GUID, Vec<GUID>, Account)> = Vec::new();
		for account in value.accounts {
			let mut address_chain = Vec::new();
			if account.parent_id.is_some() {
				address_chain.push(account.parent_id.unwrap());
			}
			account_list.push((account.id, address_chain, account.into()));
		}

		println!("{:#?}", account_list);

		fn get_path(
			account_list: &Vec<(GUID, Vec<GUID>, Account)>,
			current_account: GUID,
		) -> Vec<GUID> {
			let mut parents = account_list
				.iter()
				.find(|item| item.0 == current_account)
				.unwrap()
				.1
				.clone();
			if parents.len() == 1 {
				parents.append(&mut get_path(account_list, parents[0]));
			}
			parents
		}

		let mut path_account_list = Vec::new();

		for account in account_list.iter() {
			path_account_list.push((
				account.0,
				get_path(&account_list, account.0),
				account.2.clone(),
			));
		}

		path_account_list.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

		println!("{:#?}", path_account_list);

		// Iterate until we get to the root node
		while path_account_list.len() != 1 {
			let child = path_account_list.pop().unwrap();
			let parent_id = child.1[0];
			path_account_list
				.iter_mut()
				.rev()
				.find(|val| val.0 == parent_id)
				.unwrap()
				.2
				.children
				.push_back(child.2);
		}

		Book {
			root_account: path_account_list.pop().unwrap().2,
		}
	}
}
