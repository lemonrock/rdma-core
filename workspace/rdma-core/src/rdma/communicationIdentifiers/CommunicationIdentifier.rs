// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


struct CommunicationIdentifier<Context>
{
	pointer: *mut rdma_cm_id,
	context: PhantomData<Rc<RefCell<Context>>>,
}

impl<Context> Drop for CommunicationIdentifier<Context>
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

impl<Context> CommunicationIdentifier<Context>
{
	#[inline(always)]
	pub(crate) fn newListeningAsynchronous<C: CommunicationEventHandler>(context: Rc<RefCell<Context>>, addressing: Addressing, backLog: u32, eventChannel: &EventChannel<C>) -> Self
	{
		Self::newListening(context, addressing, backLog, eventChannel.pointer)
	}
	
	#[inline(always)]
	pub(crate) fn newListeningSynchronous(context: Rc<RefCell<Context>>, addressing: Addressing, backLog: u32) -> Self
	{
		Self::newListening(context, addressing, backLog, null_mut())
	}
	
	#[inline(always)]
	fn newListening(context: Rc<RefCell<Context>>, addressing: Addressing, backLog: u32, eventChannel: *mut rdma_event_channel) -> Self
	{
		let (mut rdmaSocketAddress, portSpace) = addressing.createForLocal();
		
		let mut communicationIdentifierPointer = unsafe { uninitialized() };
		panic_on_error!(rdma_create_id, eventChannel, &mut communicationIdentifierPointer, Rc::into_raw(context) as *mut RefCell<Context> as *mut c_void, portSpace);
		
		panic_on_error!(rdma_bind_addr, communicationIdentifierPointer, rdmaSocketAddress.as_sockaddr_mut_ptr());
		
		panic_on_error!(rdma_listen, communicationIdentifierPointer, backLog as c_int);
		
		Self
		{
			pointer: communicationIdentifierPointer,
			context: PhantomData,
		}
	}
	
	#[inline(always)]
	pub(crate) fn context(&self) -> Rc<RefCell<Context>>
	{
		self.contextInternal().clone()
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	fn contextInternal(&self) -> Rc<RefCell<Context>>
	{
		unsafe { Rc::from_raw((*self.pointer).context as *const c_void as *const RefCell<Context>) }
	}
	
	// Only appropriate for synchronous communication identifiers
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
