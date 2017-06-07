// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AtomicOperationsSynchronizationMode
{
	/// CPU mode
	cpu,
	
	/// Device mode
	device,
	
	/// Prefer device mode if at least one active transport supports it, otherwise fallback to cpu mode
	/// This is the default
	guess,
}

impl ConfigurationValueConverter for AtomicOperationsSynchronizationMode
{
	#[inline(always)]
	fn convert(&self) -> CString
	{
		use self::AtomicOperationsSynchronizationMode::*;
		
		CString::new(match *self
		{
			cpu => "cpu",
			device => "device",
			guess => "guess",
		}).unwrap()
	}
}

impl AtomicOperationsSynchronizationMode
{
	#[inline(always)]
	pub fn fromString(value: &str) -> Option<AtomicOperationsSynchronizationMode>
	{
		use self::AtomicOperationsSynchronizationMode::*;
		
		if value.is_empty()
		{
			None
		}
		else
		{
			match value
			{
				"cpu" => Some(cpu),
				"device" => Some(device),
				"guess" => Some(guess),
				_ => None,
			}
		}
	}
}
