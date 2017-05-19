// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait HasContextPointer: HasVerbsPointer
{
	#[inline(always)]
	fn context(self) -> *mut c_void;
}

impl HasContextPointer for *mut ibv_cq
{
	#[inline(always)]
	fn context(self) -> *mut c_void
	{
		unsafe { (*self.pointer()).cq_context }
	}
}

impl HasContextPointer for *mut ibv_cq_ex
{
	#[inline(always)]
	fn context(self) -> *mut c_void
	{
		unsafe { (*self.pointer()).cq_context }
	}
}

impl HasContextPointer for *mut ibv_qp
{
	#[inline(always)]
	fn context(self) -> *mut c_void
	{
		unsafe { (*self).qp_context }
	}
}

impl HasContextPointer for *mut ibv_srq
{
	#[inline(always)]
	fn context(self) -> *mut c_void
	{
		unsafe { (*self).srq_context }
	}
}

impl HasContextPointer for *mut ibv_wq
{
	#[inline(always)]
	fn context(self) -> *mut c_void
	{
		unsafe { (*self).wq_context }
	}
}
