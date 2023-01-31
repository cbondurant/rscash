use druid::{im::Vector, Data, Lens};
use serde::{Deserialize, Serialize};

use crate::gnu_data::{
	account::{Account as GnuAccount, AccountType as GnuAccountType},
	guid::GUID,
};

#[derive(Debug, Deserialize, Serialize, Data, Clone, PartialEq, Eq)]
pub enum AccountType {
	Expense,
	Root,
	Liability,
	Asset,
	Credit,
	Bank,
	Cash,
	Mutual,
	Income,
	Equity,
}

#[derive(Data, Debug, Serialize, Deserialize, Clone, Lens)]
pub struct Account {
	pub name: String,
	pub id: GUID,
	pub category: AccountType,
	description: Option<String>,
	pub children: Vector<Account>,
	pub children_hidden: bool,
}

impl Account {
	pub fn get_child(&self, id: GUID) -> Option<&Account> {
		self.children.iter().find(|account| account.id == id)
	}
}

impl From<GnuAccount> for Account {
	fn from(value: GnuAccount) -> Self {
		Self {
			name: value.name,
			id: value.id,
			category: value.kind.into(),
			description: value.description,
			children: Vector::new(),
			children_hidden: false,
		}
	}
}

impl From<GnuAccountType> for AccountType {
	fn from(value: GnuAccountType) -> Self {
		match value {
			GnuAccountType::Expense => AccountType::Expense,
			GnuAccountType::Root => AccountType::Root,
			GnuAccountType::Liability => AccountType::Liability,
			GnuAccountType::Asset => AccountType::Asset,
			GnuAccountType::Credit => AccountType::Credit,
			GnuAccountType::Bank => AccountType::Bank,
			GnuAccountType::Cash => AccountType::Cash,
			GnuAccountType::Mutual => AccountType::Mutual,
			GnuAccountType::Income => AccountType::Income,
			GnuAccountType::Equity => AccountType::Equity,
		}
	}
}
