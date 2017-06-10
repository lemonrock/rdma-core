// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


quick_error!
{
	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub enum ConfigurationModifyError
	{
		NoSuchNamedKey(keyName: String, configurationValue: CString)
		{
			display("Configuration key named '{}' does not exist (and so we can't set value '{:?}'", keyName, configurationValue)
		}
		
		InvalidParameter(keyName: String, configurationValue: CString)
		{
			display("Configuration value '{:?}' for key named '{}' is invalid", configurationValue, keyName)
		}
	}
}