// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub struct ExtendedCompletionQueue<'a>
{
	pub(crate) pointer: *mut ibv_cq_ex,
	lifetime: Option<&'a CompletionChannel<'a>>,
	isCurrentlyBeingPolled: bool,
}

impl<'a> Drop for ExtendedCompletionQueue<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.isCurrentlyBeingPolled
		{
			self.endPolling();
		}
		
		let pointer = self.pointer();
		panic_on_errno!(ibv_destroy_cq, pointer);
	}
}

impl<'a> CompletionQueue<'a> for ExtendedCompletionQueue<'a>
{
	#[doc(hidden)]
	#[inline(always)]
	fn pointer(&self) -> *mut ibv_cq
	{
		unsafe { rust_ibv_cq_ex_to_cq(self.pointer) }
	}
}

impl<'a> ExtendedCompletionQueue<'a>
{
	#[inline(always)]
	pub(crate) fn new(pointer: *mut ibv_cq_ex, lifetime: Option<&'a CompletionChannel>) -> Self
	{
		debug_assert!(!pointer.is_null(), "pointer is null");
		
		Self
		{
			pointer: pointer,
			lifetime: lifetime,
			isCurrentlyBeingPolled: false,
		}
	}
	
	/// NOTE WELL: Once poll() is called, the previous item will be invalid
	#[inline(always)]
	pub fn poll(&'a mut self) -> Option<ExtendedWorkCompletion<'a>>
	{
		if likely(self.isCurrentlyBeingPolled)
		{
			let result = unsafe { rust_ibv_next_poll(self.pointer) };
			debug_assert!(result >= 0, "result was negative '{}'", result);
			if likely(result == 0)
			{
				Some(ExtendedWorkCompletion(self))
			}
			else
			{
				self.endPolling();
				self.isCurrentlyBeingPolled = false;
				
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
			
			let result = unsafe { rust_ibv_start_poll(self.pointer, &mut attributes) };
			debug_assert!(result >= 0, "result was negative '{}'", result);
			if likely(result == 0)
			{
				self.isCurrentlyBeingPolled = true;
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
	
	#[inline(always)]
	fn endPolling(&mut self)
	{
		unsafe { rust_ibv_end_poll(self.pointer) }
	}
}
