// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait ibv_contextEx
{
	#[inline(always)]
	fn destroy(self);
	
	#[inline(always)]
	fn extendedAttributes(self) -> ibv_device_attr_ex;
	
	#[inline(always)]
	fn queryRealTimeRawClock(self) -> timespec;
	
	#[doc(hidden)]
	#[inline(always)]
	fn data(self) -> ibv_context;
	
	#[inline(always)]
	fn numberOfCompletionVectors(self) -> u32;
	
	#[inline(always)]
	fn commandFileDescriptor(self) -> FileDescriptor;
	
	#[inline(always)]
	fn asyncFileDescriptor(self) -> FileDescriptor;
	
	#[inline(always)]
	fn blockOnAsynchronousEvent(self) -> ibv_async_event;
	
	#[inline(always)]
	fn allocateProtectionDomain(self) -> *mut ibv_pd;
	
	#[inline(always)]
	fn createCompletionChannel(self) -> *mut ibv_comp_channel;
}

impl ibv_contextEx for *mut ibv_context
{
	#[inline(always)]
	fn destroy(self)
	{
		let result = unsafe { ibv_close_device(self) };
		debug_assert!(result == 0 || result == 1, "ibv_close_device returned a result '{}' which was not 0 or 1", result);
		if unlikely(result == -1)
		{
			// Examination of C source code suggests it can't actually fail
			panic!("ibv_close_device failed (no further details)");
		}
	}
	
	#[inline(always)]
	fn extendedAttributes(self) -> ibv_device_attr_ex
	{
		let mut attributes = unsafe { zeroed() };
		panic_on_error!(rust_ibv_query_device_ex, self, null_mut(), &mut attributes);
		attributes
	}
	
	#[inline(always)]
	fn queryRealTimeRawClock(self) -> timespec
	{
		const IBV_VALUES_MASK_RAW_CLOCK: u32 = 1;
		
		let mut realTimeValues = ibv_values_ex
		{
			comp_mask: IBV_VALUES_MASK_RAW_CLOCK,
			raw_clock: unsafe { zeroed() },
		};
		
		panic_on_errno!(rust_ibv_query_rt_values_ex, self, &mut realTimeValues);
		realTimeValues.raw_clock
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn data(self) -> ibv_context
	{
		unsafe { *self }
	}
	
	#[inline(always)]
	fn numberOfCompletionVectors(self) -> u32
	{
		let num_comp_vectors = self.data().num_comp_vectors;
		if unlikely(num_comp_vectors < 0)
		{
			0
		}
		else
		{
			num_comp_vectors as u32
		}
	}
	
	#[inline(always)]
	fn commandFileDescriptor(self) -> FileDescriptor
	{
		self.data().cmd_fd
	}
	
	#[inline(always)]
	fn asyncFileDescriptor(self) -> FileDescriptor
	{
		self.data().async_fd
	}
	
	#[inline(always)]
	fn blockOnAsynchronousEvent(self) -> ibv_async_event
	{
		let mut asynchronousEvent = unsafe { uninitialized() };
		panic_on_error!(ibv_get_async_event, self, &mut asynchronousEvent);
		asynchronousEvent
	}
	
	#[inline(always)]
	fn allocateProtectionDomain(self) -> *mut ibv_pd
	{
		panic_on_null!(ibv_alloc_pd, self)
	}
	
	#[inline(always)]
	fn createCompletionChannel(self) -> *mut ibv_comp_channel
	{
		panic_on_null!(ibv_create_comp_channel, self)
	}
}
