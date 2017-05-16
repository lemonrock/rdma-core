// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// 'Owned' by an event channel
// Can 'come into being'


pub struct AsynchronousCommunicationIdentifier<Context>
{
	pointer: *mut rdma_cm_id,
	phantomData: PhantomData<Rc<RefCell<Context>>>
}

impl<Context> Drop for AsynchronousCommunicationIdentifier<Context>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		drop(self.contextInternal());
		
		// TODO: all associated queue pairs must have been free'd
		// TODO: all related events must have been ack'd
		
		panic_on_error!(rdma_destroy_id, self.pointer);
	}
}

impl<Context> AsynchronousCommunicationIdentifier<Context>
{
	#[inline(always)]
	pub(crate) fn new(pointer: *mut rdma_cm_id) -> Self
	{
		Self
		{
			pointer: pointer,
			phantomData: PhantomData,
		}
	}
	
	#[inline(always)]
	pub fn context(&self) -> Rc<RefCell<Context>>
	{
		self.contextInternal().clone()
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	fn contextInternal(&self) -> Rc<RefCell<Context>>
	{
		unsafe { Rc::from_raw((*self.pointer).context as *const c_void as *const RefCell<Context>) }
	}
}
