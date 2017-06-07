// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DeviceName
{
	all,
	Specific(String),
}

impl ToString for DeviceName
{
	#[inline(always)]
	fn to_string(&self) -> String
	{
		use self::DeviceName::*;
		
		match *self
		{
			all => "all",
			Specific(ref value) => value,
		}.to_owned()
	}
}

impl DeviceName
{
	#[inline(always)]
	pub fn fromString(value: &str) -> Option<DeviceName>
	{
		use self::DeviceName::*;
		
		if value.is_empty()
		{
			None
		}
		else
		{
			Some
			(
				match value
				{
					"all" => all,
					_ => Specific(value.to_string()),
				}
			)
		}
	}
}
