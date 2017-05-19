// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A context can also be known as 'verbs' and as a 'device' (in RDMA-speak, not ibverbs-speak)
pub struct ContextMap
{
	map: HashMap<*mut ibv_context, ()>
}

impl ContextMap
{
	#[inline(always)]
	fn new() -> Self
	{
		let mut numberOfContexts = unsafe { uninitialized() };
		let listOfContexts = panic_on_null!(rdma_get_devices, &mut numberOfDevices);
		
		debug_assert!(numberOfContexts >= 0, "numberOfContexts '{}' was negative", numberOfContexts);
		
		let mut result = Self
		{
			map: HashMap::with_capacity(numberOfContexts as usize)
		};
		
		unsafe { rdma_free_devices(listOfContexts) };
		
		let mut context = unsafe { *listOfContexts };
		let mut counter = 0;
		while likely(!context.is_null())
		{
			result.map.insert(context, ());
			
			context = unsafe { context.offset(1) };
			counter = counter + 1;
		}
		debug_assert!(counter == numberOfContexts, "NULL-terminated list had '{}' elements but was supposed to have numberOfContexts '{}' elements", counter, numberOfContexts);
		
		result
	}
}
