// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct MemoryUnitsConfigurationSetting
{
	key: MemoryUnitsConfigurationKey,
	value: Option<String>,
}

impl ConfigurationSetting for MemoryUnitsConfigurationSetting
{
	#[inline(always)]
	fn set(&self, configuration: &Configuration)
	{
		configuration.modify(self.key.key(), self.value())
	}
}

impl MemoryUnitsConfigurationSetting
{
	#[inline(always)]
	pub fn new(key: MemoryUnitsConfigurationKey, value: Option<String>) -> Self
	{
		Self
		{
			key: key,
			value: value,
		}
	}
	
	#[inline(always)]
	fn value(&self) -> CString
	{
		match self.value
		{
			None => self.key.defaultValue(),
			Some(ref value) => CString::new(value.to_string()).expect("Values contains an embedded NUL")
		}
	}
}
