// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProtectionDomain<'a>
{
	pointer: *mut ibv_pd,
	lifetime: PhantomData<&'a Context>,
}

impl<'a> Drop for ProtectionDomain<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		panic_on_errno!(ibv_dealloc_pd, self.pointer);
	}
}

impl<'a> ProtectionDomain<'a>
{
	#[inline(always)]
	fn new(pointer: *mut ibv_pd) -> Self
	{
		debug_assert!(!pointer.is_null(), "pointer is null");
		
		Self
		{
			pointer: pointer,
			lifetime: PhantomData,
		}
	}
	
	#[inline(always)]
	pub fn createSharedRequestQueue(&'a self, requestedSettings: SharedRequestQueueSettings) -> SharedRequestQueue<'a>
	{
		let mut attributes = ibv_srq_init_attr
		{
			srq_context: null_mut(),
			attr: ibv_srq_attr
			{
				max_wr: requestedSettings.maximumNumberOfOutstandingWorkRequestsInInTheSharedRequestQueue,
				max_sge: requestedSettings.maximumNumberOfScatterElementsPerWorkRequest,
				srq_limit: 0,
			}
		};
		
		let pointer = panic_on_null!(ibv_create_srq, self.pointer, &mut attributes);
		SharedRequestQueue
		{
			pointer: pointer,
			settings: SharedRequestQueueSettings
			{
				maximumNumberOfOutstandingWorkRequestsInInTheSharedRequestQueue: attributes.attr.max_wr,
				maximumNumberOfScatterElementsPerWorkRequest: attributes.attr.max_sge,
			},
			limit: attributes.attr.srq_limit,
			lifetime: PhantomData,
		}
	}
	
//		pub fn ibv_create_ah(pd: *mut ibv_pd, attr: *mut ibv_ah_attr) -> *mut ibv_ah;
//
//		pub fn ibv_create_ah_from_wc(pd: *mut ibv_pd, wc: *mut ibv_wc, grh: *mut ibv_grh, port_num: u8) -> *mut ibv_ah;
//
//		pub fn ibv_reg_mr(pd: *mut ibv_pd, addr: *mut c_void, length: usize, access: c_int) -> *mut ibv_mr;
//		pub fn ibv_dereg_mr(mr: *mut ibv_mr) -> c_int;
//		pub fn ibv_rereg_mr(mr: *mut ibv_mr, flags: c_int, pd: *mut ibv_pd, addr: *mut c_void, length: usize, access: c_int) -> c_int
}
