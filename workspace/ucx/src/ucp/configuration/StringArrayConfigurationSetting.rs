// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct StringArrayConfigurationSetting
{
	key: StringArrayConfigurationKey,
	values: Option<Vec<String>>,
	uniqueValues: HashSet<String>
}

impl ConfigurationSetting for StringArrayConfigurationSetting
{
	#[inline(always)]
	fn set(&self, configuration: &Configuration)
	{
		configuration.modify(self.key.key(), self.values())
	}
}

impl StringArrayConfigurationSetting
{
	const InitialCapacity: usize = 16;
	
	#[inline(always)]
	pub fn new(key: StringArrayConfigurationKey) -> Self
	{
		Self
		{
			key: key,
			values: None,
			uniqueValues: HashSet::with_capacity(Self::InitialCapacity)
		}
	}
	
	// An alternative to the wasteful memory usage is probably to co-opt something like bluss' OrderMap, but that doesn't come with an useful .join() method
	#[inline(always)]
	pub fn addValue(&mut self, value: String)
	{
		assert!(self.key.validateValue(&value), "value {} is not valid", value);
		assert!(!self.uniqueValues.contains(&value), "value {} has already been added", value);
		
		if self.values.is_none()
		{
			self.values = Some(Vec::with_capacity(Self::InitialCapacity))
		}
		if let Some(ref mut values) = self.values
		{
			values.insert(10, value.clone());
		}
		self.uniqueValues.insert(value);
	}
	
	#[inline(always)]
	fn values(&self) -> CString
	{
		match self.values
		{
			None => self.key.defaultValue(),
			Some(ref values) => CString::new(values.join(",")).expect("Values contains an embedded NUL")
		}
	}
}
