// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait HasContextPointer: Sized
{
	#[doc(hidden)]
	#[inline(always)]
	fn getContext(self) -> *mut c_void;
	
	#[doc(hidden)]
	#[inline(always)]
	fn setContext(self, context: *mut c_void);
	
	#[inline(always)]
	fn getBoxPointerContext<T>(self) -> Box<T>
	{
		let context = self.getContext();
		
		debug_assert!(!context.is_null(), "context is null");
		
		unsafe { Box::from_raw(context as *mut T) }
	}
	
	#[inline(always)]
	fn setBoxPointerContext<T>(self, context: Box<T>)
	{
		self.setContext(Box::into_raw(context) as *mut T as *mut c_void)
	}
	
	#[inline(always)]
	fn getPossiblyNullBoxPointerContext<T>(self) -> Option<Box<T>>
	{
		let context = self.getContext();
		
		if unlikely(context.is_null())
		{
			None
		}
		else
		{
			Some(unsafe { Box::from_raw(context as *mut T) })
		}
	}
	
	#[inline(always)]
	fn destroyContext<T>(self)
	{
		drop(self.getPossiblyNullBoxPointerContext::<T>());
	}
	
	#[inline(always)]
	fn setContextFromFatPointer<T: ?Sized>(self, context: Box<T>)
	{
		let contextThinPointer = Box::new(context);
		self.setBoxPointerContext(contextThinPointer)
	}
	
	#[inline(always)]
	fn getFatPointerContext<T: ?Sized>(self) -> Box<Box<T>>
	{
		self.getBoxPointerContext()
	}
}

impl HasContextPointer for *mut epoll_event
{
	#[doc(hidden)]
	#[inline(always)]
	fn getContext(self) -> *mut c_void
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).data.ptr }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn setContext(self, context: *mut c_void)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).data.ptr = context };
	}
}

impl HasContextPointer for *mut ibv_cq
{
	#[doc(hidden)]
	#[inline(always)]
	fn getContext(self) -> *mut c_void
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self.pointer()).cq_context }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn setContext(self, context: *mut c_void)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self.pointer()).cq_context = context };
	}
}

impl HasContextPointer for *mut ibv_cq_ex
{
	#[doc(hidden)]
	#[inline(always)]
	fn getContext(self) -> *mut c_void
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self.pointer()).cq_context }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn setContext(self, context: *mut c_void)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self.pointer()).cq_context = context };
	}
}

impl HasContextPointer for *mut ibv_qp
{
	#[doc(hidden)]
	#[inline(always)]
	fn getContext(self) -> *mut c_void
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).qp_context }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn setContext(self, context: *mut c_void)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).qp_context = context };
	}
}

impl HasContextPointer for *mut ibv_srq
{
	#[doc(hidden)]
	#[inline(always)]
	fn getContext(self) -> *mut c_void
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).srq_context }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn setContext(self, context: *mut c_void)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).srq_context = context };
	}
}

impl HasContextPointer for *mut ibv_wq
{
	#[doc(hidden)]
	#[inline(always)]
	fn getContext(self) -> *mut c_void
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).wq_context }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn setContext(self, context: *mut c_void)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).wq_context = context };
	}
}
