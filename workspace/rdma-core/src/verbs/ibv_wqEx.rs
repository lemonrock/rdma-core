// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait ibv_wqEx
{
	#[inline(always)]
	fn ibv_post_wq_recv(self, recv_wr: *mut ibv_recv_wr, bad_recv_wr: *mut *mut ibv_recv_wr) -> c_int;
}

impl ibv_wqEx for *mut ibv_wq
{
	#[inline(always)]
	fn ibv_post_wq_recv(self, recv_wr: *mut ibv_recv_wr, bad_recv_wr: *mut *mut ibv_recv_wr) -> c_int
	{
		debug_assert!(!self.is_null(), "self is null");
		debug_assert!(!recv_wr.is_null(), "recv_wr is null");
		debug_assert!(!bad_recv_wr.is_null(), "bad_recv_wr is null");
		
		unsafe { (*self).post_recv.unwrap()(self, recv_wr, bad_recv_wr) }
	}
}
