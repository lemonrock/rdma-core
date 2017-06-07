// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait ConfigurationValueConverter
{
	#[inline(always)]
	fn convert(&self) -> CString;
}

impl ConfigurationValueConverter for bool
{
	#[inline(always)]
	fn convert(&self) -> CString
	{
		let string = if *self
		{
			"y"
		}
		else
		{
			"n"
		};
		CString::new(string.to_string()).unwrap()
	}
}

impl ConfigurationValueConverter for u32
{
	#[inline(always)]
	fn convert(&self) -> CString
	{
		CString::new(format!("{}", *self)).unwrap()
	}
}

impl ConfigurationValueConverter for f64
{
	#[inline(always)]
	fn convert(&self) -> CString
	{
		CString::new(format!("{}", *self)).unwrap()
	}
}

impl<T: ToString + Eq + Hash> ConfigurationValueConverter for HashSet<T>
{
	#[inline(always)]
	fn convert(&self) -> CString
	{
		let strings: Vec<String> = self.iter().map(|v| v.to_string()).collect();
		CString::new(strings.join(",")).unwrap()
	}
}

impl<T: ToString + Eq + Hash> ConfigurationValueConverter for OrderMap<T, ()>
{
	#[inline(always)]
	fn convert(&self) -> CString
	{
		let strings: Vec<String> = self.keys().map(|v| v.to_string()).collect();
		CString::new(strings.join(",")).unwrap()
	}
}
