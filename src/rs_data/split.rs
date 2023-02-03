use chrono::{DateTime, Local};
use druid::Data;
use serde::{Deserialize, Serialize};

use crate::gnu_data::{
	guid::GUID,
	split::{Split as GnuSplit, SplitState as GnuSplitState},
};

#[derive(Debug, Deserialize, Serialize, Data, Clone, PartialEq, Eq)]
pub enum SplitState {
	Reconsiled,
	Cleared,
	Pending,
}

#[derive(Debug, Deserialize, Serialize, Data, Clone)]
pub struct Split {
	id: GUID,
	state: SplitState,
	pub account_id: GUID,
	value: String,
	reconcile_date: Option<DateTime<Local>>,
}

impl From<GnuSplit> for Split {
	fn from(value: GnuSplit) -> Self {
		Self {
			id: value.id,
			state: match value.state {
				GnuSplitState::Reconsiled => SplitState::Reconsiled,
				GnuSplitState::Cleared => SplitState::Cleared,
				GnuSplitState::Pending => SplitState::Pending,
			},
			account_id: value.account_id,
			value: value.value.0,
			reconcile_date: value.reconcile_date.map(|val| val.date),
		}
	}
}
