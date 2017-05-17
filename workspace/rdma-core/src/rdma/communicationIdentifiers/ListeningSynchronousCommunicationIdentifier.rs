// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct ListeningSynchronousCommunicationIdentifier<Context>
{
	pointer: *mut rdma_cm_id,
	context: PhantomData<Rc<RefCell<Context>>>,
}

impl<Context> Drop for ListeningSynchronousCommunicationIdentifier<Context>
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

impl<Context> ListeningSynchronousCommunicationIdentifier<Context>
{
	#[inline(always)]
	pub fn new(addressing: Addressing, context: Rc<RefCell<Context>>) -> Self
	{
		let (rdmaSocketAddress, portSpace) = addressing.createForLocal();
		
		let mut communicationIdentifierPointer = unsafe { uninitialized() };
		panic_on_error!(rdma_create_id, null_mut(), &mut communicationIdentifierPointer, Rc::into_raw(context) as *mut RefCell<Context> as *mut c_void, portSpace);
		
//
//		TEST_NZ(rdma_bind_addr(listener, (struct sockaddr *)&addr));
//		TEST_NZ(rdma_listen(listener, 10)); /* backlog=10 is arbitrary */
//
//		memset(&addr, 0, sizeof(addr));
//		#if _USE_IPV6
//		addr.sin6_family = AF_INET6;
//		#else
//		addr.sin_family = AF_INET;
//		#endif
//
//
//		TEST_NZ(rdma_bind_addr(listener, (struct sockaddr *)&addr));
//		TEST_NZ(rdma_listen(listener, 10)); /* backlog=10 is arbitrary */
//
//		port = ntohs(rdma_get_src_port(listener));
//
		Self
		{
			pointer: communicationIdentifierPointer,
			context: PhantomData,
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

	// Can not move out deref of raw pointer: let eventPointer = unsafe { *self.pointer }.event;
//	#[inline(always)]
//	fn currentEvent(&self) -> Option<&rdma_cm_event>
//	{
//		let eventPointer = unsafe { *self.pointer }.event;
//		if unlikely(eventPointer.is_null())
//		{
//			None
//		}
//		else
//		{
//			Some(unsafe { &*eventPointer })
//		}
//	}
}