// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct Worker<'a>
{
	handle: ucp_worker_h,
	applicationContext: &'a ApplicationContext,
}

impl<'a> Drop for Worker<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_worker_destroy(self.handle) }
	}
}

impl<'a> PrintInformation for Worker<'a>
{
	#[inline(always)]
	fn printInformationToStream(&self, stream: *mut FILE)
	{
		unsafe { ucp_worker_print_info(self.handle, stream) };
	}
}

impl<'a> QueryAttributes for Worker<'a>
{
	type Attributes = WorkerAttributes;
	
	#[inline(always)]
	fn queryAttributes(&self) -> Self::Attributes
	{
		use ucp_worker_attr_field::*;
		
		let mut attributes: ucp_worker_attr_t = unsafe { uninitialized() };
		attributes.field_mask = UCP_WORKER_ATTR_FIELD_THREAD_MODE as u64;
		panic_on_error!(ucp_worker_query, self.handle, &mut attributes);
		WorkerAttributes(attributes)
	}
}

impl<'a> Worker<'a>
{
	#[inline(always)]
	pub fn addressHandle<'b>(&'b self) -> WorkerAddressHandle<'a, 'b>
	where 'a: 'b
	{
		let mut addressHandle = unsafe { uninitialized() };
		let mut addressHandleLength = unsafe { uninitialized() };
		panic_on_error!(ucp_worker_get_address, self.handle, &mut addressHandle, &mut addressHandleLength);
		WorkerAddressHandle
		{
			handle: addressHandle,
			length: addressHandleLength,
			worker: self,
		}
	}
	
	// ucp_worker_get_address() is used to get destinationAddress
	#[inline(always)]
	pub fn createEndPoint<'b, ErrorHandler: EndPointErrorHandler>(&'b self, errorHandler: ErrorHandler, destinationAddress: *const ucp_address_t) -> Box<EndPoint<'a, 'b, ErrorHandler>>
	{
		assert!(!destinationAddress.is_null(), "destinationAddress is null");
		
		use ucp_ep_params_field::*;
		use ucp_err_handling_mode_t::*;
		
		let mut endPoint = Box::new(EndPoint
		{
			handle: null_mut(),
			worker: self,
			parameters: ucp_ep_params_t
			{
				field_mask: UCP_EP_PARAM_FIELD_REMOTE_ADDRESS as u64 | UCP_EP_PARAM_FIELD_ERR_HANDLING_MODE as u64 | UCP_EP_PARAM_FIELD_ERR_HANDLER as u64,
				address: null(),
				err_mode: UCP_ERR_HANDLING_MODE_NONE,
				err_handler: ucp_err_handler_t
				{
					cb: Some(EndPoint::<ErrorHandler>::errorHandlerCallback),
					arg: null_mut(),
				},
			},
			errorHandler: errorHandler,
		});
		
		#[allow(trivial_casts)]
		{
			endPoint.parameters.err_handler.arg = endPoint.as_mut() as *mut _ as *mut c_void;
		}
		
		endPoint.connectOrReconnect(destinationAddress);
		
		endPoint
	}
	
	#[inline(always)]
	pub fn flushAllOutstandingRemoteMemoryAccessAndAtomicOperations(&self)
	{
		panic_on_error!(ucp_worker_flush, self.handle);
	}
	
	#[inline(always)]
	pub fn getFileDescriptorSuitableForEPoll(&self) -> RawFd
	{
		let mut fileDescriptorSuitableForEPoll = unsafe { uninitialized() };
		panic_on_error!(ucp_worker_get_efd, self.handle, &mut fileDescriptorSuitableForEPoll);
		fileDescriptorSuitableForEPoll
	}
	
	/// Returns an Err if internal logical returns UCS_ERR_IO_ERROR
	#[inline(always)]
	pub fn blockingWaitForAnyEvent(&self) -> Result<(), ()>
	{
		panic_on_error_with_clean_up!
		(
			status,
			{
				match status
				{
					ucs_status_t_UCS_ERR_IO_ERROR => return Err(()),
					ucs_status_t_UCS_ERR_NO_MEMORY => panic!("Out of memory in 'ucp_worker_wait'"),
					_ => (),
				};
			},
			ucp_worker_wait,
			self.handle
		);
		Ok(())
	}
	
	/// Returns an Err if internal logical returns UCS_ERR_IO_ERROR
	#[inline(always)]
	pub fn blockingWaitForAMemoryEvent(&self, address: *mut c_void)
	{
		unsafe { ucp_worker_wait_mem(self.handle, address) }
	}
	
	/// Returns 'true' if one should call ucp_worker_progress(), ie the worker can not arm because it is 'busy'
	///
	#[inline(always)]
	pub fn arm(&self) -> bool
	{
		panic_on_error_with_clean_up!
		(
			status,
			{
				match status
				{
					ucs_status_t_UCS_ERR_BUSY  => return true,
					_ => (),
				};
			},
			ucp_worker_arm,
			self.handle
		);
		false
	}
	
	/// Wakes up a worker waiting in blockingWaitForAnyEvent(), blockingWaitForAnyEvent() or on epoll
	#[inline(always)]
	pub fn signal(&self)
	{
		panic_on_error!(ucp_worker_signal, self.handle);
	}
}
