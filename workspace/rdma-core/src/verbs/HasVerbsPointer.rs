// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait HasVerbsPointer: Sized + Copy
{
	#[inline(always)]
	fn verbs(self) -> *mut ibv_context;
}

impl<T: HasProtectionDomainPointer> HasVerbsPointer for T
{
	#[inline(always)]
	default fn verbs(self) -> *mut ibv_context
	{
		self.protectionDomain().verbs()
	}
}

impl HasVerbsPointer for *mut ibv_ah
{
	#[inline(always)]
	fn verbs(self) -> *mut ibv_context
	{
		debug_assert!(self.protectionDomain().verbs() == unsafe { (*self).context }, "ibv_ah.context is not the same as ibv_ah.pd.context");
		
		unsafe { (*self).context }
	}
}

impl HasVerbsPointer for *mut ibv_comp_channel
{
	#[inline(always)]
	fn verbs(self) -> *mut ibv_context
	{
		unsafe { (*self).context }
	}
}

impl HasVerbsPointer for *mut ibv_cq
{
	#[inline(always)]
	fn verbs(self) -> *mut ibv_context
	{
		unsafe { (*self.pointer()).context }
	}
}

impl HasVerbsPointer for *mut ibv_cq_ex
{
	#[inline(always)]
	fn verbs(self) -> *mut ibv_context
	{
		unsafe { (*self.pointer()).context }
	}
}

impl HasVerbsPointer for *mut ibv_pd
{
	#[inline(always)]
	fn verbs(self) -> *mut ibv_context
	{
		unsafe { (*self).context }
	}
}

impl HasVerbsPointer for *mut ibv_qp
{
	#[inline(always)]
	fn verbs(self) -> *mut ibv_context
	{
		unsafe { (*self).context }
	}
}

impl HasVerbsPointer for *mut ibv_srq
{
	#[inline(always)]
	fn verbs(self) -> *mut ibv_context
	{
		unsafe { (*self).context }
	}
}

impl HasVerbsPointer for *mut ibv_wq
{
	#[inline(always)]
	fn verbs(self) -> *mut ibv_context
	{
		unsafe { (*self).context }
	}
}
