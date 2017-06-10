// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct ConfigurationSetting<'a, T: ConfigurationValueConverter>
where T: 'a
{
	key: &'a ConfigurationKey<T>,
	value: T,
}

impl<'a, T: ConfigurationValueConverter> ConfigurationSetting<'a, T>
where T: 'a
{
	#[inline(always)]
	pub fn set(&self, configuration: &Configuration)
	{
		configuration.modify(self.key.name(), &ConfigurationValueConverter::convert(&self.value)).expect("We've got something very wrong");
	}
}