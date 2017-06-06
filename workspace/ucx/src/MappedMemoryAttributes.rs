// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone)]
pub struct MappedMemoryAttributes(ucp_mem_attr_t);

impl MappedMemoryAttributes
{
	#[inline(always)]
	pub fn address(&self) -> *mut c_void
	{
		self.0.address
	}
	
	#[inline(always)]
	pub fn length(&self) -> usize
	{
		self.0.length
	}
}

impl PartialEq for MappedMemoryAttributes
{
	#[inline(always)]
	fn eq(&self, other: &MappedMemoryAttributes) -> bool
	{
		self.address() == other.address() && self.length() == other.length()
	}
}

impl Eq for MappedMemoryAttributes
{
}

impl PartialOrd for MappedMemoryAttributes
{
	#[inline(always)]
	fn partial_cmp(&self, other: &MappedMemoryAttributes) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Ord for MappedMemoryAttributes
{
	#[inline(always)]
	fn cmp(&self, other: &MappedMemoryAttributes) -> Ordering
	{
		self.address().cmp(&other.address()).then(self.length().cmp(&other.length()))
	}
}

impl Hash for MappedMemoryAttributes
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.address().hash(state);
		self.length().hash(state);
	}
}
