// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct EndPoint<'a, 'b, ErrorHandler: EndPointErrorHandler>
where 'a: 'b
{
	handle: ucp_ep_h,
	worker: &'b Worker<'a>,
	parameters: ucp_ep_params_t,
	errorHandler: ErrorHandler,
}

impl<'a, 'b, ErrorHandler: EndPointErrorHandler> Drop for EndPoint<'a, 'b, ErrorHandler>
where 'a: 'b
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if unlikely(!self.handle.is_null())
		{
			// We do this as a callback can still be pending
			let handle = self.handle;
			self.handle = null_mut();
			NonBlockingRequest(unsafe { ucp_disconnect_nb(handle) });
		}
	}
}

impl<'a, 'b, ErrorHandler: EndPointErrorHandler> PrintInformation for EndPoint<'a, 'b, ErrorHandler>
{
	#[inline(always)]
	fn printInformationToStream(&self, stream: *mut FILE)
	{
		unsafe { ucp_ep_print_info(self.handle, stream) };
	}
}

impl<'a, 'b, ErrorHandler: EndPointErrorHandler> EndPoint<'a, 'b, ErrorHandler>
where 'a: 'b
{
	#[inline(always)]
	pub fn flushAllOutstandingRemoteMemoryAccessAndAtomicOperations(&self)
	{
		panic_on_error!(ucp_ep_flush, self.handle);
	}
	
	/// remoteMemoryAccessKeyBuffer should have been created by packing on a MappedMemory object on the remote side
	/// We are the receiver
	#[inline(always)]
	pub fn unpackRemoteMemoryAccessKeyBuffer<'c>(&'c self, remoteMemoryAccessKeyBuffer: *mut c_void) -> RemoteMemoryAccessKey<'a, 'b, 'c, ErrorHandler>
	{
		let mut handle = unsafe { uninitialized() };
		panic_on_error!(ucp_ep_rkey_unpack, self.handle, remoteMemoryAccessKeyBuffer, &mut handle);
		RemoteMemoryAccessKey
		{
			handle: handle,
			endPoint: self,
		}
	}
	
	// TODO: arg HAS TO BE A WEAK REFERENCE
	
	unsafe extern "C" fn errorHandlerCallback(arg: *mut c_void, ep: ucp_ep_h, status: ucs_status_t)
	{
		debug_assert!(!ep.is_null(), "ep is null");
		debug_assert!(status != ucs_status_t_UCS_OK, "status is UCS_OK!");
		
		let mut endPoint = &mut *(arg as *mut Self);
		endPoint.errorHandler(status);
	}
	
	#[inline(always)]
	fn errorHandler(&mut self, status: ucs_status_t)
	{
		// Can occur when
		// - between initial creation and assignment of handle (XXX)
		// - after ucp_disconnect_nb() called? In which case, ucp_disconnect_nb() should not be called in drop()
		if self.handle.is_null()
		{
			return;
		}
		
		// It is believed that at this time the handle is invalid and has already become disconnected
		// It is possible that this function is invoked immediately after EndPoint has been created and before self.handle has been set
		// This is seen as unlikely: see (XXX)
		self.handle = null_mut();
		
		let destinationAddress = self.errorHandler.shouldWeReconnect(status);
		if unlikely(destinationAddress.is_null())
		{
			return;
		}
		
		self.connectOrReconnect(destinationAddress)
	}
	
	#[inline(always)]
	fn connectOrReconnect(&mut self, destinationAddress: *const ucp_address_t)
	{
		// TODO: Does reconnecting invalidate any rkey s created from this endpoint?
		
		debug_assert!(self.handle.is_null(), "handle is not null");
		
		self.parameters.address = destinationAddress;
		let mut handle = unsafe { uninitialized() };
		panic_on_error!(ucp_ep_create, self.worker.handle, &self.parameters, &mut handle);
		// (XXX) At this juncture here, self.handle is null but self.errorHandler() could be called back by ucp_ep_create()
		self.handle = handle;
	}
}
