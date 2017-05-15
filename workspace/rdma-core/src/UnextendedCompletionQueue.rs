// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub struct UnextendedCompletionQueue<'a>
{
	pointer: *mut ibv_cq,
	lifetime: Option<&'a CompletionChannel<'a>>,
}

impl<'a> Drop for UnextendedCompletionQueue<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		panic_on_errno!(ibv_destroy_cq, self.pointer);
	}
}

impl<'a> CompletionQueue<'a> for UnextendedCompletionQueue<'a>
{
	#[doc(hidden)]
	#[inline(always)]
	fn pointer(&self) -> *mut ibv_cq
	{
		self.pointer
	}
}

pub const UnextendedCompletionQueuePollArraySize: usize = 32;

impl<'a> UnextendedCompletionQueue<'a>
{
	#[inline(always)]
	fn new(pointer: *mut ibv_cq, lifetime: Option<&'a CompletionChannel>) -> Self
	{
		debug_assert!(!pointer.is_null(), "pointer is null");
		
		Self
		{
			pointer: pointer,
			lifetime: lifetime,
		}
	}
	
	/// Returns number of additional work completions added; it is recommended that `into` is empty
	#[inline(always)]
	pub fn poll(&self, into: &mut ArrayVec<[WorkCompletion; UnextendedCompletionQueuePollArraySize]>) -> usize
	{
		let length = into.len();
		
		let space = UnextendedCompletionQueuePollArraySize - length;
		
		if unlikely(space == 0)
		{
			return 0;
		}
		
		let fillFrom = unsafe { transmute(into.as_mut_ptr().offset(length as isize)) };
		
		let result = unsafe { rust_ibv_poll_cq(self.pointer, space as i32, fillFrom) };
		if likely(result >= 0)
		{
			debug_assert!(result as usize <= space, "Overfilled; defect in rust_ibv_poll_cq()");
			let increasedBy = result as usize;
			unsafe { into.set_len(length + increasedBy) };
			increasedBy
		}
		else
		{
			let errno = errno();
			panic!("rust_ibv_poll_cq failed with result '{}' error number '{}' ('{}')", result, errno.0, errno);
		}
	}
}
