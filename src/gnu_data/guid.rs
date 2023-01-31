use std::fmt::{Debug, Display};

use druid::Data;
use serde::{Deserialize, Serialize};

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Data)]
pub struct GUID(pub [u8; 16]);

impl Display for GUID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
			self.0[0],
			self.0[1],
			self.0[2],
			self.0[3],
			self.0[4],
			self.0[5],
			self.0[6],
			self.0[7],
			self.0[8],
			self.0[9],
			self.0[10],
			self.0[11],
			self.0[12],
			self.0[13],
			self.0[14],
			self.0[15],
		)
	}
}

impl Debug for GUID {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
			self.0[0],
			self.0[1],
			self.0[2],
			self.0[3],
			self.0[4],
			self.0[5],
			self.0[6],
			self.0[7],
			self.0[8],
			self.0[9],
			self.0[10],
			self.0[11],
			self.0[12],
			self.0[13],
			self.0[14],
			self.0[15],
		)
	}
}
use std::fmt;

use serde::de::{self, Visitor};

struct GUIDVisitor;

impl<'de> Visitor<'de> for GUIDVisitor {
	type Value = GUID;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a 32 character hex string.")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: de::Error,
	{
		let mut val = GUID([0; 16]);
		for i in 0..16 {
			val.0[i] = match u8::from_str_radix(&v[i * 2..2 + i * 2], 16) {
				Ok(val) => val,
				Err(_) => {
					return Err(de::Error::invalid_value(
						de::Unexpected::Str(&v[i * 2..2 + i * 2]),
						&self,
					))
				}
			}
		}
		Ok(val)
	}
}

impl<'de> Deserialize<'de> for GUID {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserializer.deserialize_str(GUIDVisitor)
	}
}

impl Serialize for GUID {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.collect_str(&format!("{self}"))
	}
}
