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
			NonBlockingRequest(unsafe { ucp_disconnect_nb(self.handle) });
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
	unsafe extern "C" fn errorHandlerCallback(arg: *mut c_void, ep: ucp_ep_h, status: ucs_status_t)
	{
		debug_assert!(!ep.is_null(), "ep is null");
		debug_assert!(status != ucs_status_t::UCS_OK, "status is UCS_OK!");
		
		let mut endPoint = &mut *(arg as *mut Self);
		endPoint.errorHandler(status);
	}
	
	#[inline(always)]
	fn errorHandler(&mut self, status: ucs_status_t)
	{
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
		debug_assert!(self.handle.is_null(), "handle is not null");
		
		self.parameters.address = destinationAddress;
		let mut handle = unsafe { uninitialized() };
		panic_on_error!(ucp_ep_create, self.worker.handle, &self.parameters, &mut handle);
		// (XXX) At this juncture here, self.handle is null but self.errorHandler() could be called back by ucp_ep_create()
		self.handle = handle;
	}
	
	#[inline(always)]
	pub fn flushAllOutstandingRemoteMemoryAccessAndAtomicOperations(&self)
	{
		panic_on_error!(ucp_ep_flush, self.handle);
	}
}
