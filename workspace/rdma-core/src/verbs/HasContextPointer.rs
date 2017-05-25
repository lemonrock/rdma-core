// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait HasContextPointer: Sized
{
	#[inline(always)]
	fn context(self) -> *mut c_void;
	
	#[inline(always)]
	fn setContext(self, context: *mut c_void);
	
	#[allow(trivial_casts)]
	#[inline(always)]
	fn setContextFromFatPointer<T: ?Sized>(self, context: Box<T>)
	{
		let contextThinPointer = Box::new(context);
		self.setContext(Box::into_raw(contextThinPointer) as *mut _ as *mut c_void)
	}
	
	#[inline(always)]
	fn getFatPointerContext<T: ?Sized>(self) -> Box<Box<T>>
	{
		let context = self.context();
		
		debug_assert!(!context.is_null(), "context is null");
		
		unsafe { Box::from_raw(context as *mut Box<T>) }
	}
}

impl HasContextPointer for *mut epoll_event
{
	#[inline(always)]
	fn context(self) -> *mut c_void
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).data.ptr }
	}
	
	#[inline(always)]
	fn setContext(self, context: *mut c_void)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).data.ptr = context };
	}
}

impl HasContextPointer for *mut ibv_cq
{
	#[inline(always)]
	fn context(self) -> *mut c_void
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self.pointer()).cq_context }
	}
	
	#[inline(always)]
	fn setContext(self, context: *mut c_void)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self.pointer()).cq_context = context };
	}
}

impl HasContextPointer for *mut ibv_cq_ex
{
	#[inline(always)]
	fn context(self) -> *mut c_void
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self.pointer()).cq_context }
	}
	
	#[inline(always)]
	fn setContext(self, context: *mut c_void)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self.pointer()).cq_context = context };
	}
}

impl HasContextPointer for *mut ibv_qp
{
	#[inline(always)]
	fn context(self) -> *mut c_void
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).qp_context }
	}
	
	#[inline(always)]
	fn setContext(self, context: *mut c_void)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).qp_context = context };
	}
}

impl HasContextPointer for *mut ibv_srq
{
	#[inline(always)]
	fn context(self) -> *mut c_void
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).srq_context }
	}
	
	#[inline(always)]
	fn setContext(self, context: *mut c_void)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).srq_context = context };
	}
}

impl HasContextPointer for *mut ibv_wq
{
	#[inline(always)]
	fn context(self) -> *mut c_void
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).wq_context }
	}
	
	#[inline(always)]
	fn setContext(self, context: *mut c_void)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).wq_context = context };
	}
}
