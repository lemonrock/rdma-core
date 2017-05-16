// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub trait ExtendedCompletionQueue<'a>: CompletionQueue + Sized
{
	#[doc(hidden)]
	#[inline(always)]
	fn extendedPointer(&self) -> *mut ibv_cq_ex;
	
	#[doc(hidden)]
	#[inline(always)]
	fn isCurrentlyBeingPolled(&self)-> bool;
	
	#[doc(hidden)]
	#[inline(always)]
	fn isNoLongerBeingPolled(&mut self);
	
	/// NOTE WELL: Once poll() is called, the previous item will be invalid
	#[inline(always)]
	fn poll(&'a mut self) -> Option<ExtendedWorkCompletion<'a, Self>>
	{
		if likely(self.isCurrentlyBeingPolled())
		{
			let result = unsafe { rust_ibv_next_poll(self.extendedPointer()) };
			debug_assert!(result >= 0, "result was negative '{}'", result);
			if likely(result == 0)
			{
				Some(ExtendedWorkCompletion(self))
			}
			else
			{
				self.endPolling();
				self.isNoLongerBeingPolled();
				
				if likely(result == E::ENOENT)
				{
					None
				}
				else
				{
					panic!("rust_ibv_next_poll() returned an error number '{}'", result);
				}
			}
		}
		else
		{
			let mut attributes = ibv_poll_cq_attr
			{
				comp_mask: 0
			};
			
			let result = unsafe { rust_ibv_start_poll(self.extendedPointer(), &mut attributes) };
			debug_assert!(result >= 0, "result was negative '{}'", result);
			if likely(result == 0)
			{
				self.isNoLongerBeingPolled();
				Some(ExtendedWorkCompletion(self))
			}
			else
			{
				// NOTE: We MUST NOT call self.endPolling() here
				
				if likely(result == E::ENOENT)
				{
					None
				}
				else
				{
					panic!("rust_ibv_start_poll() returned an error number '{}'", result);
				}
			}
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn endPolling(&mut self)
	{
		unsafe { rust_ibv_end_poll(self.extendedPointer()) }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn destroy(&mut self)
	{
		if self.isCurrentlyBeingPolled()
		{
			self.endPolling();
		}
		
		let pointer = self.pointer();
		panic_on_errno!(ibv_destroy_cq, pointer);
	}
}

