// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryUnit
{
	UnitLess(usize),
	Bytes(usize),
	KiloBytes(usize),
	MegaBytes(usize),
	GigaBytes(usize),
	Infinity,
	Automatic,
}

impl ConfigurationValueConverter for MemoryUnit
{
	#[inline(always)]
	fn convert(&self) -> CString
	{
		use self::MemoryUnit::*;
		
		let value = match *self
		{
			UnitLess(value) => format!("{}", value),
			Bytes(value) => format!("{}b", value),
			KiloBytes(value) => format!("{}kb", value),
			MegaBytes(value) => format!("{}mb", value),
			GigaBytes(value) => format!("{}gb", value),
			Infinity => "inf".to_string(),
			Automatic => "auto".to_string(),
		};
		CString::new(value).unwrap()
	}
}
