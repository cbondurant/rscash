use druid::{
	im::{HashMap, Vector},
	Data, Lens,
};
use serde::{Deserialize, Serialize};

use super::{account::Account, page::Page, transaction::Transaction};
use crate::gnu_data::{book::Book as GnuBook, guid::GUID};

#[derive(Debug, Deserialize, Serialize, Clone, Data, Lens)]
pub struct Book {
	pub id: GUID,
	pub root_account: Account,
	pub account_paths: HashMap<GUID, Vector<GUID>>,
	pub transactions: Vector<Transaction>,
	pub pages: Vector<Page>,
}

impl Book {
	pub fn get_account_name_path(&self, account: &GUID) -> String {
		let mut path = String::new();
		let mut node = &self.root_account;

		let mut account_path = self.account_paths.get(account).unwrap().clone();
		account_path.push_back(*account);
		account_path.pop_front();

		for node_id in account_path {
			node = node.get_child(node_id).unwrap();
			path.push_str(node.name.as_str());
			path.push(':')
		}

		path.pop();
		path
	}
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

		let mut paths = HashMap::new();

		// Iterate until we get to the root node
		while path_account_list.len() != 1 {
			let mut child = path_account_list.pop().unwrap();
			let parent_id = child.1[0];
			child.1.reverse();
			paths.entry(child.0).or_insert(child.1.into());
			path_account_list
				.iter_mut()
				.rev()
				.find(|val| val.0 == parent_id)
				.unwrap()
				.2
				.children
				.push_back(child.2);
		}

		let mut transactions = Vector::new();
		for transaction in value.transactions.iter() {
			transactions.push_back(transaction.clone().into());
		}

		Book {
			id: value.id,
			root_account: path_account_list.pop().unwrap().2,
			account_paths: paths,
			pages: Vector::new(),
			transactions,
		}
	}
}
